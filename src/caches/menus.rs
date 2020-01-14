use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::Menus;

lazy_static! {
    pub static ref MENUS: Mutex<HashMap<usize, String>> = {
        let fields = "id, name";
        let mut conn = db::get_conn();
        let cond = cond!["parent_id" => &"0",];
        let query = query![fields => &fields,];
        let rs = Menus::fetch_rows(&mut conn, &query, Some(&cond));
        let mut menus: HashMap<usize, String> = HashMap::new();
        for r in rs { 
            let (id, name): (usize, String) = from_row!(r);
            menus.insert(id, name);
        }
        Mutex::new(menus)
    };
}

lazy_static! { 
    pub static ref BREADS: Mutex<HashMap<String, String>> = { 
        let menus = Menus::get_related();
        let mut breads: HashMap<String, String> = HashMap::new();
        for menu in &menus { 
            for sub in &menu.menus { 
                let bread = format!("<a href='#'>{}</a> <a href='#'><cite>{}</cite></a>", menu.name, sub.name);
                breads.insert(sub.url.to_owned(), bread);
            }
        }
        Mutex::new(breads)
    };
}

pub const MENU_LEVELS: [&'static str; 2] = ["一级菜单", "二级菜单"];
