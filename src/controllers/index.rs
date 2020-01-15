use std::collections::HashMap;
use actix_web::{HttpResponse, web::Form};
use fluffy::{tmpl::Tpl, db, model::Model, datetime, utils, random, response, };
use crate::models::{Menus, Users, Index as ThisModel};

pub struct Index {}

impl Index { 

    /// 后台首页登录
    pub async fn index(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "index/index.html")
    }

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

    pub async fn right(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "index/right.html")
    }
}

