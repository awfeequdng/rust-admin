use crate::models::VideoTags as ThisModel;
use super::Controller;

pub struct VideoTags {}

impl Controller for VideoTags { 

    type M = ThisModel;

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("name", "%"), ("remark", "%")]
    }
}
