use std::collections::HashMap;
use serde_json::value::Value;
use tera::{Result};

pub fn level_name<'r, 's>(val: &'r Value, data: &'s HashMap<String, Value>) -> Result<Value> { 
    if let Value::Number(n) = val { 
        let n = n.as_u64().unwrap();
        if n != 0 && n != 1 { 
            return Ok(val.to_owned());
        }
    }
    println!("val = {:?}, data = {:?}", val, data);
    Ok(val.to_owned())
}
