//! Selección round-robin determinista para un balanceador educativo.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    pub address: String,
    pub healthy: bool,
}

#[derive(Debug)]
pub struct RoundRobin {
    backends: Vec<Backend>,
    cursor: usize,
}

impl RoundRobin {
    pub fn new(backends: Vec<Backend>) -> Self {
        Self {
            backends,
            cursor: 0,
        }
    }
    pub fn select_next(&mut self) -> Result<&Backend, String> {
        let total = self.backends.len();
        if total == 0 {
            return Err("no hay backends saludables".into());
        }
        for offset in 0..total {
            let index = (self.cursor + offset) % total;
            if self.backends[index].healthy {
                self.cursor = (index + 1) % total;
                return Ok(&self.backends[index]);
            }
        }
        Err("no hay backends saludables".into())
    }
}
