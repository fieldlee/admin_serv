use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;
use std::env;


macro_rules! get_setting_from_toml {
    ($struct: ident) => ({
        let result = $struct::default();
        let curr_dir = env::current_dir().unwrap();
        let current_path = curr_dir.to_str().unwrap();
        let toml_file = format!("{}/setting.dev.toml", current_path);
        match File::open(&toml_file) {
            Ok(mut v)=>{
                let mut content = String::new();
                if let Ok(_) = v.read_to_string(&mut content){
                    if let Ok(t) = toml::from_str::<$struct>(&content) {
                        t
                    }else{
                        result
                    }
                }else{
                    result
                }
            }
            Err(err) => { 
                println!("读取文件失败: {}", err);
                result
            }
        }
    });
}


lazy_static! { 
    pub static ref SETTING: Setting = get_setting_from_toml!(Setting);
    // pub static ref DB_INFO: Database = { dbg!(get_setting_from_toml!(Database)) };
    // pub static ref APP_INFO: App = { get_setting_from_toml!(App) };
    // pub static ref OSS_INFO: OSS = { get_setting_from_toml!(OSS) };
}

#[derive(Deserialize,Default,Debug)]
pub struct App {
    pub host :String,
    pub port :usize,
}

/// 数据库连接信息
#[derive(Deserialize, Default, Debug)]
pub struct Database { 
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: usize,
}

/// oss配置信息
#[derive(Deserialize, Default, Debug)]
pub struct OSS { 
    pub access_key_id: String,
    pub access_key_secret: String,
    pub end_point: String,
    pub region: String,
    pub bucket: String,
}

/// 系统配置信息
#[derive(Deserialize, Default, Debug)]
pub struct Setting { 
    pub app: App,
    pub database: Database,
    pub oss: OSS,
}

/// 得到数据库连接字符串
pub fn get_conn_string() -> String { 
    let setting = &*SETTING;
    let db = &setting.database;
    format!("mysql://{}:{}@{}:{}/{}", db.user, db.password, db.host, db.port, db.name)
}
