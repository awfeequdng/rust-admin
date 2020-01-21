use fluffy::{model::{Model}, db};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::{AdminRoles, Menus, MainMenu};

lazy_static! { 
    pub static ref ADMIN_ROLES: Mutex<HashMap<usize, String>> = {
        let mut conn = db::get_conn();
        let query = query![
            fields => "id, name",
        ];
        let mut roles = HashMap::new();
        let rs = AdminRoles::fetch_rows(&mut conn, &query, None);
        for r in rs { 
            let (id, name): (usize, String) = from_row!(r);
            roles.insert(id, name);
        }
        Mutex::new(roles)
    };
}

lazy_static! { 
    pub static ref ROLE_MENUS: Mutex<HashMap<usize, Vec<MainMenu>>> = { 
        Mutex::new(Menus::get_role_menus())
    };
}

/// 是否允许访问
pub fn allow_access(role_id: usize, url: &str) -> bool { 
    if url == "" { 
        return false;
    }
    let role_menus = &*ROLE_MENUS.lock().unwrap();
    //let urls = url.split("|").collect::<Vec<&str>>();
    if let Some(menus) = role_menus.get(&role_id) { 
        for menu in menus { 
            for sub in &menu.menus { 
                if sub.url.contains("|") { 
                    let urls = sub.url.split("|").collect::<Vec<&str>>();
                    if urls.contains(&sub.url.as_str()) { 
                        return true;
                    }
                } else if url == sub.url { 
                    return true;
                }
            }
        }
    }
    false
}

/// 刷新缓存
pub fn refresh() { 
    let mut role_menus = ROLE_MENUS.lock().unwrap();
    *role_menus = Menus::get_role_menus();
}
