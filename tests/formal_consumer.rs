#![forbid(unsafe_code)]

use dpm::formal::{LeaseState, Migration, OwnerId, ValidationProof};

fn main() {
    let mut leases = LeaseState::default();
    let guard = leases
        .acquire(OwnerId::new("cli-contract-consumer").expect("valid owner"))
        .expect("first owner acquires the lease");

    let migration =
        Migration::new("SELECT 1").validate(|sql| ValidationProof::for_bytes(sql.as_bytes(), 1));
    let authorized = migration.authorize(&guard);

    assert_eq!(authorized.owner().as_str(), "cli-contract-consumer");
    assert_eq!(authorized.epoch(), 1);
    assert_eq!(authorized.proof().checked_items(), 1);
    assert_ne!(authorized.proof().fingerprint(), 0);

    let applied = authorized.finish();
    assert_eq!(applied.into_inner(), "SELECT 1");

    let receipt = guard.release();
    assert_eq!(receipt.owner().as_str(), "cli-contract-consumer");
    assert_eq!(receipt.epoch(), 1);
    assert!(leases.owner().is_none());
}
