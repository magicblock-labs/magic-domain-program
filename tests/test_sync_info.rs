use common::TestEnv;
use mdp::state::{field::Addr, record::ErRecord};
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
    let result = ErRecord::deserialize(&acc.data);
    let mut info = assert_ok!(result, "error querying registration PDA from banks {}");

    const NEW_BLOCK_TIME: u16 = 1000;

    let new_addr: Addr = b"http://12.78.13.224:23435".to_vec().into();

    *info.block_time_ms = NEW_BLOCK_TIME;
    info.addr = new_addr.clone();

    let result = common::sync_info(&mut banks, &identity, info).await;
    assert_ok!(result, "error processing sync info transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(
        result,
        "error querying registration PDA after modification {}"
    );
    assert!(matches!(acc, Some(Account { owner: mdp::ID, .. })));
    let acc = acc.unwrap();
    let result = ErRecord::deserialize(&acc.data);
    let info = assert_ok!(
        result,
        "error querying registration PDA post modification {}"
    );
    assert_eq!(info.addr, new_addr);
    assert_eq!(*info.block_time_ms, NEW_BLOCK_TIME);

    let result = common::unregister(&mut banks, &identity, pda).await;

    assert_ok!(result, "error processing unregister transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying unregistered PDA from banks {}");
    assert!(
        acc.is_none(),
        "registration PDA hasn't been removed from banks"
    );
}
