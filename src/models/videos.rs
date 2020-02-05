use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;
use std::collections::HashMap;

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
    pub category_id: usize,
    pub tag_ids: String,
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
        category_id => usize,
        tag_ids => String,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("title", "标题长度必须在2-30之间", 2, 30, true)
            .string_limit("remark", "备注长度不能超过200", 200)
            .string_length("cover_image", "封面地址长度必须在2-200之间", 2, 200, true)
            .is_unsigned("duration", "时长必须是正确的数字")
            .is_numeric("seq", "排序必须是有效的数字")
            .is_state("state", "必须选择正确的状态值")
            .validate()
    }
}
