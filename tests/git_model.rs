use rust_projects::git::Repository;
#[test]
fn commits_keep_old_snapshots_immutable() {
    let mut repo = Repository::default();
    repo.stage("a", "uno");
    let first = repo.commit("main");
    repo.stage("a", "dos");
    let second = repo.commit("main");
    assert_eq!(repo.snapshot(first).unwrap()["a"], "uno");
    assert_eq!(repo.snapshot(second).unwrap()["a"], "dos");
    assert_eq!(repo.reference("main"), Some(second));
}
