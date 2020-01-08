use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct Menus { 
    pub id: usize, //编号
    pub parent_id: usize, //上级编号
    pub name: String, //菜单名称
    pub level_id: u32, //菜单级别
    pub state: u32, //状态
    pub is_blank: u32, //是否新窗口
    pub url: String, //链接地址
}

type Row = (usize, usize, String, u32, u32, u32, String);

impl Model for Menus { 
    fn get_table_name() -> &'static str { "menus" }
}

impl ModelBackend for Menus { 

    type M = Self;

    //fn get_headers() -> Vec<&'static str> { 
    //    vec!["编号", "上级编号", "名称", "菜单级别", "状态", "链接地址", "是否外链"]
    //}

    fn get_fields() -> &'static str { 
        "id, parent_id, name, level_id, state, is_blank, url"
    }

    fn get_record(r: DbRow) -> Self { 
        let (id, parent_id, name, level_id, state, is_blank, url): Row = from_row!(r);
        Self { id, parent_id, name, level_id, state, is_blank, url }
    }
}
