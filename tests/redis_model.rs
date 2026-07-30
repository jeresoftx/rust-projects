use rust_projects::redis::Store;

#[test]
fn set_replaces_value_and_expiration() {
    let mut store = Store::default();
    store.set("a", "uno");
    store.expire("a", 2);
    store.set("a", "dos");
    assert_eq!(store.get("a", 9), Some("dos".into()));
}
#[test]
fn expired_values_disappear_lazily() {
    let mut store = Store::default();
    store.set("a", "uno");
    assert!(store.expire("a", 2));
    assert_eq!(store.get("a", 2), None);
    assert!(!store.delete("a"));
}
#[test]
fn expire_missing_key_is_false() {
    assert!(!Store::default().expire("ausente", 1));
}
