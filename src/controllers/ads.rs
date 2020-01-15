use crate::models::Ads as ThisModel;
use super::Controller;

pub struct Ads {}

impl Controller for Ads { 

    type M = ThisModel;
}
