use std::collections::HashMap;
use actix_web::{HttpResponse, web::Form};
use fluffy::{tmpl::Tpl, db, model::Model, datetime, utils, random, response, };
use crate::models::{Menus, Users, Index as ThisModel};
use std::env;
use sys_info;

pub struct Index {}

impl Index { 
    
    /// 测试请求
    pub async fn test() -> &'static str { 
        "hello, there"
    }

    /// 后台首页登录
    pub async fn index(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "index/index.html")
    }

    /// 用户登录
    pub async fn login(post: Form<HashMap<String, String>>) -> HttpResponse { 
        if let Err(message) = ThisModel::check_login(&post) {  //如果校验数据出现错误
            return response::error(&message);
        }
        
        let name = post.get("name").unwrap();
        let password_ori = post.get("password").unwrap();
        let query = query![fields => "id, password, secret, login_count",];
        let cond = cond!["id" => &name,];
        let mut conn = db::get_conn();
        if let Some(row) = Users::fetch_row(&mut conn, &query, Some(&cond)) { 
            let (id, password, secret, login_count): (usize, String, String, usize) = from_row!(row);
            let password_enc = utils::get_password(password_ori, &secret);
            if password_enc != password {  //对比加密之后的密码是否一致
                return response::error("用户名称或密码错误");
            }

            let secret_new = random::rand_str(32);
            let password_new = utils::get_password(&password_ori, &secret_new);
            let now = datetime::timestamp();
            let data = update_row![
                "login_count" => login_count + 1,
                "last_login" => &now,
                "updated" => &now,
                "secret" => &secret_new,
                "password" => &password_new,
            ];
            let cond = cond!["id" => id,];
            if  Users::update(&mut conn, &data, &cond) == 0 { 
                return response::error("更新用户信息失败");
            }

            return response::ok();
        } 
        response::error("用户名称或密码错误")
    }

    /// 后台管理主界面
    pub async fn manage(tpl: Tpl) -> HttpResponse { 
        let related_menus = Menus::get_related();
        let data = tmpl_data![
            "menus" => &related_menus,
        ];
        render!(tpl, "index/manage.html", &data)
    }
    
    /// 后台进入之后的首页
    pub async fn right(tpl: Tpl) -> HttpResponse { 
        let mut data = tmpl_data![];
        // 当前目录
        let current_dir = if let Ok(v) = env::current_dir() { 
            if let Some(p) = v.to_str() { p.to_owned() } else { "".to_owned() }
        } else { "".to_owned() };
        data.insert("current_dir", &current_dir);
        // cpu信息
        let cpu_num = if let Ok(n) = sys_info::cpu_num() { n } else { 0 };
        data.insert("cpu_num", &cpu_num);
        // cpu频率
        let cpu_speed = if let Ok(n) = sys_info::cpu_speed() { n } else { 0 };
        data.insert("cpu_speed", &cpu_speed);
        // 硬盘信息
        let disk_info = if let Ok(v) = sys_info::disk_info() { v } else { sys_info::DiskInfo{ total: 0, free: 0 } };
        let disk_info_total = format!("{:.2}", disk_info.total as f64 / (1024. * 1024.));
        let disk_info_free = format!("{:.2}", disk_info.free as f64 / (1024. * 1024.));
        data.insert("disk_info_total", &disk_info_total);
        data.insert("disk_info_free", &disk_info_free);
        // 启动时间
        let boot_time_secs = if let Ok(n) = sys_info::boottime() { n.tv_sec as isize } else { 0 };
        let boot_time = format!("{} 天 {} 时 {} 分", boot_time_secs / 86400 , (boot_time_secs % 86400) / 3600, (boot_time_secs % 3600) / 60);
        data.insert("boot_time", &boot_time);
        // 主机名称
        let host_name = if let Ok(v) = sys_info::hostname() { v } else { "".to_owned() };
        data.insert("host_name", &host_name);
        // 内存信息
        let mem_info = if let Ok(v) = sys_info::mem_info() { (v.total, v.free, v.avail) } else { (0, 0, 0) };
        let mem_info_total = format!("{:.2}", mem_info.0 as f64 / (1024. * 1024.));
        let mem_info_free = format!("{:.2}", mem_info.1 as f64 / (1024. * 1024.));
        let mem_info_avail = format!("{:.2}", mem_info.2 as f64 / (1024. * 1024.));
        data.insert("mem_info_total", &mem_info_total);
        data.insert("mem_info_free", &mem_info_free);
        data.insert("mem_info_avail", &mem_info_avail);
        // 操作系统
        let os_type = if let Ok(v) = sys_info::os_type() { v } else { "".to_owned()  };
        data.insert("os_type", &os_type);
        // 操作系统版本
        let os_version = if let Ok(v) = sys_info::os_release() { v } else { "".to_owned() } ;
        data.insert("os_version", &os_version);
        // 进程数量
        let process_count = if let Ok(n) = sys_info::proc_total() { n } else { 0 };
        data.insert("process_count", &process_count);
        // 负载
        let avg = if let Ok(v) = sys_info::loadavg() { (v.one, v.five, v.fifteen) } else { (0., 0., 0.) };
        data.insert("avg_1", &avg.0);
        data.insert("avg_2", &avg.1);
        data.insert("avg_3", &avg.2);
        let my_version = env!("CARGO_PKG_VERSION");
        data.insert("my_version", &my_version);
        render!(tpl, "index/right.html", &data)
    }
}

