use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct VideoCategories { 
    pub id: usize, //编号
    pub name: String, //名称
    pub remark: String, //备注
}

impl Model for VideoCategories { 
    fn get_table_name() -> &'static str { "admin_roles" }
}

impl ModelBackend for VideoCategories { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
    ]);
}
