use borsh::BorshDeserialize;
use common::TestEnv;
use mdp::state::{
    features::{Feature, FeaturesSet},
    record::ErRecord,
};
use sdk::{account::Account, signer::Signer};

pub mod common;

#[tokio::test]
async fn test_registration() {
    let TestEnv {
        mut banks,
        identity,
        record,
        ..
    } = common::setup().await;
    let addr = record.addr().to_owned();
    let pda = record.pda().0;

    let result = common::register(&mut banks, record, &identity).await;

    assert_ok!(result, "error processing register transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying registration PDA from banks {}");
    assert!(matches!(acc, Some(Account { owner: mdp::ID, .. })));
    let acc = acc.unwrap();
    let result = ErRecord::try_from_slice(&acc.data);
    let info = assert_ok!(result, "error querying registration PDA from banks {}");
    assert_eq!(*info.identity(), identity.pubkey());
    assert_eq!(
        info.features(),
        FeaturesSet::default().activate(Feature::Randomness)
    );
    assert_eq!(info.addr(), addr);
}
