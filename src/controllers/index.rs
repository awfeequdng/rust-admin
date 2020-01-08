use actix_web::{HttpResponse};
use fluffy::{tmpl::Tpl};

pub struct Index {}

impl Index { 

    /// 后台首页登录
    pub async fn index(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "Index/index.html")
    }

    /// 后台管理主界面
    pub async fn manage(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "Index/manage.html")
    }

    pub async fn right(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "Index/right.html")
    }
}

