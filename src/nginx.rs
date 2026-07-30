//! Routing por prefijo más largo para NGINX educativo.

#[derive(Debug, Default)]
pub struct Router {
    rules: Vec<(String, String)>,
}
impl Router {
    pub fn route(mut self, prefix: &str, backend: &str) -> Self {
        self.rules.push((prefix.into(), backend.into()));
        self
    }
    pub fn resolve(&self, path: &str) -> Result<&str, String> {
        self.rules
            .iter()
            .filter(|(prefix, _)| path.starts_with(prefix))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(_, backend)| backend.as_str())
            .ok_or_else(|| "no hay backend para la ruta".into())
    }
}
