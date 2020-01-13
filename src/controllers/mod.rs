use std::collections::HashMap;
use std::fmt::Debug;
use fluffy::{ tmpl::Tpl, response, model::Model, model::Db, data_set::DataSet, db, };
use crate::models::ModelBackend;
use actix_web::{HttpResponse, web::{Path, Form}};
use crate::caches;
use serde::ser::{Serialize};

#[macro_export]
macro_rules! row_for_update {
    ($struct: ident, $id: expr, [$($field: ident => $type: ident,)+]) => ({
        let mut row = $struct::default();
        let fields = concat!("id", $(",", stringify!($field)),+);
        let query = query![fields => &fields, ];
        let cond = cond!["id" => &$id, ];
        let mut conn = fluffy::db::get_conn();
        if let Some(r) = $struct::fetch_row(&mut conn, &query, Some(&cond)) { 
            let (id, $($field),+): (usize, $($type),+) = from_row!(r);
            row.id = id;
            $(row.$field = $field;)+
        }
        row
    })
}

pub trait Controller { 

    type M: ModelBackend + Default + Serialize + Debug;

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

    fn edit_for_create() -> Self::M { 
        Self::M::default()
    }

    fn edit_for_update(_id: usize) -> Self::M { 
        Self::M::default()
    }

    /// 編輯
    fn edit(info: Path<usize>, tpl: Tpl) -> HttpResponse { 
        let controller_name = Self::get_controller_name(); //控制器名称
        let id = info.into_inner();
        let is_update = id > 0;
        let row = if is_update { Self::edit_for_update(id) } else { Self::edit_for_create() };
        println!("row = {:?}", row);
        let button_text = if is_update { "保存记录" } else { "添加记录" };
        let data = tmpl_data![
            "controller_name" => controller_name,
            "row" => &row,
            "button_text" => button_text,
            "id" => &id,
        ];
        let view_file = &format!("{}/edit.html", controller_name);
        render!(tpl, view_file, &data)
    }

    /// 編輯
    fn save(info: Path<usize>, post: Form<HashMap<String, String>>) -> HttpResponse { 
        let id = info.into_inner();
        if id == 0 { Self::save_for_create(post) } else { Self::save_for_update(id, post) }
    }

    /// 添加
    fn save_for_create(post: Form<HashMap<String, String>>) -> HttpResponse { 
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
    fn save_for_update(id: usize, post: Form<HashMap<String, String>>) -> HttpResponse { 
        let table_name = Self::M::get_table_name();
        let table_fields = caches::TABLE_FIELDS.lock().unwrap();
        let post_fields = post.into_inner();
        let checked_fields = Db::check_fields(table_name, &table_fields, post_fields, true); //經過檢驗之後的數據
        let mut data = DataSet::update();
        for (k, v) in &checked_fields { 
            data.set(k, v);
        }
        let mut conn = db::get_conn();
        let cond = cond![ "id" => &id, ];
        let id = Self::M::update(&mut conn, &data, &cond);
        if id > 0 { 
            return response::ok();
        } 
        response::error("修改記錄失敗")
    }
    
    /// 刪除
    fn delete(ids: Path<Vec<usize>>) -> HttpResponse { 
        println!("ids = {:?}", ids);
        response::ok()
    }
}

pub mod index;
pub mod admins;
pub mod admin_roles;
pub mod menus;

//pub use index::Index;
//pub use admins::Admins;
