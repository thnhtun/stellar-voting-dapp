# Title
CommitStake – Commitment-based Deposit on Stellar
# Description
This project explores a simple mechanism to improve commitment and accountability using blockchain.

In many real-world situations (study, freelance tasks, group work), people often fail to follow through on their commitments. This project introduces a smart contract where users can make a commitment and attach a deposit to it.

By using a cryptographic hash (commitment), users can prove later that they did not change their original intention, ensuring integrity and trust without revealing sensitive information upfront.
# Features
Create Deposit with Commitment
Users can create a deposit by submitting:
their address
an amount (stake)
a commitment (SHA-256 hash of a secret)
Privacy-Preserving Commitment
The actual content (secret) is not stored on-chain.
Only the hash is stored, ensuring privacy.

Reveal Mechanism
Users can later reveal their secret.
The contract verifies:

hash(secret) == commitment

State Management
Each deposit has a status:
0: Active (not revealed)
2: Completed (revealed successfully)
Tamper-proof Logic
Once a commitment is stored, it cannot be modified.
Users cannot fake or change their commitment afterward.
# Contract
https://stellar.expert/explorer/testnet/contract/CBH3LI2KJYQBUJ6S7RHAG6F6FLCAV2NJRTPPCSKSHNZK32ZNUSHLOCCR

Contract's scre![screenshot](![screenshot](Screenshot 2026-04-23 161137.png)
# Future scopes
Verifier Integration
Add a third-party verifier (admin) to confirm real-world actions before refund.
Deadline Mechanism
Add a time limit. If the user does not reveal in time, the deposit is forfeited.
Penalty Distribution
Lost deposits could be redistributed to other participants or a pool.
Oracle Integration
Connect with external data sources to verify real-world conditions automatically.
Real Token Integration
Replace mock logic with actual token transfers on Stellar.
# Profile
Name: Tuan Nguyen
Role: Student – Information Systems

Skills:

Blockchain basics (Stellar, Soroban)
Smart contract development (Rust)
Web3 concepts (commit-reveal, hashing, on-chain logic)
#Projetc Vision
The goal of this project is to create a transparent and trustless commitment mechanism using blockchain.

It enables users to make commitments that cannot be altered and can be verified later.

In the future, this system can be extended to support accountability in education, teamwork, and decentralized applications requiring integrity and trust.