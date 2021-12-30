use std::collections::HashMap;
use mysql::prelude::Queryable;
use super::{DbRow as Row};
use super::{
    cond_builder::CondBuilder, 
    data_set::DataSet, 
    query_builder::QueryBuilder,
};
use crate::utils::datetime;
use super::Pager;

pub struct Db { }

pub trait Model {

    /// 獲取表名稱
    fn get_table_name() -> &'static str { 
        &""
    }

    // 獲取所有記錄
    //fn find_all<P: GenericConnection>(pool: &mut P, query: &QueryBuilder, cond: Option<&CondBuilder>) -> Vec<M> {
    //    let sql = query.build_query(Self::get_table_name(), cond);
    //    let rows: Vec<M> = pool.prep_exec(sql, ())
    //        .map(|result| {
    //            result.map(|r| r.unwrap())
    //            .map(|r| { 
    //                let mut m: M = Default::default();
    //                Self::process_row(r, &mut m);
    //                m
    //            }).collect()
    //        }).unwrap();
    //    rows
    //}
    
    /// 得到分頁信息
    fn get_pager<P: Queryable>(pool: &mut P, query: &QueryBuilder, cond: Option<&CondBuilder>) -> Pager { 
        let sql = query.build_query_total(Self::get_table_name(), cond);
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        let rows_total: u32 = { 
            let result = pool.exec_first(sql, ()).unwrap();
            if let Some(r) = result { 
                let (total,): (u32,) = from_row!(r);
                total
            } else { 0 }
        };
        let limit = query.page_size;
        let pages = { 
            if rows_total > 0 { 
                let val = rows_total / limit;
                if rows_total % limit == 0 { val } else { val + 1 }
            } else { 1 }
        };
        Pager { pages, current: query.page, rows_total, limit, }
    }

    /// 獲取一條記錄
    //fn find<P: GenericConnection>(pool: &mut P, query: &QueryBuilder, cond: Option<&CondBuilder>) -> Option<M> {
    //    let sql = query.build_query_first(Self::get_table_name(), cond);
    //    pool.first_exec(sql, ())
    //    .unwrap()
    //    .map(|r| { 
    //        let mut m: M = Default::default();
    //        Self::process_row(r, &mut m);
    //        m
    //    })
    //}

    /// 得到所有行
    fn fetch_rows<P: Queryable>(pool: &mut P, fields: &QueryBuilder, cond: Option<&CondBuilder>) -> Vec<Row> {
        let sql = fields.build_query(Self::get_table_name(), cond);
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        let query_result = pool.exec_iter(sql,()).unwrap();
        let mut rows: Vec<Row> = query_result.map(|r| {
            r.unwrap()
        }).collect();
        rows.shrink_to_fit();
        rows
    }
        
    /// 獲得一條數據
    fn fetch_row<P: Queryable>(pool: &mut P, query: &QueryBuilder, cond: Option<&CondBuilder>) -> Option<Row> {
        let sql = query.build_query_first(Self::get_table_name(), cond); 
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        let query_result = pool.exec_first(sql, ());
        let result = if let Ok(v) = query_result { v } else { 
            return None;
        };
        result
    }

    /// 獲得所有行
    fn query<P: Queryable>(pool: &mut P, sql: &str) -> Vec<Row> {
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        let query_result = pool.exec_iter(sql, ()).unwrap();
        let mut rows: Vec<mysql::Row> = query_result.map(|r| {
            r.unwrap()
        }).collect();
        rows.shrink_to_fit();
        rows
    }

    /// 獲得一行數據
    fn query_first<P: Queryable>(pool: &mut P, sql: &str) -> Option<Row> {
        #[cfg(feature="debug")]
        println!("SQL: {}", sql);
        let query_result = pool.exec_first(sql, ());
        let result = if let Ok(v) = query_result { v } else { 
            return None;
        };
        result
    }

    /// create the record
    fn execute<P: Queryable>(pool: &mut P, sql: &str) -> u64 { 
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        //let sql = &format!("INSERT INTO {} {}", Self::get_table_name(), data.build());
        if let Ok(result) = pool.query_iter(sql) { 
            result.last_insert_id().unwrap()
        } else { 0 }
    }


    /// create the record
    fn create<P: Queryable>(pool: &mut P, data: &DataSet) -> u64 { 
        let sql = &format!("INSERT INTO {} {}", Self::get_table_name(), data.build());
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        if let Ok(result) = pool.query_iter(sql) { 
            result.last_insert_id().unwrap()
        } else { 0 }
    }
    
    /// update the record
    fn update<P: Queryable>(pool: &mut P, data: &DataSet, cond: &CondBuilder) -> u64 { 
        let sql = &format!("UPDATE {} SET {} WHERE {}", Self::get_table_name(), data.build(), cond.build());
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        if let Ok(result) = pool.query_iter(sql) { 
            result.affected_rows()
        } else { 0 }
    }

    /// update the record
    fn delete<P: Queryable>(pool: &mut P, cond: &CondBuilder) -> u64 { 
        let sql = &format!("DELETE FROM {} WHERE {}", Self::get_table_name(), cond.build());
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        if let Ok(result) = pool.query_iter(sql) { 
            result.affected_rows()
        } else { 0 }
    }

    // 处理单条记录
    //fn process_row(_r: Row, _m: &mut M) { }
}

impl Model for Db { }

impl Db { 
    
    /// 获取数据库当中所有表
    pub fn get_tables<P: Queryable>(pool: &mut P) -> Vec<String> {
        let sql = "SHOW TABLES";
        let query_result = pool.query_iter(sql).unwrap();
        query_result.map(|r| { let v = from_row!(r.unwrap()); v }).collect::<Vec<String>>()
    }
    
    /// 获取表的所有字段
    pub fn get_fields<P: Queryable>(pool: &mut P, database: &str, table_name: &str) -> Vec<String> {
        let sql = format!("SELECT COLUMN_NAME FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'", database, table_name);
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        let query_result = pool.query_iter(sql).unwrap();
        query_result.map(|r| { let v = from_row!(r.unwrap()); v }).collect::<Vec<String>>()
    }

    /// 获取所有表-字段
    pub fn get_table_fields<P: Queryable>(pool: &mut P, database: &str) -> HashMap<String, Vec<String>> { 
        let sql = "SHOW TABLES";
        #[cfg(feature="debug")]
        println!("[Debug] SQL: {}", sql);
        let query_result = pool.query_iter(sql).unwrap();
        let tables = query_result.map(|r| { let v = from_row!(r.unwrap()); v }).collect::<Vec<String>>();
        let mut table_fields: HashMap<String, Vec<String>> = HashMap::new();
        for table in &tables { 
            let sql_fields = format!("SELECT COLUMN_NAME FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'", database, table);
            let query_result = pool.query_iter(sql_fields).unwrap();
            let fields = query_result.map(|r| { let v = from_row!(r.unwrap()); v }).collect::<Vec<String>>();
            table_fields.insert(table.to_owned(), fields);
        }
        table_fields
    }

    /// 檢測並重新組合準備提交到數據庫的字段信息
    pub fn check_fields(table_name: &str, real_fields: &HashMap<String, Vec<String>>, post_fields: HashMap<String, String>, is_update: bool) -> HashMap<String, String> { 
        let mut checked_fields: HashMap<String, String> = HashMap::new();
        if let Some(v) = real_fields.get(table_name) { 
            for field in v { 
                if field == "id" { 
                    continue;
                }
                if field == "updated" && is_update { 
                    let now = datetime::timestamp();
                    checked_fields.insert("updated".to_owned(), now.to_string());
                    continue;
                }
                if field == "created" { 
                    let now = datetime::timestamp();
                    checked_fields.insert("created".to_owned(), now.to_string());
                    continue;
                }
                if let Some(ev) = post_fields.get(field) { 
                    checked_fields.insert(field.to_owned(), ev.to_owned());
                    continue;
                }
            }
        }
        checked_fields
    }
}
