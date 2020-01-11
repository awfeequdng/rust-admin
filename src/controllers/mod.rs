use std::collections::HashMap;
use fluffy::{ tmpl::Tpl, response, model::Model, model::Db, data_set::DataSet, db, };
use crate::models::ModelBackend;
use actix_web::{HttpResponse, web};
use crate::caches;

pub trait Controller { 

    type M: ModelBackend;

    fn get_controller_name() -> &'static str;
    
    /// 主頁
    fn index(tpl: Tpl) -> HttpResponse { 
        let controller_name = Self::get_controller_name(); //控制器名称
        let info = Self::M::get_records();
        let data = tmpl_data![
            "action_name" => &"index",
            "controller_name" => &controller_name,
            "records" => &info.records,
            "pager" => &info.pager,
        ];
        let view_file = &format!("{}/index.html", controller_name);
        render!(tpl, view_file, &data)
    }

    /// 編輯
    fn edit(tpl: Tpl) -> HttpResponse { 
        let controller_name = Self::get_controller_name(); //控制器名称
        let data = tmpl_data![
            "controller_name" => &controller_name,
        ];
        let view_file = &format!("{}/edit.html", controller_name);
        render!(tpl, view_file, &data)
    }
    
    /// 添加
    fn create(post: web::Form<HashMap<String, String>>) -> HttpResponse { 
        let table_name = Self::M::get_table_name();
        let table_fields = caches::TABLE_FIELDS.lock().unwrap();
        let post_fields = post.into_inner();
        let checked_fields = Db::check_fields(table_name, &table_fields, post_fields, false); //經過檢驗之後的數據
        let mut data = DataSet::create();
        for (k, v) in &checked_fields { 
            data.set(k, v);
        }
        let mut conn = db::get_conn();
        let id = Self::M::create(&mut conn, &data);
        if id > 0 { 
            return response::ok();
        } 
        response::error("增加記錄失敗")
    }
    
    /// 修改
    fn update() { 

    }
    
    /// 刪除
    fn delete() { 

    }
}

pub mod index;
pub mod admins;
pub mod admin_roles;
pub mod menus;

//pub use index::Index;
//pub use admins::Admins;
