use rust_projects::curl::{build_get_request, parse_http_url, parse_response, HttpUrl};

#[test]
fn parses_a_plain_http_url() {
    assert_eq!(
        parse_http_url("http://example.test:8080/ruta").unwrap(),
        HttpUrl {
            host: "example.test".into(),
            port: 8080,
            path: "/ruta".into()
        }
    );
}

#[test]
fn rejects_non_http_urls() {
    assert_eq!(
        parse_http_url("https://example.test"),
        Err("solo se admite el esquema http".into())
    );
}

#[test]
fn builds_a_connection_close_get_request() {
    let request = build_get_request(&HttpUrl {
        host: "example.test".into(),
        port: 80,
        path: "/".into(),
    });
    assert_eq!(
        String::from_utf8(request).unwrap(),
        "GET / HTTP/1.1\r\nHost: example.test\r\nConnection: close\r\n\r\n"
    );
}

#[test]
fn parses_status_headers_and_body() {
    let response =
        parse_response(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nhola").unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(
        response.headers,
        [("content-type".into(), "text/plain".into())]
    );
    assert_eq!(response.body, b"hola");
}
