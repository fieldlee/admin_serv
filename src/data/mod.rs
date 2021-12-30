use actix_web::web;
use mysql::Pool;
use std::sync::Mutex;
use serde_derive::{Deserialize, Serialize};

pub type Db = web::Data<Mutex<Pool>>;
pub type DbRow = mysql::Row;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pager { 
    pub pages: u32,
    pub current: u32,
    pub rows_total: u32,
    pub limit: u32,
}

#[macro_use]
pub mod cond_builder;
#[macro_use]
pub mod data_set;
#[macro_use]
pub mod db;
#[macro_use]
pub mod query_builder;
pub mod model;