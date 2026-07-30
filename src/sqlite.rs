//! Tabla paginada mínima para SQLite educativo.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: u64,
    pub text: String,
}

#[derive(Debug)]
pub struct Table {
    rows: Vec<Row>,
    page_size: usize,
}

impl Table {
    pub fn new(page_size: usize) -> Self {
        Self {
            rows: Vec::new(),
            page_size,
        }
    }
    pub fn insert(&mut self, row: Row) -> Result<(), String> {
        if self.rows.iter().any(|item| item.id == row.id) {
            return Err("el identificador ya existe".into());
        }
        self.rows.push(row);
        self.rows.sort_by_key(|item| item.id);
        Ok(())
    }
    pub fn select(&self, id: u64) -> Option<&Row> {
        self.rows.iter().find(|item| item.id == id)
    }
    pub fn scan_page(&self, page: usize) -> &[Row] {
        let start = page.saturating_mul(self.page_size);
        self.rows
            .get(start..(start + self.page_size).min(self.rows.len()))
            .unwrap_or(&[])
    }
}
