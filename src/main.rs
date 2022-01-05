#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use actix_files::Files;
use actix_session::CookieSession;
use std::time::{Duration, Instant};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, middleware};


mod config;
mod common;
mod caches;
mod models;
mod controllers;
mod validations;
mod filters;
#[macro_use]
mod utils;
#[macro_use]
mod data;

use controllers::{index::Index};

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); //正式环境可以注释此行 ***
    env_logger::init(); //正式环境可以注释此行 ***

    let setting = &*config::SETTING;
    let info = &setting.app;
    let conn_string = config::get_conn_string();
    data::db::init_connections(&conn_string); //初始化
    let host_port = &format!("{}:{}", &info.host, &info.port); //地址/端口
    println!("Started At: {}", host_port);

    HttpServer::new(move || {
        let mut tpl = tmpl!("/templates/**/*"); //模板引擎
        tpl.register_filter("state_name", filters::state_name);
        tpl.register_filter("menu_name", filters::menus::menu_name);
        tpl.register_filter("yes_no", filters::yes_no);
        tpl.register_filter("admin_role", filters::admin_roles::role_name);
        tpl.register_filter("position_name", filters::ads::position_name);
        tpl.register_filter("tag_name", filters::video_tags::tag_name);
        tpl.register_filter("author_name", filters::video_authors::author_name);

        App::new()
        .data(tpl)
        .wrap(CookieSession::signed(&[0; 32]).secure(false))
        .wrap(middleware::Logger::default())
        .service(Files::new("/static", "public/static/"))
        .service(Files::new("/upload", "public/upload/"))
        .service(get!("/test",Index::test))
        .service(get!("/",Index::index))
        .service(post!("/index/login", Index::login))

    })
    .bind(host_port)?
    .run()
    .await
}
