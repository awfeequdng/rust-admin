use std::default::Default;
use fluffy::{
    db, DbRow,
    model::Model, 
    query_builder::QueryBuilder,
};

/// 數據列表操作選項
#[derive(Default)]
pub struct RowOption { 
    pub create: bool, //是否允許添加記錄
    pub update: bool, //是否允許修改記錄
    pub delete: bool, //是否允許刪除記錄
    pub dleete_all: bool, //是否允許一次刪除全部記錄
}

pub trait Backend<M> { 
    
    fn get_option() -> RowOption { 
        RowOption::default()
    }

    fn get_headers() -> Vec<&'static str>;

    fn get_fields() -> &'static str;

    fn get_record(_: DbRow) -> M;

    fn get_records() -> Vec<M> { 
        let fields = Self::get_fields();
        let query = query![
            fields => fields,
        ];
        let mut conn = db::get_conn();
        let rows = Admins::fetch_rows(&mut conn, &query, None);
        let mut rs: Vec<M> = vec![];
        for r in rows { 
            rs.push(Self::get_record(r));
        }
        rs
    }
}

/// 後臺用戶
mod admins;

pub use admins::Admins;
