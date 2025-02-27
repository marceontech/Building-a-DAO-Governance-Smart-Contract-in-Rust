# Building-a-DAO-Governance-Smart-Contract-in-Rust

How DAO Governance Works on Solana?
1) Firstly members stake governance tokens to participate in decision-making.
2) Proposals are created and submitted on-chain.
3) Voting occurs within a set period.
4) Winning proposals are executed via smart contracts.

# Explaning main functions:

create_proposal(): Allows DAO members to submit new proposals

vote(): Token holders vote for or against proposals

execute_proposal(): Executes proposals if approved by majority.


# Interacting with the DAO Governance Contract

1) Create a proposal:

``` solana program invoke --program-id <PROGRAM_ID> --signer <WALLET> --data create_proposal ```

2) Cast a vote:

``` solana program invoke --program-id <PROGRAM_ID> --signer <WALLET> --data vote --approve ```

3) Execute an approved proposal:

``` solana program invoke --program-id <PROGRAM_ID> --signer <ADMIN_WALLET> --data execute_proposal ```


* The SPL Governance Program (Docs) provides a pre-built framework for DAOs on Solana. You can more detailed info here:
  
  https://docs.rs/spl-governance/1.1.1/spl_governance/state/governance/struct.GovernanceConfig.html 


