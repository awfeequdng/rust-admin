use crate::models::Menus as ThisModel;
use super::Controller;

pub struct Menus { }

impl Controller for Menus { 

    type M = ThisModel;

    fn get_controller_name() -> &'static str { 
        "menus"
    }
}
