use serde_json::{Value};

pub struct Config{
    fields: Value
}

impl Config{

    pub fn new(fields: Value) -> Self {
        Self {fields}
    }
 
    pub fn from_str(json: &str) -> Value {
        let fields: Value = serde_json::from_str(json).unwrap();
        fields
    }

    pub fn read_config(&self, field: &str) -> String {
        match self.fields[field] {
            Value::String(ref v) => v.to_string(),
            _ => panic!("Invalid field type")
        }
    }
}