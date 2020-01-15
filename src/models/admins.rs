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
    pub role_id: usize,
}

impl Model for Admins { 
    fn get_table_name() -> &'static str { "admins" }
}

impl ModelBackend for Admins { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        last_ip => String,
        role_id => usize,
        state => u32,
        login_count => u32,
        last_login => u32,
        created => u32,
        updated => u32,
    ]);

}
