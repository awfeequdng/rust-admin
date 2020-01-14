use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct Videos { 
    pub id: usize,
    pub title: String,
    pub remark: String,
    pub cover_image: String,
    pub seq: isize,
    pub duration: u32, 
    pub state: u32,
    pub created: u32, 
    pub updated: u32,
    pub content: String,
}

impl Model for Videos { 
    fn get_table_name() -> &'static str { "videos" }
}

impl ModelBackend for Videos { 

    type M = Self;

    get_fields!(Self, [
        title => String,
        remark => String,
        cover_image => String,
        duration => u32,
        seq => isize,
        state => u32,
        created => u32,
        updated => u32,
    ]);
}
