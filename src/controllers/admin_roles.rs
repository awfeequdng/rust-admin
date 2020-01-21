use crate::models::AdminRoles as ThisModel;
use super::Controller;
use crate::models::Menus;

pub struct AdminRoles { }

impl Controller for AdminRoles { 

    type M = ThisModel;

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("name", "%"), ("remark", "%")]
    }

    fn edit_after(data: &mut tera::Context) { 
        data.insert("menus", &Menus::get_related());
    }
}
