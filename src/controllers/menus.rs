use crate::models::Menus as ThisModel;
use super::Controller;
use crate::caches::menus::MENUS;

pub struct Menus { }

impl Controller for Menus { 

    type M = ThisModel;

    fn edit_after(data: &mut tera::Context) {
        data.insert("menus", &*MENUS);
    }

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("name", "%"), ("state", "="), ("url", "%"), ("is_blank", "=")]
    }
}
