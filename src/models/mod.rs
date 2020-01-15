use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;
use fluffy::{
    db, DbRow, Pager,
    model::Model, 
};
use serde::ser::Serialize;

#[derive(Debug, Default)]
pub struct DataGrid<M: Model + Serialize> { 
    pub records: Vec<M>,
    pub pager: Pager,
}

#[macro_export]
macro_rules! get_fields {
    ($struct: ident, [$($field: ident => $type: ident,)+]) => {

        fn get_fields() -> &'static str { 
            concat!("id", $(",", stringify!($field)),+)
        }

        fn get_record(r: DbRow) -> Self { 
            let mut row = Self::default();
            let (id, $($field),+): (usize, $($type),+) = from_row!(r);
            row.id = id;
            $(row.$field = $field;)+
            row
        }
    }
}


pub trait ModelBackend: Model { 

    type M: Model + Serialize + Default + Debug;

    /// 得到要從資料庫中提取的列頭
    fn get_fields() -> &'static str;

    /// 得到單條記錄
    fn get_record(_: DbRow) -> Self::M;

    fn get_default() -> Self::M { 
        Self::M::default()
    }

    fn validate(_data: &HashMap<String, String>) -> Result<(), String>{ 
        Ok(())
    }

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
mod users;
mod videos;
mod video_categories;
mod video_replies;
mod video_tags;
mod user_levels;
mod watch_records;
mod ads;

pub use admins::Admins;
pub use menus::Menus;
pub use admin_roles::AdminRoles;
pub use videos::Videos;
pub use video_categories::VideoCategories;
pub use video_replies::VideoReplies;
pub use users::Users;
pub use user_levels::UserLevels;
pub use video_tags::VideoTags;
pub use watch_records::WatchRecords;
pub use ads::Ads;
