use fluffy::{model::{Model}, db};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::AdminRoles;

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
