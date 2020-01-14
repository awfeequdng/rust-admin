use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::Menus;

lazy_static! {
    pub static ref MENUS: Mutex<HashMap<usize, String>> = {
        let fields = "id, name";
        let mut conn = db::get_conn();
        let query = query![fields => &fields,];
        let rs = Menus::fetch_rows(&mut conn, &query, None);
        let mut menus: HashMap<usize, String> = HashMap::new();
        for r in rs { 
            let (id, name): (usize, String) = from_row!(r);
            menus.insert(id, name);
        }
        Mutex::new(menus)
    };
}

pub const MENU_LEVELS: [&'static str; 2] = ["一级菜单", "二级菜单"];
