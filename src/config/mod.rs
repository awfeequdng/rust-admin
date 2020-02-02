use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;
use std::env;

/// 最多允許登錄出錯次數
pub const LOGIN_ERROR_MAX: usize = 1000;

/// 登录失败后锁定时间
pub const LOGIN_LOCKED_TIME: usize = 3600;

/// 绑定主机/端口
#[derive(Deserialize, Default)]
pub struct AppInfo { 
    pub host: String,
    pub port: usize,
}

/// 数据库连接信息
#[derive(Deserialize, Default)]
pub struct DbInfo { 
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: usize,
}

#[derive(Deserialize, Default)]
pub struct OSS { 
    pub access_key_id: String,
    pub access_key_secret: String,
    pub end_point: String,
    pub region: String,
    pub bucket: String,
}

/// 系统配置信息
#[derive(Deserialize, Default)]
pub struct Setting { 
    pub app: AppInfo,
    pub database: DbInfo,
    pub oss: OSS,
}

lazy_static! { 
    pub static ref SETTING: Setting = { 
        let current_dir = env::current_dir().unwrap();
        let current_path = current_dir.to_str().unwrap();
        let toml_file = dbg!(format!("{}/setting.toml", current_path));
        let setting = Setting::default();
        match File::open(&toml_file) { 
            Ok(mut v) => { 
                let mut content = String::new();
                if let Ok(_) = v.read_to_string(&mut content) { 
                    if let Ok(t) = toml::from_str::<Setting>(&content) { t } else { setting }
                } else { setting }
            },
            Err(err) => { 
                println!("读取文件失败: {}", err);
                setting
            }
        }
    };
}

/// 得到数据库连接字符串
pub fn get_conn_string() -> String { 
    let setting = &*SETTING;
    let db = &setting.database;
    format!("mysql://{}:{}@{}:{}/{}", db.user, db.password, db.host, db.port, db.name)
}

/// 获取OSS信息
pub fn get_oss_info<'a>() -> &'a OSS { 
    let setting = &*SETTING;
    &setting.oss
}
