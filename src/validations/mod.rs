use std::collections::HashMap;
use regex::Regex;

lazy_static! { 
    static ref RE_USERNAME: Regex = { Regex::new(r"^[a-zA-Z]+[a-zA-Z_0-9]{5,19}$").unwrap() };
    static ref RE_MAIL: Regex = { Regex::new(r"/^([A-Za-z0-9_\-\.])+\@([A-Za-z0-9_\-\.])+\.([A-Za-z]{2,5})$/").unwrap() };
}

#[derive(Debug)]
pub struct Validator<'a> { 
    errors: Vec<&'static str>,
    data: &'a HashMap<String, String>
}


pub trait Validation { 
    fn validate(_data: &HashMap<String, String>) -> Result<(), String> { 
        Ok(())
    }
}

impl<'a> Validator<'a> { 
    
    pub fn load(data: &'a HashMap<String, String>) -> Self { 
        Self { 
            errors: vec![],
            data: data,
        }
    }

    /// 是否是用户名称, 6-20位, 英文开头, 数字、下划线、英文
    pub fn is_username(&mut self, field: &'static str, message: &'static str, is_required: bool) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            let count = v.chars().count();
            if count < 5 || count > 20 || !RE_USERNAME.is_match(v) { 
                self.errors.push(message);
            }
        } else if is_required { 
            self.errors.push(message);
        }
        self
    }

    /// 检测是否是正确的密码格式
    pub fn is_password(&mut self, field: &'static str, message: &'static str, is_required: bool) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            let count = v.chars().count();
            if count < 5 || count > 20 { 
                self.errors.push(message);
            }
        } else if is_required { 
            self.errors.push(message);
        }
        self
    }

    /// 检测是否是正确的验证码
    pub fn is_check_code(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Err(_) = v.parse::<usize>() { 
                self.errors.push(message);
            }
        } else { 
            self.errors.push(message);
        }
        self
    }

    /// 判断是否是电子邮件
    pub fn is_mail(&mut self, field: &'static str, message: &'static str, is_required: bool) -> &mut Self  {
        if let Some(v) = self.data.get(field) { 
            if RE_MAIL.is_match(v) { 
                self.errors.push(message);
            }
        } else if is_required { 
            self.errors.push(message);
        }
        self
    }

    /// 指定长度的字符串
    pub fn length(&mut self, field: &'static str, message: &'static str, min: usize, max: usize, is_required: bool) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if v.len() < min || v.len() > max {
                self.errors.push(message);
            }
        } else if is_required { 
            self.errors.push(message);
        }
        self
    }

    /// 执行校验
    pub fn validate(&mut self) -> Result<(), String> { 
        if self.errors.len() > 0 { 
            return Err(self.errors.join(","));
        }
        Ok(())
    }
}
