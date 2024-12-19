
# Magic Domain Program

Magic Domain Program, a Solana smart contract which allows Ephemeral Rollups (ERs) to register themselves on chain, thus allowing clients to discover those ERs along with the services they provide. 

## Table of Contents

1. [Introduction](#introduction)
2. [Usage](#usage)

## Introduction

The Magic Domain Program is a Solana-based smart contract that facilitates registration of Ephemeral Rollups providers on solana blockchain. The registration process allows those provides to advertise themselves to users, declaring various parameters of their services, like IP address via which the ER can reached, block time, fees, supported features and so on.


## Usage

Currently the program supports 3 instructions:
1. registration
2. state synchronization
3. unregistration 

Once deployed, the Magic Domain Program can be interacted with using regular transactions. Here are some example commands:

1. **Register ER**

   ```rust
    let identity = Keypair::new();
    let features = FeaturesSet::default().activate(Feature::Randomness);
    // here we declare all the parameters of our ER
    let info = ValidatorInfo {
        identity: identity.pubkey(),
        addr: SocketAddrV4::from_str("241.132.2.41:9324").unwrap(),
        block_time_ms: 50,
        fees: 1000,
        features,
    };
    let ix = Instruction::Register(RegisterInstruction(info));
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let hash = rpc.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[identity], hash);
    rpc.send_transaction(tx).await
   ```

2. **Sync ER parameters with chain**

   ```rust
    let ix = Instruction::SyncInfo(SyncInfoInstruction {
        identity: identity.pubkey(),
        addr: Some(SocketAddrV4::from_str("21.132.1.4:9324").unwrap()),
        block_time_ms: Some(50),
        fees: None,
        features: None,
    });
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
        ],
    );
    let hash = rpc.get_latest_blockhash().await.unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[&identity], hash);
    rpc.send_transaction(tx).await

   ```

3. **Unregister ER (delete record on chain)**

   ```rust
   let ix = Instruction::Unregister(UnregisterInstruction(identity.pubkey()));
   let ix = SolanaInstruction::new_with_borsh(
           mdp::ID,
           &ix,
           vec![
           AccountMeta::new(identity.pubkey(), true),
           AccountMeta::new(pda, false),
           AccountMeta::new_readonly(system_program::ID, false),
           ],
           );
    let hash = rpc.get_latest_blockhash().await.unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[&identity], hash);
    rpc.send_transaction(tx).await
   ```

