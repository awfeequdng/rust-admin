use std::collections::HashMap;
use crate::validations::{Validator};

pub struct Index { 
}

impl Index { 

    pub fn check_login(data: &HashMap<String, String>) -> Result<(), String> { 
        if let Err(message) = Validator::load(data)
            .is_username("username", "必须输入正确格式的用户名称", true)
            .validate() { 
                return Err(message);
        }
        
        Ok(())
    }
}