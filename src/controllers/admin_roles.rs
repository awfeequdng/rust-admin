use crate::models::AdminRoles as ThisModel;
use super::Controller;

pub struct AdminRoles { }

impl Controller for AdminRoles { 

    type M = ThisModel;

    fn get_controller_name() -> &'static str { 
        "admin_roles"
    }
}
