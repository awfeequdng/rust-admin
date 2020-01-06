#[macro_use] extern crate fluffy;

use actix_web::{App, HttpServer, middleware};
use tera::Tera;

mod config;
mod models;
mod controllers;

use controllers::{index};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //地址/端口
    let host_port = &format!("{}:{}", config::BIND_HOST, config::BIND_PORT);

    HttpServer::new(|| {

        let tpl = tmpl!("/templates/**/*");
        
        App::new()
            .data(tpl)
            .wrap(middleware::Logger::default()) // enable logger
            .service(index::index)
            .service(index::manage)
            .service(index::right)
    })
    .bind(host_port)?
    .run()
    .await
}
