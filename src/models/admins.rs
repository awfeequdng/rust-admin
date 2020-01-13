use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct Admins { 
    pub id: usize, //编号
    pub name: String, //用户名称
    pub last_ip: String, //最后登录ip
    pub state: u32, //状态, 是否可用, 0: 不可用, 1:可用
    pub login_count: u32, //登录次数
    pub last_login: u32, //最后登录时间
    pub created: u32, //添加时间
    pub updated: u32, //更新时间
}

impl Model for Admins { 
    fn get_table_name() -> &'static str { "admins" }
}

impl ModelBackend<Admins> for Admins { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        last_ip => String,
        state => u32,
        login_count => u32,
        last_login => u32,
        created => u32,
        updated => u32,
    ]);

}
