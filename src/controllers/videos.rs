use crate::models::Videos as ThisModel;
use super::Controller;

pub struct Videos { }

impl Controller for Videos { 

    type M = ThisModel;

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("title", "%"), ("remark", "%"), ("created", "[date]"), ("updated", "[date]")]
    }
}
