use mdp::{
    instructions::{sync::SyncInstruction, version::v0::SyncRecordV0, Instruction},
    state::{
        features::{Feature, FeaturesSet},
        record::{CountryCode, ErRecord},
        status::ErStatus,
        version::v0::RecordV0,
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
    let record = ErRecord::V0(RecordV0 {
        identity: identity.pubkey(),
        status: ErStatus::Active,
        addr: "https://241.132.2.41:9324/".to_string(),
        block_time_ms: 50,
        base_fee: 1000,
        features,
        load_average: 1_000_000,
        country_code: CountryCode::from("083"),
    });
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
    let ix = Instruction::Register(record);
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
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
    let ix = Instruction::Unregister(identity.pubkey());
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
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

pub async fn sync(
    banks: &mut BanksClient,
    identity: &Keypair,
    record: ErRecord,
) -> Result<(), BanksClientError> {
    let pda = record.pda().0;
    let ix = Instruction::Sync(SyncInstruction::V0(SyncRecordV0 {
        identity: *record.identity(),
        status: Some(record.status()),
        addr: Some(record.addr().to_owned()),
        block_time_ms: Some(record.block_time_ms()),
        base_fee: Some(record.base_fee()),
        features: Some(record.features().clone()),
        load_average: Some(record.load_average()),
        country_code: Some(record.country_code()),
    }));
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
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
