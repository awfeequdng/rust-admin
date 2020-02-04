use crate::models::Videos as ThisModel;
use super::Controller;
use crate::config;

pub struct Videos { }

impl Controller for Videos { 

    type M = ThisModel;

    fn edit_after(data: &mut tera::Context) {
        let setting = &*config::SETTING;
        let info = &setting.oss;
        data.insert("bucket", &info.bucket);
        data.insert("region",  &info.region);
        data.insert("end_point", &info.end_point);
    }

    fn get_query_cond() -> Vec<(&'static str, &'static str)> { 
        vec![("title", "%"), ("remark", "%"), ("created", "[date]"), ("updated", "[date]"), ("duration", "[]")]
    }
}
