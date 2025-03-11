use mdp::{
    instructions::{
        register::RegisterInstruction, sync::SyncRecordInstruction,
        unregister::UnregisterInstruction, Instruction,
    },
    state::{
        features::{Feature, FeaturesSet},
        record::ErRecord,
    },
};
use program_test::{BanksClient, BanksClientError, ProgramTest};
use sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction as SolanaInstruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

pub struct TestEnv {
    pub banks: BanksClient,
    pub identity: Keypair,
    pub record: ErRecord,
}

pub async fn setup() -> TestEnv {
    const PROGNAME: &str = "mdp";
    std::env::set_var("SBF_OUT_DIR", "target/deploy/");
    let mut test = ProgramTest::new(PROGNAME, mdp::ID, None);
    let identity = Keypair::new();

    test.add_account(
        identity.pubkey(),
        Account::new(LAMPORTS_PER_SOL, 0, &system_program::ID),
    );

    let features = FeaturesSet::default().activate(Feature::Randomness);
    let record = ErRecord {
        identity: identity.pubkey().into(),
        addr: b"https://241.132.2.41:9324".to_vec().into(),
        block_time_ms: 50.into(),
        fees: 1000.into(),
        features,
    };
    let (banks, _, _) = test.start().await;

    TestEnv {
        banks,
        identity,
        record,
    }
}

pub async fn register(
    banks: &mut BanksClient,
    record: ErRecord,
    identity: &Keypair,
) -> Result<(), BanksClientError> {
    let pda = record.pda().0;
    let ix = Instruction::Register(RegisterInstruction(record));
    let ix = SolanaInstruction::new_with_bytes(
        mdp::ID,
        &ix.serialize(),
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let hash = banks.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[identity], hash);
    banks.process_transaction(tx).await
}

pub async fn unregister(
    banks: &mut BanksClient,
    identity: &Keypair,
    pda: Pubkey,
) -> Result<(), BanksClientError> {
    let ix = Instruction::Unregister(UnregisterInstruction(identity.pubkey()));
    let ix = SolanaInstruction::new_with_bytes(
        mdp::ID,
        &ix.serialize(),
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let hash = banks.get_latest_blockhash().await.unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[&identity], hash);
    banks.process_transaction(tx).await
}

pub async fn sync_info(
    banks: &mut BanksClient,
    identity: &Keypair,
    record: ErRecord,
) -> Result<(), BanksClientError> {
    let pda = record.pda().0;
    let ix = Instruction::SyncRecord(SyncRecordInstruction {
        identity: record.identity,
        addr: Some(record.addr),
        block_time_ms: Some(record.block_time_ms),
        fees: Some(record.fees),
        features: Some(record.features),
    });
    let ix = SolanaInstruction::new_with_bytes(
        mdp::ID,
        &ix.serialize(),
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
        ],
    );
    let hash = banks.get_latest_blockhash().await.unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[&identity], hash);
    banks.process_transaction(tx).await
}

#[macro_export]
macro_rules! assert_ok {
    ($result: ident, $errmsg: expr) => {{
        assert!($result.is_ok(), $errmsg, $result.unwrap_err());
        $result.unwrap()
    }};
}
