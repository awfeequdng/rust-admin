use fluffy::{
    tmpl::Tpl,
};
use crate::models::ModelBackend;
use actix_web::{HttpResponse};

pub trait Controller { 

    type M: ModelBackend;

    fn name() -> &'static str { 
        std::any::type_name::<Self>()
    }
    
    /// 主頁
    fn index(tpl: Tpl) -> HttpResponse { 
        let paths: Vec<&str> = Self::name().rsplit("::").collect();
        let controller_name = paths[0].to_lowercase(); //控制器名称
        let info = Self::M::get_records();
        let data = tmpl_data![
            "action_name" => &"index",
            "controller_name" => &controller_name,
            "records" => &info.records,
            "headers" => &info.headers,
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
