//! Log particionado en memoria para Kafka educativo.

use std::collections::BTreeMap;
#[derive(Debug, Default)]
pub struct Topic {
    partitions: Vec<Vec<String>>,
    offsets: BTreeMap<(String, usize), usize>,
}
impl Topic {
    pub fn new(partitions: usize) -> Self {
        Self {
            partitions: vec![Vec::new(); partitions],
            offsets: BTreeMap::new(),
        }
    }
    pub fn produce(&mut self, partition: usize, message: &str) -> Result<usize, String> {
        let log = self
            .partitions
            .get_mut(partition)
            .ok_or_else(|| "partición inexistente".to_string())?;
        log.push(message.into());
        Ok(log.len() - 1)
    }
    pub fn consume(&mut self, group: &str, partition: usize) -> Result<Option<String>, String> {
        let log = self
            .partitions
            .get(partition)
            .ok_or_else(|| "partición inexistente".to_string())?;
        let key = (group.into(), partition);
        let offset = *self.offsets.get(&key).unwrap_or(&0);
        let message = log.get(offset).cloned();
        if message.is_some() {
            self.offsets.insert(key, offset + 1);
        }
        Ok(message)
    }
}
