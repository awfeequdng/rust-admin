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
        let info = Self::M::get_records();
        let data = tmpl_data![
            "records" => &info.records,
            "headers" => &info.headers,
        ];
        let paths: Vec<&str> = Self::name().rsplit("::").collect();
        let view_file =  &format!("{}/index.html", paths[0].to_lowercase());
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

//pub use index::Index;
//pub use admins::Admins;
