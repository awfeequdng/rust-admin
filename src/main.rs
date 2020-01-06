#[macro_use] extern crate fluffy;

//use actix_web::{App, HttpServer, middleware};
use actix_web::{App, HttpServer};
use tera::Tera;
use fluffy::{db};

mod config;
mod models;
mod controllers;

use controllers::{index};
use models::{ModelBackend, Admins};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    //std::env::set_var("RUST_LOG", "actix_web=info");
    //env_logger::init();

    db::init_connections(config::MYSQL_CONN_STR); //資料庫初始化
    let host_port = &format!("{}:{}", config::BIND_HOST, config::BIND_PORT); //地址/端口

    let rs = Admins::get_records();
    println!("{:?}", rs);

    HttpServer::new(|| {

        let tpl = tmpl!("/templates/**/*"); //模板引擎
        
        App::new()
            .data(tpl)
            //.wrap(middleware::Logger::default()) // enable logger
            .service(index::hello)
            .service(index::index)
            .service(index::manage)
            .service(index::right)
    })
    .bind(host_port)?
    .run()
    .await
}
