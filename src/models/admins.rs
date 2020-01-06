use fluffy::{model::Model};
use super::Backend;

pub struct Admins { 
    pub id: usize, //编号
    pub name: String, //用户名称
    pub password: String, //密码
    pub last_ip: String, //最后登录ip
    pub state: u32, //状态, 是否可用, 0: 不可用, 1:可用
    pub login_count: u32, //登录次数
    pub last_login: u32, //最后登录时间
    pub created: u32, //添加时间
    pub updated: u32, //更新时间
}

impl Backend for Admins { 

    type  M = Self;

    fn get_headers() -> Vec<&str> { 
        vec![
            "用户名称",
            "状态",
            "状态",
            "加入时间",
            "最后更新"
        ]
    }

    fn get_records() -> Vec<Self::M> { 
        vec![]
    }
}
