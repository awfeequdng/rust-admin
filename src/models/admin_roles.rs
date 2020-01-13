use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct AdminRoles { 
    pub id: usize, //编号
    pub name: String, //名称
    pub remark: String, //备注
}

impl Model for AdminRoles { 
    fn get_table_name() -> &'static str { "admin_roles" }
}

impl ModelBackend for AdminRoles { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
    ]);
}
