//use tokio for async


use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


async fn user() -> impl Responder {
    HttpResponse::Ok().body("Hello user!")
}


async fn user_info(info: web::Path<(String, i32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.0, info.1))
}


async fn user_info2(info: web::Path<(String, i32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.0, info.1))
}


async fn user_info3(info: web::Path<(String, i32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.0, info.1))
}


async fn user_info4(info: web::Path<(String, i32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.0, info.1))
}


async fn user_info5(info: web::Path<(String, i32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.0, info.1))
}
