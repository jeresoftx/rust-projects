//! Snapshots inmutables y referencias mutables para Git educativo.

use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Repository {
    index: BTreeMap<String, String>,
    commits: Vec<BTreeMap<String, String>>,
    refs: BTreeMap<String, usize>,
}

impl Repository {
    pub fn stage(&mut self, path: &str, blob: &str) {
        self.index.insert(path.into(), blob.into());
    }
    pub fn commit(&mut self, reference: &str) -> usize {
        self.commits.push(self.index.clone());
        let id = self.commits.len() - 1;
        self.refs.insert(reference.into(), id);
        id
    }
    pub fn reference(&self, name: &str) -> Option<usize> {
        self.refs.get(name).copied()
    }
    pub fn snapshot(&self, id: usize) -> Option<&BTreeMap<String, String>> {
        self.commits.get(id)
    }
}
