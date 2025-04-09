use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Ok;

pub struct DB {
    key_value: Arc<Mutex<HashMap<String, String>>>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            key_value: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set_key_value(&self, key: String, value: String) -> anyhow::Result<crate::redis::Reply> {
        let mut data = self
            .key_value
            .lock()
            .map_err(|_| anyhow::anyhow!("set_key_value: lock db fail"))?;
        data.insert(key, value);

        Ok(crate::redis::Reply::Ok)
    }

    pub fn get_key_value(&self, key: String) -> anyhow::Result<crate::redis::Reply> {
        let data = self
            .key_value
            .lock()
            .map_err(|_| anyhow::anyhow!("get_key_value: lock db fail"))?;

        match data.get(&key) {
            Some(value) => Ok(crate::redis::Reply::String(String::from(value))),
            None => Ok(crate::redis::Reply::Nil),
        }
    }
}
