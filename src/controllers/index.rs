use std::collections::HashMap;
use actix_web::{HttpResponse, web::Form, HttpRequest, web};
use fluffy::{tmpl::Tpl, db, model::Model, datetime, utils, random, response,};
use crate::models::{Index as ThisModel, Admins, OSSResult, OSSData, UploadResult};
use std::env;
use actix_session::Session;
use crate::common::Acl;
use crate::config::{LOGIN_ERROR_MAX, LOGIN_LOCKED_TIME, self};
use crate::caches::admin_roles::ROLE_MENUS;
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;
use std::path::Path;
use std::fs;
pub struct Index{}

impl Index {
    /// test
    pub async fn test() -> HttpResponse{
        HttpResponse::Ok().json("test ok!")
    }

    /// 后台首页登录
    pub async fn index(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "index/index.html")
    }

    /// 错误页面
    pub async fn error(request: HttpRequest, tpl: Tpl) -> HttpResponse { 
        let query_string = request.query_string();
        let queries = fluffy::request::get_queries(&query_string);
        let duration = if let Some(v) = queries.get(&"duration") { if let Ok(n) = v.parse::<usize>() { n } else { 0 } } else { 0 };
        let data = tmpl_data![
            "duration" => &duration,
        ];
        render!(tpl, "index/error.html", &data)
    }
}
