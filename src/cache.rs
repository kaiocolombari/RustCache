use std::sync::{Arc, RwLock};

// Apelido genérico para facilitar o uso nas threads
pub type SharedCache<Key, Value> = Arc<RwLock<Cache<Key, Value>>>;

struct Cache<Key, Value> {
    max_size: usize,
    dados: Vec<(Key, Value)>,
}

impl<Key: PartialEq, Value> Cache<Key, Value> {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            dados: Vec::with_capacity(max_size),
        }
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Result<(), &'static str> {
        if self.max_size == 0 {
            return Err("Impossivel inserir em um cache de tamanho 0");
        }

        if self.dados.iter().any(|(k, _)| k == &key) {
            return Err("Chave já existe");
        }

        if self.dados.len() >= self.max_size {
            self.dados.remove(0);
        }

        self.dados.push((key, value));
        Ok(())
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.dados.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn clear(&mut self) {
        self.dados.clear();
    }

    pub fn remove(&mut self, key: &Key) -> bool {
        if let Some(index) = self.dados.iter().position(|(k, _)| k == key) {
            self.dados.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool{
        self.dados.is_empty()
    }

    pub fn len(&self) -> usize {
        self.dados.len()
    }
}
