// use actix_web::{web, Responder};
// use crate::service::CONTEXT;
// use crate::domain::dto::{NesNewsCatePageDTO,NesNewsPageDTO,NesMomentPageDTO,NesMomentAddDTO,NesUserAddDTO,NesUserLoginDTO};
// use crate::domain::vo::RespVO;

// /// 新闻分类分页
// pub async fn newscate_page(arg: web::Json<NesNewsCatePageDTO>) -> impl Responder {
//     let vo = CONTEXT.nes_news_service.newscate_page(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }

// /// 新闻分页
// pub async fn news_page(arg: web::Json<NesNewsPageDTO>) -> impl Responder {
//     let vo = CONTEXT.nes_news_service.news_page(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }

// /// 动态分页
// pub async fn moment_page(arg: web::Json<NesMomentPageDTO>) -> impl Responder {
//     let vo = CONTEXT.nes_news_service.moment_page(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }

// /// 发布动态
// pub async fn moment_add(arg: web::Json<NesMomentAddDTO>) -> impl Responder {
//     let vo = CONTEXT.nes_news_service.moment_add(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }

// /// 用户注册
// pub async fn user_add(arg: web::Json<NesUserAddDTO>) -> impl Responder {
//     let vo = CONTEXT.nes_news_service.user_add(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }

// /// 用户登录
// pub async fn user_login(arg: web::Json<NesUserLoginDTO>) -> impl Responder {
//     let vo = CONTEXT.nes_news_service.user_login(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }
