use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use std::collections::HashMap;
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct Ads { 
    pub id: usize,
    pub name: String,
    pub remark: String, 
    pub image: String,
    pub page_id: u32,
    pub position_id: u32,
    pub url: String,
    pub is_blank: u32,
    pub seq: isize,
}

impl Model for Ads { 
    fn get_table_name() -> &'static str { "ads" }
}

impl ModelBackend for Ads { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
        image => String,
        page_id => u32,
        position_id => u32,
        url => String,
        is_blank => u32,
        seq => isize,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .length("name", "分类名称必须在2-20之间", 2, 20, true)
            .validate()
    }
}
