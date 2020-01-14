use std::collections::HashMap;
use serde_json::value::Value;
use tera::{Result};
use crate::caches::menus::MENU_LEVELS;

/// 菜单等级名称
pub fn level_name<'r, 's>(val: &'r Value, _data: &'s HashMap<String, Value>) -> Result<Value> { 
    if let Value::Number(n) = val { 
        let n = n.as_u64().unwrap();
        if n != 0 && n != 1 { 
            return Ok(json!("未知等级"));
        }
        
        return Ok(json!(MENU_LEVELS[n as usize]));
    }
    Ok(json!("错误!!!"))
}
