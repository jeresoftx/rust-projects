use rust_projects::docker::{Container, State};
#[test]
fn lifecycle_is_explicit() {
    let mut c = Container::new("curso:1", 1);
    assert_eq!(c.state, State::Created);
    c.start().unwrap();
    assert_eq!(c.state, State::Running);
    c.stop().unwrap();
    assert_eq!(c.state, State::Stopped);
    assert!(c.start().is_err());
}
