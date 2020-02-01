use crate::models::Videos as ThisModel;
use super::Controller;
use crate::config;

pub struct Videos { }

impl Controller for Videos { 

    type M = ThisModel;

    fn edit_after(data: &mut tera::Context) {
        let info = config::get_oss_info();
        data.insert("bucket", &info.bucket);
        data.insert("end_point", &info.end_point);
    }

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("title", "%"), ("remark", "%"), ("created", "[date]"), ("updated", "[date]"), ("duration", "[]")]
    }
}
