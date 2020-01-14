use fluffy::{DbRow, model::Model,};
use serde_derive::{Serialize};
use std::collections::HashMap;
use super::ModelBackend;
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct WatchRecords { 
    pub id: usize,
    pub video_id: usize,
    pub user_id: usize,
    pub user_name: String,
    pub created: u32,
}

impl Model for WatchRecords { 
    fn get_table_name() -> &'static str { "watch_records" }
}

impl ModelBackend for WatchRecords { 

    type M = Self;

    get_fields!(Self, [
        video_id => usize,
        user_id => usize,
        user_name => String,
        created => u32,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .length("name", "分类名称必须在2-20之间", 2, 20, true)
            .validate()
    }
}
