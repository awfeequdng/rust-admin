use std::collections::HashMap;
use crate::models::Admins as ThisModel;
use super::Controller;
use crate::caches::admin_roles::ADMIN_ROLES;

pub struct Admins { }

impl Controller for Admins { 

    type M = ThisModel;

    fn edit_data() -> Option<(&'static str, HashMap<String, String>)> { 
        let roles = ADMIN_ROLES.lock().unwrap();
        Some(("roles", roles.to_owned()))
    }
}
