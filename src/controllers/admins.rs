use crate::models::Admins as ThisModel;
use super::Controller;

pub struct Admins { }

impl Controller<ThisModel> for Admins { 

    type M = ThisModel;
}
