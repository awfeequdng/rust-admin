#[macro_use] extern crate fluffy;
#[macro_use] extern crate lazy_static;

use actix_web::{App, HttpServer, middleware};
use fluffy::{db};

mod config;
mod validations;
mod models;
mod controllers;
mod caches;

use controllers::{
    Controller, 
    index::Index, 
    admins::Admins,
    admin_roles::AdminRoles,
    menus::Menus,
    users::Users,
    videos::Videos,
    video_categories::VideoCategories,
    video_replies::VideoReplies,
};

#[derive(Default, Debug)]
struct Test { 
    id: u32,
    name: String,
}

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
            //后台用户
            .service(get!("/admins", Admins::index))
            .service(get!("/admin_roles/edit/{id}", Admins::edit))
            .service(post!("/admin_roles/save/{id}", Admins::save))
            .service(get!("/admin_roles/delete/{ids}", Admins::delete))
            //角色管理 
            .service(get!("/admin_roles", AdminRoles::index))
            .service(get!("/admin_roles/edit/{id}", AdminRoles::edit))
            .service(post!("/admin_roles/save/{id}", AdminRoles::save))
            .service(get!("/admin_roles/delete/{ids}", AdminRoles::delete))
            //菜单管理
            .service(get!("/menus", Menus::index))
            .service(get!("/menus/edit/{id}", Menus::edit))
            .service(post!("/menus/save/{id}", Menus::save))
            .service(get!("/menus/delete/{ids}", Menus::delete))
            //users
            .service(get!("/users", Users::index))
            .service(get!("/users/edit/{id}", Users::edit))
            .service(post!("/users/save/{id}", Users::save))
            .service(get!("/users/delete/{ids}", Users::delete))
            //分类
            .service(get!("/video_categories", VideoCategories::index))
            .service(get!("/video_categories/edit/{id}", VideoCategories::edit))
            .service(post!("/video_categories/save/{id}", VideoCategories::save))
            .service(get!("/video_categories/delete/{ids}", VideoCategories::delete))
            //videos
            .service(get!("/videos", Videos::index))
            .service(get!("/videos/edit/{id}", Videos::edit))
            .service(post!("/videos/save/{id}", Videos::save))
            .service(get!("/videos/delete/{ids}", Videos::delete))
            //replies
            .service(get!("/video_replies", VideoReplies::index))
            .service(get!("/video_replies/edit/{id}", VideoReplies::edit))
            .service(post!("/video_replies/save/{id}", VideoReplies::save))
            .service(get!("/video_replies/delete/{ids}", VideoReplies::delete))
    })
    .bind(host_port)?
    .run()
    .await
}
