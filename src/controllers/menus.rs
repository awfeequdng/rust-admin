use crate::models::Menus as ThisModel;
use super::Controller;

pub struct Menus { }

impl Controller<ThisModel> for Menus { 

    type M = ThisModel;
}
