use fluffy::{DbRow, model::Model, db,};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize)]
pub struct Menus { 
    pub id: usize, //编号
    pub parent_id: usize, //上级编号
    pub name: String, //菜单名称
    pub level_id: usize, //菜单级别
    pub state: u32, //状态
    pub is_blank: u32, //是否新窗口
    pub url: String, //链接地址
    pub seq: isize, //排序
}

#[derive(Default, Debug, Serialize)]
pub struct SubMenu { 
    pub id: usize,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Serialize)]
pub struct MainMenu { 
    pub id: usize,
    pub name: String,
    pub menus: Vec<SubMenu>,
}

impl Model for Menus { 
    fn get_table_name() -> &'static str { "menus" }
}

impl ModelBackend for Menus { 

    type M = Self;
    
    get_fields!(Self, [
        parent_id => usize,
        name => String,
        level_id => usize,
        state => u32,
        is_blank => u32,
        url => String,
        seq => isize,
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

impl Menus { 

    pub fn get_related() -> Vec<MainMenu> { 
        let mut main_menus = vec![];
        let mut conn = db::get_conn();
        let query = query![ fields => "id, name, url", ];
        let cond = cond![ "level_id" => "0", ];
        let rs_main = Menus::fetch_rows(&mut conn, &query, Some(&cond));
        for r_main in rs_main { 
            let (id, name, _): (usize, String, String) = from_row!(r_main);
            let mut menus: Vec<SubMenu> = vec![];
            let cond_sub = cond!["parent_id" => &id, ];
            let rs_subs = Menus::fetch_rows(&mut conn, &query, Some(&cond_sub));
            for r_sub in rs_subs { 
                let (sub_id, sub_name, sub_url): (usize, String, String) = from_row!(r_sub);
                menus.push(SubMenu{ id: sub_id, name: sub_name, url: sub_url, });
            }
            main_menus.push(MainMenu{ id, name, menus, });
        }
        main_menus
    }

    pub fn get_role_menus_by_id(role_id: usize) -> Vec<MainMenu> { 
        let mut main_menus = vec![];
        let sql_main = format!("SELECT id, name FROM menus WHERE id IN (SELECT parent_id FROM menus INNER JOIN admin_roles ON menus.id IN (admin_roles.menu_ids) AND admin_roles.id = {})", role_id);
        let mut conn = db::get_conn();
        let rows = Self::query(&mut conn, &sql_main);
        for r in rows { 
            let mut menus = vec![];
            let (main_id, main_name): (usize, String) = from_row!(r);
            let sql_sub = format!("SELECT id, name, url FROM menus INNER JOIN admin_roles ON menus.id IN (admin_roes.menu_ids) AND parent_id = {}", main_id);
            let subs = Self::query(&mut conn, sql_sub.as_str());
            for s in subs { 
                let (id, name, url): (usize, String, String) = from_row!(s);
                menus.push(SubMenu{id, name, url});
            }
            main_menus.push(MainMenu{id: main_id, name: main_name, menus});
        }
        main_menus
    }

    pub fn get_role_menus() -> HashMap<usize, Vec<MainMenu>> { 
        let sql = "SELECT id FROM admin_roles";
        let mut conn = db::get_conn();
        let rows = Self::query(&mut conn, &sql);
        let mut role_menus = HashMap::new();
        for r in rows { 
            let role_id: usize = from_row!(r);
            let menus = Self::get_role_menus_by_id(role_id);
            role_menus.insert(role_id, menus);
        }
        role_menus
    }
}
