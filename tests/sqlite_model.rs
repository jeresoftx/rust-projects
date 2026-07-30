use rust_projects::sqlite::{Row, Table};
#[test]
fn insert_selects_and_orders_rows() {
    let mut table = Table::new(2);
    table
        .insert(Row {
            id: 2,
            text: "b".into(),
        })
        .unwrap();
    table
        .insert(Row {
            id: 1,
            text: "a".into(),
        })
        .unwrap();
    assert_eq!(table.select(1).unwrap().text, "a");
    assert_eq!(
        table
            .scan_page(0)
            .iter()
            .map(|row| row.id)
            .collect::<Vec<_>>(),
        [1, 2]
    );
}
#[test]
fn rejects_duplicate_ids() {
    let mut table = Table::new(1);
    table
        .insert(Row {
            id: 1,
            text: "a".into(),
        })
        .unwrap();
    assert_eq!(
        table.insert(Row {
            id: 1,
            text: "b".into()
        }),
        Err("el identificador ya existe".into())
    );
}
