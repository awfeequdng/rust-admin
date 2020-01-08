use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct AdminRoles { 
    pub id: usize, //编号
    pub name: String, //名称
    pub remark: String, //备注
}

type Row = (usize, String, String);

impl Model for AdminRoles { 
    fn get_table_name() -> &'static str { 
        "admin_roles"
    }
}

impl ModelBackend for AdminRoles { 

    type M = Self;

    //fn get_headers() -> Vec<&'static str> { 
    //    vec!["編號", "名称", "备注"]
    //}

    fn get_fields() -> &'static str { 
        "id, name, remark"
    }

    fn get_record(r: DbRow) -> Self { 
        let (id, name, remark): Row = from_row!(r);
        Self { id, name, remark }
    }
}
