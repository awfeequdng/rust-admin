use std::default::Default;
use fluffy::{
    db, DbRow, Pager,
    model::Model, 
    query_builder::QueryBuilder,
};
use serde::ser::Serialize;

#[derive(Debug, Default)]
pub struct DataGrid<M: Model + Serialize> { 
    pub records: Vec<M>,
    pub pager: Pager,
}

pub trait ModelBackend: Model { 

    type M: Model + Serialize;

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
            records: rs,
            pager: pager,
        }
    }
}

/// 後臺用戶
mod admins;
mod menus;
mod admin_roles;

pub use admins::Admins;
pub use menus::Menus;
pub use admin_roles::AdminRoles;
