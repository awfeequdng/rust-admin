use actix_web::{HttpResponse, get, Responder};
use fluffy::{tmpl::Tpl};

#[get("/hello")]
pub async fn hello() -> impl Responder { 
    "hello world"
}

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

