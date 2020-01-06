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

pub trait ModelBackend<M: Default> { 

    type This : Model<M>;
    
    /// 得到後臺可用操作選項
    fn get_option() -> RowOption { 
        RowOption::default()
    }

    /// 得到頭部信息
    fn get_headers() -> Vec<&'static str>;

    /// 得到要從資料庫中提取的列頭
    fn get_fields() -> &'static str;

    /// 得到單條記錄
    fn get_record(_: DbRow) -> M;

    /// 得到所有記錄
    fn get_records() -> Vec<M> { 
        let fields = Self::get_fields();
        let query = query![
            fields => fields,
        ];
        let mut conn = db::get_conn();
        let rows = Self::This::fetch_rows(&mut conn, &query, None);
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
