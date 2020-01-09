#[macro_use] extern crate fluffy;
#[macro_use] extern crate lazy_static;

use actix_web::{App, HttpServer, middleware};
use fluffy::{db};

mod config;
mod models;
mod controllers;
mod caches;

use controllers::{
    Controller, 
    index::Index, 
    admins::Admins,
    admin_roles::AdminRoles,
    menus::Menus,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let conn_string = config::get_conn_string();
    db::init_connections(&conn_string); //資料庫初始化
    let host_port = &format!("{}:{}", config::BIND_HOST, config::BIND_PORT); //地址/端口
    println!("Started At: {}", host_port);

    //let table_fields = caches::TABLE_FIELDS.lock().unwrap();
    HttpServer::new(|| {

        let tpl = tmpl!("/templates/**/*"); //模板引擎
        
        App::new()
            .data(tpl)
            .wrap(middleware::Logger::default()) // enable logger
            .service(get!("/", Index::index))
            .service(get!("/index/manage", Index::manage))
            .service(get!("/index/right", Index::right))
            .service(get!("/index/right", Index::right))
            .service(get!("/admins", Admins::index))
            .service(get!("/admin_roles", AdminRoles::index))
            .service(get!("/menus", Menus::index))
    })
    .bind(host_port)?
    .run()
    .await
}
