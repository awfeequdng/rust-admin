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

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .is_numeric("parent_id", "上级菜单编号必须是有效的数字")
            .string_length("name", "分类名称必须在2-20之间", 2, 20, true)
            .string_limit("url", "链接地址长度不能超过200", 200)
            .is_yes_no("state", "状态值不正确")
            .is_yes_no("is_blank", "是否外链值不正确")
            .is_numeric("seq", "排序必须是有效的数字")
            .validate()
    }
}
