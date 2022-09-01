use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::{anyhow, Result};

pub struct Json;

impl Json {
    pub fn str_to_obj<'a, T: Deserialize<'a>>(&self, str: &'a str) -> Result<T> {
        let result = serde_json::from_str::<'a, T>(str);
        match result {
            Ok(r) => Ok(r),
            Err(err) => Err(anyhow!("str_to_obj err {}",err)),
        }
    }

    pub fn str_to_json<'a>(&self, str: &'a str) -> Result<Value> {
        let result = serde_json::from_str::<'a, Value>(str);
        match result {
            Ok(r) => Ok(r),
            Err(err) => Err(anyhow!("str_to_json err {}",err)),
        }
    }

    pub fn json_to_obj<T: DeserializeOwned>(&self, value: Value) -> Result<T> {
        let result = serde_json::from_value::<T>(value);
        match result {
            Ok(r) => Ok(r),
            Err(err) => Err(anyhow!("json_to_obj err {}",err)),
        }
    }

    pub fn obj_to_string<T: ?Sized + Serialize>(&self, obj: &T) -> Result<String> {
        let result = serde_json::to_string(obj);
        match result {
            Ok(r) => Ok(r),
            Err(err) => Err(anyhow!("obj_to_string err {}",err)),
        }
    }

    pub fn obj_to_json<T: Serialize>(&self, obj: &T) -> Result<Value> {
        let result = serde_json::to_value(obj);
        match result {
            Ok(r) => Ok(r),
            Err(err) => Err(anyhow!("obj_to_json err {}",err)),
        }
    }

    pub fn json_to_string(&self, value: Value) -> Result<String> {
        let result = serde_json::to_string(&value);
        match result {
            Ok(r) => Ok(r),
            Err(err) => Err(anyhow!("json_to_string err {}",err)),
        }
    }
}
