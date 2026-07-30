use rust_projects::load_balancer::{Backend, RoundRobin};

fn backend(address: &str, healthy: bool) -> Backend {
    Backend {
        address: address.into(),
        healthy,
    }
}

#[test]
fn selects_healthy_backends_in_rotation() {
    let mut balancer = RoundRobin::new(vec![backend("a", true), backend("b", true)]);
    assert_eq!(balancer.select_next().unwrap().address, "a");
    assert_eq!(balancer.select_next().unwrap().address, "b");
    assert_eq!(balancer.select_next().unwrap().address, "a");
}

#[test]
fn skips_unhealthy_backends_without_losing_rotation() {
    let mut balancer = RoundRobin::new(vec![backend("a", false), backend("b", true)]);
    assert_eq!(balancer.select_next().unwrap().address, "b");
}

#[test]
fn reports_when_no_backend_is_healthy() {
    let mut balancer = RoundRobin::new(vec![backend("a", false)]);
    assert_eq!(
        balancer
            .select_next()
            .map(|backend| backend.address.clone()),
        Err("no hay backends saludables".into())
    );
}
