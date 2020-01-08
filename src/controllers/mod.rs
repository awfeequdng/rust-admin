use fluffy::{
    tmpl::Tpl,
};
use crate::models::ModelBackend;
use actix_web::{HttpResponse};

pub trait Controller { 

    type M: ModelBackend;

    fn get_controller_name() -> &'static str { 
        let paths: Vec<&'static str> = std::any::type_name::<Self>().rsplit("::").collect();
        paths[0]
    }
    
    /// 主頁
    fn index(tpl: Tpl) -> HttpResponse { 
        let controller_name = Self::get_controller_name(); //控制器名称
        let info = Self::M::get_records();
        let data = tmpl_data![
            "action_name" => &"index",
            "controller_name" => &controller_name,
            "records" => &info.records,
        ];
        let view_file =  &format!("{}/index.html", controller_name);
        render!(tpl, view_file, &data)
    }
    
    /// 添加
    fn create() { 

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
