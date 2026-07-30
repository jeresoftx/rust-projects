use rust_projects::kafka::Topic;
#[test]
fn groups_keep_independent_offsets() {
    let mut topic = Topic::new(1);
    assert_eq!(topic.produce(0, "a"), Ok(0));
    assert_eq!(topic.produce(0, "b"), Ok(1));
    assert_eq!(topic.consume("uno", 0), Ok(Some("a".into())));
    assert_eq!(topic.consume("dos", 0), Ok(Some("a".into())));
    assert_eq!(topic.consume("uno", 0), Ok(Some("b".into())));
}
