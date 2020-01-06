use fluffy::{
    DbRow,
    model::Model, 
};
use super::ModelBackend;

#[derive(Default, Debug)]
pub struct Admins { 
    pub id: usize, //编号
    pub name: String, //用户名称
    //pub password: String, //密码
    pub last_ip: String, //最后登录ip
    pub state: u32, //状态, 是否可用, 0: 不可用, 1:可用
    pub login_count: u32, //登录次数
    pub last_login: u32, //最后登录时间
    pub created: u32, //添加时间
    pub updated: u32, //更新时间
}

type Row = (usize, String, String, u32, u32, u32, u32, u32);

impl Model<Admins> for Admins { 
    
    fn get_table_name() -> &'static str { 
        "admins"
    }
}

impl ModelBackend<Admins> for Admins { 

    type This = Admins;

    fn get_headers() -> Vec<&'static str> { 
        vec!["編號", "用户名称", "上次登錄IP", "状态", "登錄次數", "最後登錄時間", "加入时间", "最后更新"]
    }

    fn get_fields() -> &'static str { 
        "id, name, last_ip, state, login_count, last_login, created, updated"
    }

    fn get_record(r: DbRow) -> Admins { 
        let (id, name, last_ip, state, login_count, last_login, created, updated): Row = from_row!(r);
        Self { 
            id, name, last_ip, state, login_count, last_login, created, updated,
        }
    }
}
