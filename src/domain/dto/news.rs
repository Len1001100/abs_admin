use serde::{Deserialize, Serialize};

/// 新闻分类分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NesNewsCatePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub bigcate: Option<String>,
}

/// 新闻分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NesNewsPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub title: Option<String>,
}

/// 动态分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NesMomentPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub content: Option<String>,
}

/// 发布动态
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NesMomentAddDTO {
    pub bigcate: Option<String>,
    pub user_id: Option<i32>,
    pub content: Option<String>,
    pub imgusls: Option<String>,
}

/// 用户注册
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NesUserAddDTO {
    pub bigcate: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

/// 用户登录
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NesUserLoginDTO {
    pub bigcate: Option<String>,
    pub name: Option<String>,
    pub password: String,
}