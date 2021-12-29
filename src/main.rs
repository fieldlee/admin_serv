#[macro_use]
extern crate fluffy;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use actix_files::Files;
use actix_session::CookieSession;
use fluffy::db;

use std::time::{Duration, Instant};
use actix_rt;
use actix::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, middleware};
use actix_web_actors::ws;

mod config;
mod controllers;
use controllers::{index::Index};

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); //正式环境可以注释此行 ***
    env_logger::init(); //正式环境可以注释此行 ***

    let setting = &*config::SETTING;
    let info = &setting.app;
    let conn_string = config::get_conn_string();
    db::init_connections(&conn_string); //初始化
    let host_port = &format!("{}:{}", &info.host, &info.port); //地址/端口
    println!("Started At: {}", host_port);

    HttpServer::new(move || {
        App::new()
        .wrap(CookieSession::signed(&[0; 32]).secure(false))
        .wrap(middleware::Logger::default())
        .service(Files::new("/static", "public/static/"))
        .service(Files::new("/upload", "public/upload/"))
        .service(web::resource("/test").to(Index::test))
    })
    .bind(host_port)?
    .run()
    .await
}
