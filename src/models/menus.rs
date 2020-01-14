use fluffy::{DbRow, model::Model, db,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct Menus { 
    pub id: usize, //编号
    pub parent_id: usize, //上级编号
    pub name: String, //菜单名称
    pub level_id: String, //菜单级别
    pub state: u32, //状态
    pub is_blank: u32, //是否新窗口
    pub url: String, //链接地址
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
        level_id => String,
        state => u32,
        is_blank => u32,
        url => String,
    ]);
}

impl Menus { 

    pub fn get_related() -> Vec<MainMenu> { 
        let mut main_menus: Vec<MainMenu> = vec![];
        let mut conn = db::get_conn();
        let query = query![ fields => "id, name, url", ];
        let cond = cond![ "level_id" => "1", ];
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
}
