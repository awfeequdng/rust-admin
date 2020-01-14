use crate::models::UserLevels as ThisModel;
use super::Controller;

pub struct UserLevels {}

impl Controller for UserLevels { 

    type M = ThisModel;
}
