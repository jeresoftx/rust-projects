//! Búsqueda literal por líneas para el proyecto educativo de `grep`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchOptions {
    pub ignore_case: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub line_number: usize,
    pub line: String,
}

pub fn search(query: &str, input: &str, options: SearchOptions) -> Result<Vec<Match>, String> {
    if query.is_empty() {
        return Err("la consulta no puede estar vacía".into());
    }

    let normalized_query = options.ignore_case.then(|| query.to_lowercase());
    Ok(input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let matches = normalized_query.as_ref().map_or_else(
                || line.contains(query),
                |needle| line.to_lowercase().contains(needle),
            );
            matches.then(|| Match {
                line_number: index + 1,
                line: line.to_owned(),
            })
        })
        .collect())
}
