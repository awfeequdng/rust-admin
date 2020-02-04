#[macro_use] extern crate fluffy;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;

use actix_web::{App, HttpServer, middleware, web};
use fluffy::{db};
use actix_session::{CookieSession};
use actix_files::Files;

mod config;
mod filters;
mod validations;
mod models;
mod common;
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
    video_tags::VideoTags,
    user_levels::UserLevels,
    watch_records::WatchRecords,
    ads::Ads,
    navs::Navs,
    configs::Configs,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    // 正式环境可以去掉日志显示
    std::env::set_var("RUST_LOG", "actix_web=info"); //正式环境可以注释此行 ***
    env_logger::init(); //正式环境可以注释此行 ***

    let setting = &*config::SETTING;
    let info = &setting.app;
    let conn_string = config::get_conn_string();
    db::init_connections(&conn_string); //資料庫初始化
    let host_port = &format!("{}:{}", &info.host, &info.port); //地址/端口
    println!("Started At: {}", host_port);

    //let table_fields = caches::TABLE_FIELDS.lock().unwrap();
    HttpServer::new(|| {

        let mut tpl = tmpl!("/templates/**/*"); //模板引擎
        tpl.register_filter("state_name", filters::state_name);
        tpl.register_filter("menu_name", filters::menus::menu_name);
        tpl.register_filter("yes_no", filters::yes_no);
        tpl.register_filter("admin_role", filters::admin_roles::role_name);
        tpl.register_filter("position_name", filters::ads::position_name);

        //let generated = generate();
        
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(tpl)
            .wrap(middleware::Logger::default()) //正式环境可以注释此行 ***
            .service(Files::new("/static", "public/static/")) //静态文件目录
            .service(Files::new("/upload", "public/upload/")) //上传文件目录
            .service(web::resource("/test").to(Index::test))
            .service(get!("/", Index::index))
            .service(post!("/index/login", Index::login))
            .service(get!("/index/manage", Index::manage))
            .service(get!("/index/right", Index::right))
            .service(get!("/index/right", Index::right))
            .service(get!("/index/error", Index::error))
            .service(get!("/index/logout", Index::logout))
            .service(get!("/index/change_pwd", Index::change_pwd))
            .service(post!("/index/change_pwd_save", Index::change_pwd_save))
            .service(get!("/index/oss_signed_url", Index::oss_signed_url))
            .service(post!("/index/upload", Index::upload_images))
            //后台用户
            .service(get!("/admins", Admins::index))
            .service(get!("/admins/edit/{id}", Admins::edit))
            .service(post!("/admins/save/{id}", Admins::save))
            .service(get!("/admins/delete/{ids}", Admins::delete))
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
            //前台用户
            .service(get!("/users", Users::index))
            .service(get!("/users/edit/{id}", Users::edit))
            .service(post!("/users/save/{id}", Users::save))
            .service(get!("/users/delete/{ids}", Users::delete))
            //视频分类
            .service(get!("/video_categories", VideoCategories::index))
            .service(get!("/video_categories/edit/{id}", VideoCategories::edit))
            .service(post!("/video_categories/save/{id}", VideoCategories::save))
            .service(get!("/video_categories/delete/{ids}", VideoCategories::delete))
            //视频管理
            .service(get!("/videos", Videos::index))
            .service(get!("/videos/edit/{id}", Videos::edit))
            .service(post!("/videos/save/{id}", Videos::save))
            .service(get!("/videos/delete/{ids}", Videos::delete))
            //视频标签
            .service(get!("/video_tags", VideoTags::index))
            .service(get!("/video_tags/edit/{id}", VideoTags::edit))
            .service(post!("/video_tags/save/{id}", VideoTags::save))
            .service(get!("/video_tags/delete/{ids}", VideoTags::delete))
            //用户等级
            .service(get!("/user_levels", UserLevels::index))
            .service(get!("/user_levels/edit/{id}", UserLevels::edit))
            .service(get!("/user_levels/delete/{ids}", UserLevels::delete))
            .service(post!("/user_levels/save/{id}", UserLevels::save))
            //观看记录
            .service(get!("/watch_records", WatchRecords::index))
            .service(get!("/watch_records/edit/{id}", WatchRecords::edit))
            .service(get!("/watch_records/delete/{ids}", WatchRecords::delete))
            .service(post!("/watch_records/save/{id}", WatchRecords::save))
            //replies
            .service(get!("/video_replies", VideoReplies::index))
            .service(get!("/video_replies/edit/{id}", VideoReplies::edit))
            .service(post!("/video_replies/save/{id}", VideoReplies::save))
            .service(get!("/video_replies/delete/{ids}", VideoReplies::delete))
            //广告管理
            .service(get!("/ads", Ads::index))
            .service(get!("/ads/edit/{id}", Ads::edit))
            .service(post!("/ads/save/{id}", Ads::save))
            .service(get!("/ads/delete/{ids}", Ads::delete))
            //网站导航
            .service(get!("/navs", Navs::index))
            .service(get!("/navs/edit/{id}", Navs::edit))
            .service(post!("/navs/save/{id}", Navs::save))
            .service(get!("/navs/delete/{ids}", Navs::delete))
            //网站设置
            .service(get!("/configs/edit/{id}", Configs::edit))
            .service(post!("/configs/save/{id}", Configs::save))

    })
    .bind(host_port)?
    .run()
    .await
}
