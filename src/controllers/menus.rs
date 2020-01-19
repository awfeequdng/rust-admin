use crate::models::Menus as ThisModel;
use super::Controller;
use crate::caches::menus::MENUS;

pub struct Menus { }

impl Controller for Menus { 

    type M = ThisModel;

    fn edit_after(data: &mut tera::Context) {
        let menus = MENUS.lock().unwrap();
        data.insert("menus", &*menus);
    }
}
