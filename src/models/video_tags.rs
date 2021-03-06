use crate::data::{model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use std::collections::HashMap;
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct VideoTags { 
    pub id: usize,
    pub name: String,
    pub remark: String, 
    pub seq: isize,
}

impl Model for VideoTags { 
    fn get_table_name() -> &'static str { "video_tags" }
}

impl ModelBackend for VideoTags { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
        seq => isize,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("name", "名称必须在2-20之间", 2, 20, true)
            .string_limit("remark", "备注长度必须在0-50之间", 50)
            .is_numeric("seq", "排序必须是有效的数字")
            .validate()
    }
}
