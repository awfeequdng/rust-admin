use crate::models::Admins as ThisModel;
use super::Controller;

pub struct Admins { }

impl Controller for Admins { 

    type M = ThisModel;
}
