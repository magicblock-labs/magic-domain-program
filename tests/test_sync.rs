use borsh::BorshDeserialize;
use common::TestEnv;
use mdp::state::{record::ErRecord, status::ErStatus};
use sdk::account::Account;

pub mod common;

#[tokio::test]
async fn test_sync_info() {
    let TestEnv {
        mut banks,
        identity,
        record,
        ..
    } = common::setup().await;
    let pda = record.pda().0;

    let result = common::register(&mut banks, record, &identity).await;
    assert_ok!(result, "error processing register transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying registration PDA from banks {}");
    assert!(matches!(acc, Some(Account { owner: mdp::ID, .. })));
    let acc = acc.unwrap();
    let result = ErRecord::try_from_slice(&acc.data);
    let mut record = assert_ok!(result, "error querying registration PDA from banks {}");

    const NEW_BLOCK_TIME: u16 = 1000;
    const NEW_ADDR: &str = "https://127.145.24.55:9324";

    record.set_block_time_ms(NEW_BLOCK_TIME);
    record.set_addr(NEW_ADDR.to_string());
    record.set_status(ErStatus::Draining);
    record.set_load_average(2_200_000);

    let result = common::sync(&mut banks, &identity, record).await;
    assert_ok!(result, "error processing sync info transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(
        result,
        "error querying registration PDA after modification {}"
    );
    assert!(matches!(acc, Some(Account { owner: mdp::ID, .. })));
    let acc = acc.unwrap();
    let result = ErRecord::try_from_slice(&acc.data);
    let record = assert_ok!(
        result,
        "error querying registration PDA post modification {}"
    );
    assert_eq!(record.addr(), NEW_ADDR);
    assert_eq!(record.block_time_ms(), NEW_BLOCK_TIME);
    assert_eq!(record.status(), ErStatus::Draining);
    assert_eq!(record.load_average(), 2_200_000);

    let result = common::unregister(&mut banks, &identity, pda).await;

    assert_ok!(result, "error processing unregister transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying unregistered PDA from banks {}");
    assert!(
        acc.is_none(),
        "registration PDA hasn't been removed from banks"
    );
}
