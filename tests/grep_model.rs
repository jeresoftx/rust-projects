use rust_projects::grep::{search, SearchOptions};

fn options() -> SearchOptions {
    SearchOptions { ignore_case: false }
}

#[test]
fn returns_matching_lines_in_original_order_with_line_numbers() {
    let matches = search("rust", "Rust\nGo\nrustacean\n", options()).unwrap();

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].line_number, 3);
    assert_eq!(matches[0].line, "rustacean");
}

#[test]
fn ignores_case_only_when_requested() {
    let matches = search("rust", "Rust\nrust\n", SearchOptions { ignore_case: true }).unwrap();

    assert_eq!(
        matches
            .into_iter()
            .map(|item| item.line)
            .collect::<Vec<_>>(),
        ["Rust", "rust"]
    );
}

#[test]
fn rejects_an_empty_query() {
    assert_eq!(
        search("", "texto", options()),
        Err("la consulta no puede estar vacía".into())
    );
}
