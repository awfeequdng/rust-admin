use fluffy::{model::{Model}, db};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::AdminRoles;

lazy_static! { 
    pub static ref ADMIN_ROLES: Mutex<HashMap<String, String>> = {
        let mut conn = db::get_conn();
        let query = query![
            fields => "id, name",
        ];
        let mut roles: HashMap<String, String> = HashMap::new();
        let rs = AdminRoles::fetch_rows(&mut conn, &query, None);
        for r in rs { 
            let (id, name): (usize, String) = from_row!(r);
            roles.insert(id.to_string(), name);
        }
        Mutex::new(roles)
    };
}
