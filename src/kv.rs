use std::{collections::HashMap, path::Path};
use crate::{KvsError, Result};

pub struct KvStore {
    store: HashMap<String, String>
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self>{
        todo!()
    }
    pub fn new() -> Self {
        Self { 
            store: HashMap::new()
         }
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()>{
        self.store.remove(&key);
        Ok(())
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}