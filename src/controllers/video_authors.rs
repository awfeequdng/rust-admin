use crate::models::VideoAuthors as ThisModel;
use super::Controller;

pub struct VideoAuthors { }

impl Controller for VideoAuthors { 

    type M = ThisModel;

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("name", "%"), ("remark", "%")]
    }
}
