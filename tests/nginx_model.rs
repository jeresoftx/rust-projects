use rust_projects::nginx::Router;
#[test]
fn most_specific_prefix_wins() {
    let router = Router::default().route("/", "web").route("/api", "api");
    assert_eq!(router.resolve("/api/cursos"), Ok("api"));
    assert_eq!(router.resolve("/inicio"), Ok("web"));
}
