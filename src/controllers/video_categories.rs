use crate::models::VideoCategories as ThisModel;
use super::Controller;

pub struct VideoCategories { }

impl Controller for VideoCategories { 

    type M = ThisModel;

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("name", "%"), ("remark", "%")]
    }
}
