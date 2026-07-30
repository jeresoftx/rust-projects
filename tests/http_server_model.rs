use rust_projects::http_server::{parse_request, serialize_response, Request, Response, Router};

#[test]
fn parses_a_minimal_request_line() {
    assert_eq!(
        parse_request(b"GET /salud HTTP/1.1\r\nHost: local\r\n\r\n").unwrap(),
        Request {
            method: "GET".into(),
            path: "/salud".into()
        }
    );
}

#[test]
fn routes_exact_matches_and_distinguishes_method_errors() {
    let router = Router::new().route("GET", "/salud", "ok");
    assert_eq!(
        router.handle(Request {
            method: "GET".into(),
            path: "/salud".into()
        }),
        Response {
            status: 200,
            body: "ok".into()
        }
    );
    assert_eq!(
        router
            .handle(Request {
                method: "POST".into(),
                path: "/salud".into()
            })
            .status,
        405
    );
    assert_eq!(
        router
            .handle(Request {
                method: "GET".into(),
                path: "/ausente".into()
            })
            .status,
        404
    );
}

#[test]
fn serializes_content_length() {
    let bytes = serialize_response(&Response {
        status: 200,
        body: "ok".into(),
    });
    assert_eq!(
        String::from_utf8(bytes).unwrap(),
        "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok"
    );
}
