use fluffy::{model::Db, db};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::config::{MYSQL_DB_NAME};

lazy_static! { 
    pub static ref TABLE_FIELDS: Mutex<HashMap<String, Vec<String>>> = {
        let mut conn = db::get_conn();
        let table_fields = Db::get_table_fields(&mut conn, MYSQL_DB_NAME);
        Mutex::new(table_fields)
    };
}
