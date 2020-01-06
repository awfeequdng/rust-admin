use actix_web::{HttpResponse, get};
use fluffy::{tmpl::Tpl};

/// 后台首页登录
#[get("/")]
pub async fn index(tpl: Tpl) -> HttpResponse { 
    render!(tpl, "index")
}

/// 后台管理主界面
#[get("/index/manage")]
pub async fn manage(tpl: Tpl) -> HttpResponse { 
    render!(tpl, "manage")
}

#[get("/index/right")]
pub async fn right(tpl: Tpl) -> HttpResponse { 
    render!(tpl, "right")
}

