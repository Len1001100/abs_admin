
// use crate::domain::dto::{NesNewsCatePageDTO,NesNewsPageDTO,NesMomentPageDTO,NesMomentAddDTO,NesUserAddDTO,NesUserLoginDTO};
// use crate::domain::vo::{NesNewsCateVO,NesNewsVO,NesMomentVO};
// use crate::domain::table::{NesNewscate,NesNews,NesMoment,NesUser};

// use crate::error::Error;
// use crate::error::Result;
// use crate::pool;
// use crate::service::CONTEXT;

// use rbatis::sql::{Page, PageRequest};
// use std::collections::{BTreeMap, HashMap};

// use rbatis::plugin::object_id::ObjectId;
// use crate::util::password_encoder::PasswordEncoder;

// pub struct NesNewsService {}

// impl  NesNewsService {

//     /// 新闻分类分页
//     pub async fn newscate_page(&self, arg: &NesNewsCatePageDTO) -> Result<Page<NesNewsCateVO>> {
//         let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
//         let data = CONTEXT
//             .rbatis
//             .fetch_page_by_wrapper::<NesNewscate>(
//                 CONTEXT
//                     .rbatis
//                     .new_wrapper()
//                     .do_if(arg.bigcate.is_some(), |w| w.like(NesNewscate::bigcate(), &arg.bigcate))
//                     .order_by(false, &[NesNewscate::id()]),
//                 &page_req,
//             )
//             .await?;
//         let mut vos = vec![];
//         for x in data.records {
//             vos.push(NesNewsCateVO::from(x));
//         }
//         let new_page = Page {
//             records: vos,
//             total: data.total,
//             pages: data.pages,
//             page_no: data.page_no,
//             page_size: data.page_size,
//             search_count: data.search_count,
//         };
//         Ok(new_page)
//     }

//     /// 新闻分页
//     pub async fn news_page(&self, arg: &NesNewsPageDTO) -> Result<Page<NesNewsVO>> {
//         let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
//         let data = CONTEXT
//             .rbatis
//             .fetch_page_by_wrapper::<NesNews>(
//                 CONTEXT
//                     .rbatis
//                     .new_wrapper()
//                     .do_if(arg.title.is_some(), |w| w.like(NesNews::title(), &arg.title))
//                     .order_by(false, &[NesNews::id()]),
//                 &page_req,
//             )
//             .await?;
//         let mut vos = vec![];
//         for x in data.records {
//             vos.push(NesNewsVO::from(x));
//         }
//         let new_page = Page {
//             records: vos,
//             total: data.total,
//             pages: data.pages,
//             page_no: data.page_no,
//             page_size: data.page_size,
//             search_count: data.search_count,
//         };
//         Ok(new_page)
//     }

//     /// 动态分页
//     pub async fn moment_page(&self, arg: &NesMomentPageDTO) -> Result<Page<NesMomentVO>> {
//         let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
//         let data = CONTEXT
//             .rbatis
//             .fetch_page_by_wrapper::<NesMoment>(
//                 CONTEXT
//                     .rbatis
//                     .new_wrapper()
//                     .do_if(arg.content.is_some(), |w| w.like(NesMoment::content(), &arg.content))
//                     .order_by(false, &[NesMoment::id()]),
//                 &page_req,
//             )
//             .await?;
//         let mut vos = vec![];
//         for x in data.records {
//             vos.push(NesMomentVO::from(x));
//         }
//         let new_page = Page {
//             records: vos,
//             total: data.total,
//             pages: data.pages,
//             page_no: data.page_no,
//             page_size: data.page_size,
//             search_count: data.search_count,
//         };
//         Ok(new_page)
//     }
    
//     /// 发布动态
//     pub async fn moment_add(&self, arg: &NesMomentAddDTO) -> Result<u64> {
//         let old: Vec<NesMoment> = CONTEXT
//             .rbatis
//             .fetch_list_by_wrapper(
//                 CONTEXT
//                     .rbatis
//                     .new_wrapper()
//                     .eq(NesMoment::content(), &arg.content)
//             ).await?;
//         if old.len() > 0 {
//             return Err(Error::from(format!("已存在该动态")));
//         }
//         let moment = NesMoment {
//             id: ObjectId::new().to_string().parse::<i32>().unwrap().into(),
//             bigcate: arg.bigcate.clone(),
//             user_id: arg.user_id,
//             create_date: DateTime::now().into(),
//             content: arg.content.clone(),
//             imgusls: arg.imgusls.clone(),
//             imgwidth: None,
//             imgheight: None,
            
//         };
//         let result = Ok(CONTEXT.rbatis.save(&moment, &[]).await?.rows_affected);
//         return result;
//     }

//     /// 用户注册
//     pub async fn user_add(&self, arg: &NesUserAddDTO) -> Result<u64> {
//         let old: Vec<NesUser> = CONTEXT
//             .rbatis
//             .fetch_list_by_wrapper(
//                 CONTEXT
//                     .rbatis
//                     .new_wrapper()
//                     .eq(NesUser::name(), &arg.name)
//             ).await?;
//         if old.len() > 0 {
//             return Err(Error::from(format!("已存在该用户")));
//         }
//         let user = NesUser {
//             id: ObjectId::new().to_string().parse::<i32>().unwrap().into(),
//             bigcate: arg.bigcate.clone(),
//             name: arg.name.clone(),
//             password: arg.password.clone(),
//             salt: None,
//             head_url: None,
//             tel: None,
            
//         };
//         let result = Ok(CONTEXT.rbatis.save(&user, &[]).await?.rows_affected);
//         return result;

//     }

//     /// 用户登录
//     pub async fn user_login(&self, arg: &NesUserLoginDTO) -> Result<u64> {
//         let user: Option<NesUser> =CONTEXT
//             .rbatis
//             .fetch_by_wrapper(
//                 CONTEXT
//                     .rbatis
//                     .new_wrapper()
//                     .eq(NesUser::name(), &arg.name)
//             ).await?;
//         let user = user.ok_or_else(|| Error::from(format!("账号不存在")))?;

//         if PasswordEncoder::verify(
//             user.password.as_ref().ok_or_else(|| Error::from("错误，密码为空"))?,
//             &arg.password,
//         ) {
//             return Ok(1)
//         } else {
//             return Err(Error::from("密码不正确"));
//         }
//     }

// }