use actix_web::{HttpResponse, web::Form, HttpRequest, web};

pub struct Index{}

impl Index {
    pub async fn test()->HttpResponse{
        HttpResponse::Ok().json("test ok!")
    }
}