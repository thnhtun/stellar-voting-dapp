#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Bytes, BytesN, Env,
};

#[derive(Clone)]
#[contracttype]
pub struct CommitmentDeposit {
    pub user: Address,
    pub amount: i128,
    pub commitment: BytesN<32>,
    pub created_at: u64,
    pub revealed: bool,
    pub refunded: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Counter,
    Deposit(u64),
}

#[contract]
pub struct CommitDepositContract;

#[contractimpl]
impl CommitDepositContract {
    // Tạo deposit (không dùng token, chỉ lưu logic)
    pub fn create_deposit(
        env: Env,
        user: Address,
        amount: i128,
        commitment: BytesN<32>,
    ) -> u64 {
        user.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let mut counter: u64 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        counter += 1;

        let deposit = CommitmentDeposit {
            user,
            amount,
            commitment,
            created_at: env.ledger().timestamp(),
            revealed: false,
            refunded: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Deposit(counter), &deposit);

        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    // Xem deposit
    pub fn get_deposit(env: Env, id: u64) -> CommitmentDeposit {
        env.storage()
            .instance()
            .get(&DataKey::Deposit(id))
            .unwrap_or_else(|| panic!("deposit not found"))
    }

    // Reveal để chứng minh commitment đúng
    pub fn reveal(env: Env, user: Address, id: u64, secret: Bytes) -> bool {
        user.require_auth();

        let mut deposit: CommitmentDeposit = env
            .storage()
            .instance()
            .get(&DataKey::Deposit(id))
            .unwrap_or_else(|| panic!("deposit not found"));

        if user != deposit.user {
            panic!("not owner");
        }

        if deposit.revealed {
            panic!("already revealed");
        }

        let computed_hash: BytesN<32> = env.crypto().sha256(&secret).into();

        if computed_hash != deposit.commitment {
            return false;
        }

        deposit.revealed = true;
        deposit.refunded = true;

        env.storage()
            .instance()
            .set(&DataKey::Deposit(id), &deposit);

        true
    }

    // Trạng thái
    pub fn status(env: Env, id: u64) -> u32 {
        let deposit: CommitmentDeposit = env
            .storage()
            .instance()
            .get(&DataKey::Deposit(id))
            .unwrap_or_else(|| panic!("deposit not found"));

        if deposit.refunded {
            2 // refunded
        } else if deposit.revealed {
            1 // revealed
        } else {
            0 // active
        }
    }

    // Tổng số deposit
    pub fn total_deposits(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::Counter).unwrap_or(0)
    }
}