use std::default::Default;
use fluffy::{
    db, DbRow, Pager,
    model::Model, 
    query_builder::QueryBuilder,
};
use serde::ser::Serialize;

/// 數據列表操作選項
#[derive(Debug, Default)]
pub struct RowOption { 
    pub create: bool, //是否允許添加記錄
    pub update: bool, //是否允許修改記錄
    pub delete: bool, //是否允許刪除記錄
    pub dleete_all: bool, //是否允許一次刪除全部記錄
}

#[derive(Debug, Default)]
pub struct DataGrid<M: Model + Serialize> { 
    pub headers: Vec<&'static str>,
    pub records: Vec<M>,
    pub pager: Pager,
}

pub trait ModelBackend: Model { 

    type M: Model + Serialize;

    /// 得到後臺可用操作選項
    fn get_option() -> RowOption { 
        RowOption::default()
    }

    /// 得到頭部信息
    fn get_headers() -> Vec<&'static str>;

    /// 得到要從資料庫中提取的列頭
    fn get_fields() -> &'static str;

    /// 得到單條記錄
    fn get_record(_: DbRow) -> Self::M;

    /// 得到所有記錄-帶分頁信息
    fn get_records() -> DataGrid<Self::M> { 
        let fields = Self::get_fields();
        let query = query![
            fields => fields,
        ];
        let mut conn = db::get_conn();
        let rows = Self::M::fetch_rows(&mut conn, &query, None);
        let mut rs: Vec<Self::M> = vec![];
        for r in rows { 
            rs.push(Self::get_record(r));
        }
        let pager = Self::M::get_pager(&mut conn, &query, None);
        DataGrid { 
            headers: Self::get_headers(),
            records: rs,
            pager: pager,
        }
    }
}

/// 後臺用戶
mod admins;

pub use admins::Admins;
