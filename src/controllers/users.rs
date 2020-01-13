use crate::models::Users as ThisModel;
use super::Controller;

pub struct Users { }

impl Controller for Users { 

    type M = ThisModel;
}
