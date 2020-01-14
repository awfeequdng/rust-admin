use std::collections::HashMap;
use serde_json::value::Value;
use tera::{Result};

/// 状态说明
pub const STATES: [&'static str; 2] = ["正常", "禁用"];

pub fn state_name<'r, 's>(val: &'r Value, _data: &'s HashMap<String, Value>) -> Result<Value> { 
    if let Value::Number(n) = val { 
        let n = n.as_u64().unwrap();
        if n != 0 && n != 1 { 
            return Ok(json!("未知等级"));
        }
        
        return Ok(json!(STATES[n as usize]));
    }
    Ok(json!("错误!!!"))
}

pub mod menus;
