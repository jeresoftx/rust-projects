//! Almacén clave-valor con expiración perezosa para Redis educativo.

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Store {
    values: HashMap<String, (String, Option<u64>)>,
}

impl Store {
    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.into(), (value.into(), None));
    }
    pub fn get(&mut self, key: &str, now: u64) -> Option<String> {
        let expired = self
            .values
            .get(key)
            .is_some_and(|(_, deadline)| deadline.is_some_and(|time| time <= now));
        if expired {
            self.values.remove(key);
            return None;
        }
        self.values.get(key).map(|(value, _)| value.clone())
    }
    pub fn expire(&mut self, key: &str, deadline: u64) -> bool {
        if let Some((_, expires_at)) = self.values.get_mut(key) {
            *expires_at = Some(deadline);
            true
        } else {
            false
        }
    }
    pub fn delete(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
