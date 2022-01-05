use std::collections::HashMap;
use actix_session::Session;
use actix_web::{HttpResponse, web::Form, HttpRequest, web};
use crate::caches::admin_roles::{ROLE_MENUS};
use crate::config::{LOGIN_LOCKED_TIME,LOGIN_ERROR_MAX};
use crate::common::Acl;
use crate::data::model::Model;
use crate::models::{Index as ThisModel, Admins, OSSResult, OSSData, UploadResult};
use crate::{render,tmpl_data,query,update_row,cond,from_row};
use crate::utils::{tmpl::Tpl,request,datetime,response,random,com_fun};
use crate::data::db;
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
    /// 登录
    pub async fn login(session:Session,post:Form<HashMap<String,String>>) -> HttpResponse{
        if let Ok(locked_time) = session.get::<usize>("locked_time") {
            if let Some(n) = locked_time {
                if (datetime::timestamp() as usize) - n < LOGIN_LOCKED_TIME {
                    return response::error("登录次败次数过多,请2小时后再次尝试");
                }
            }
        }
        let mut failure_count = 0_usize; //登录失败的次数
        if let Ok(failure) = session.get::<usize>("failure_count") {  //检测登录失败次数
            if let Some(n) = failure { 
                failure_count = n; //已经失败的次数
                if n > LOGIN_ERROR_MAX { 
                    if let Err(message) = session.set::<usize>("locked_time", datetime::timestamp() as usize) { 
                        return response::error(&message.to_string());
                    }
                    return response::error("失败次数过多, 请稍后重试");
                }
            }
        } else { 
            if let Err(message) = session.set::<usize>("failure_count", failure_count) { 
                return response::error(&message.to_string());
            }
        } //设置登录失败次数的默认值
        //如果校验数据出现错误
        if let Err(message) = ThisModel::check_login(&post) {  
            return response::error(&message);
        }
        
        let name = post.get("username").unwrap();
        let password_ori = post.get("password").unwrap();
        let query = query![fields => "id, password, secret, login_count, role_id",];
        let cond = cond!["name" => &name,];
        let mut conn = db::get_conn();
        if let Some(row) = Admins::fetch_row(&mut conn, &query, Some(&cond)) { 
            let (id, password, secret, login_count, role_id): (usize, String, String, usize, usize) = from_row!(row);
            let password_enc = com_fun::get_password(password_ori, &secret);
            if password_enc != password {  //对比加密之后的密码是否一致
                session.set::<usize>("failure_count", failure_count + 1).unwrap();
                return response::error("用户名称或密码错误");
            }

            let secret_new = random::rand_str(32);
            let password_new = com_fun::get_password(&password_ori, &secret_new);
            let now = datetime::timestamp();
            let data = update_row![
                "login_count" => login_count + 1,
                "last_login" => &now,
                "updated" => &now,
                "secret" => &secret_new,
                "password" => &password_new,
            ];
            let cond = cond!["id" => id,];
            if  Admins::update(&mut conn, &data, &cond) == 0 { 
                session.set::<usize>("failure_count", failure_count + 1).unwrap();
                return response::error("更新用户信息失败");
            }

            session.remove("failure_count"); //清空失败次数
            session.remove("locked_time"); //清空锁定时间
            session.set::<usize>("user_id", id).unwrap(); //session
            session.set::<String>("user_name", name.to_owned()).unwrap(); //session
            session.set::<usize>("role_id", role_id).unwrap(); //session
            return response::ok();
        } 
        session.set::<usize>("failure_count", failure_count + 1).unwrap();
        response::error("用户名称或密码错误")
    }
    /// 退出系统
    pub async fn logout(session: Session) -> HttpResponse { 
        session.remove("user_id");
        session.remove("user_name");
        session.remove("role_id");
        response::ok()
    }
    /// 错误页面
    pub async fn error(request: HttpRequest, tpl: Tpl) -> HttpResponse { 
        let query_string = request.query_string();
        let queries = request::get_queries(&query_string);
        let duration = if let Some(v) = queries.get(&"duration") { if let Ok(n) = v.parse::<usize>() { n } else { 0 } } else { 0 };
        let data = tmpl_data![
            "duration" => &duration,
        ];
        render!(tpl, "index/error.html", &data)
    }
    /// 修改密码
    pub async fn change_pwd(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error?duration=2");
        }
        return render!(tpl, "admins/change_pwd.html");
    }

    /// 保存修改密码
    pub async fn change_pwd_save(session: Session, post: Form<HashMap<String, String>>) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::error("缺少权限");
        }
        if let Err(message) = ThisModel::check_change_pwd(&post) { //检测密码输入是否正确
            return response::error(message);
        }
        let password_ori = post.get("old_password").unwrap();
        let user_id = session.get::<usize>("user_id").unwrap().unwrap(); //用户编号
        let query = query![fields => "secret, password", ];
        let cond = cond!["id" => user_id, ];
        let mut conn = db::get_conn();
        let row = if let Some(r) = Admins::fetch_row(&mut conn, &query, Some(&cond)) { r }  else { return response::error("检测用户信息失败"); };
        let (secret, password): (String, String) = from_row!(row);
        if com_fun::get_password(&password_ori, &secret) != password { 
            return response::error("旧的密码输入错误");
        }
        let password_new = post.get("password").unwrap();
        let secret_new = random::rand_str(32);
        let password_enc = com_fun::get_password(&password_new, &secret_new);
        let data = update_row![
            "password" => &password_enc,
            "secret" => &secret_new,
            "updated" => &datetime::timestamp(),
        ];
        if Admins::update(&mut conn, &data, &cond) == 0 { 
            return response::error("修改密码失败");
        }
        response::ok()
    }
    /// 后台管理主界面
    pub async fn manage(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error?duration=2");
        }
        let mut role_id = 0;
        if let Ok(v) = session.get::<usize>("role_id") { 
            if let Some(n) = v { 
                role_id = n;
            }
        }

        let role_menus = &*ROLE_MENUS.lock().unwrap();
        let menus = role_menus.get(&role_id);
        let data = tmpl_data![
            "menus" => &menus,
        ];
        render!(tpl, "index/manage.html", &data)
    }
}
