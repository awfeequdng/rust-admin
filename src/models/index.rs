use std::collections::HashMap;
use crate::validations::{Validator};

pub struct Index { 
}

impl Index { 

    /// 检测用户登录
    pub fn check_login(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(data)
            .is_username("username", "必须输入正确格式的用户名称", true)
            .is_password("password", "必须输入密码")
            .validate()
    }
}
