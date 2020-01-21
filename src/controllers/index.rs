use std::collections::HashMap;
use actix_web::{HttpResponse, web::Form,};
use fluffy::{tmpl::Tpl, db, model::Model, datetime, utils, random, response,};
use crate::models::{Index as ThisModel, Admins};
use std::env;
use actix_session::{Session};
use crate::common::Acl;
use crate::config::{LOGIN_ERROR_MAX, LOGIN_LOCKED_TIME};
use crate::caches::admin_roles::ROLE_MENUS;

//struct Login { 
//    pub ip: String,
//    pub locked_time: usize,
//}
//
//lazy_static! { 
//    pub static ref LOGIN_INFO: HashMap<String, Login> = {
//        HashMap::new()
//    };
//}

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

    /// 错误页面
    pub async fn error(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "index/error.html")
    }

    /// 用户登录
    pub async fn login(session: Session, post: Form<HashMap<String, String>>) -> HttpResponse { 
        //let s1 = random::rand_str(32); //用于生成默认的用户密码
        //let p1 = utils::get_password("qwe123", &s1); //默认密码qwe123
        //println!("UPDATE admins SET secret = '{}', password = '{}' WHERE id = 1", s1, p1);
        //session.remove("locked_time");
        //session.remove("failure_count");
        if let Ok(locked_time) = session.get::<usize>("locked_time") {  //如果session中记录的有锁定时间
            if let Some(n) = locked_time { 
                if (datetime::timestamp() as usize) - n < LOGIN_LOCKED_TIME { 
                    return response::error("登录次败次数过多,请2小时后再次尝试");
                }
            }
        } 

        let mut failure_count = 0_usize; //登录失败次数
        if let Ok(failure) = session.get::<usize>("failure_count") {  //检测登录失败次数
            if let Some(n) = failure { 
                failure_count = n; //已经失败的次数
                if n > LOGIN_ERROR_MAX { 
                    if let Err(message) = session.set::<usize>("locked_time", datetime::timestamp() as usize) { 
                        return response::error(&message.to_string());
                    }
                    return response::error("失败次数过多, 请稍后重试");
                }
            }
        } else { 
            if let Err(message) = session.set::<usize>("failure_count", failure_count) { 
                return response::error(&message.to_string());
            }
        } //设置登录失败次数的默认值

        if let Err(message) = ThisModel::check_login(&post) {  //如果校验数据出现错误
            return response::error(&message);
        }
        
        let name = post.get("username").unwrap();
        let password_ori = post.get("password").unwrap();
        let query = query![fields => "id, password, secret, login_count, role_id",];
        let cond = cond!["name" => &name,];
        let mut conn = db::get_conn();
        if let Some(row) = Admins::fetch_row(&mut conn, &query, Some(&cond)) { 
            let (id, password, secret, login_count, role_id): (usize, String, String, usize, usize) = from_row!(row);
            let password_enc = utils::get_password(password_ori, &secret);
            if password_enc != password {  //对比加密之后的密码是否一致
                session.set::<usize>("failure_count", failure_count + 1).unwrap();
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
            if  Admins::update(&mut conn, &data, &cond) == 0 { 
                session.set::<usize>("failure_count", failure_count + 1).unwrap();
                return response::error("更新用户信息失败");
            }

            session.remove("failure_count"); //清空失败次数
            session.remove("locked_time"); //清空锁定时间
            session.set::<usize>("user_id", id).unwrap();
            session.set::<String>("user_name", name.to_owned()).unwrap();
            session.set::<usize>("role_id", role_id).unwrap();
            return response::ok();
        } 
        session.set::<usize>("failure_count", failure_count + 1).unwrap();
        response::error("用户名称或密码错误")
    }

    /// 后台管理主界面
    pub async fn manage(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error");
        }
        let mut role_id = 0;
        if let Ok(v) = session.get::<usize>("role_id") { 
            if let Some(n) = v { 
                role_id = n;
            }
        }
        let role_menus = &*ROLE_MENUS.lock().unwrap();
        let menus = role_menus.get(&role_id);
        let data = tmpl_data![
            "menus" => &menus,
        ];
        render!(tpl, "index/manage.html", &data)
    }
    
    /// 后台进入之后的首页
    pub async fn right(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error");
        }
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

