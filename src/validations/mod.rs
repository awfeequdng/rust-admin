use std::collections::HashMap;

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

    pub fn is_username(&mut self, field: &'static str, message: &'static str, is_required: bool) -> &mut Self { 
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

    pub fn validate(&mut self) -> Result<(), String> { 
        if self.errors.len() > 0 { 
            return Err(self.errors.join(","));
        }
        Ok(())
    }
}
