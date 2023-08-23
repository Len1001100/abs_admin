use crate::domain::table::LoginCheck;
use rbatis::rbdc::datetime::DateTime;
///Permission Resource Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysPermission {
    pub id: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

///RoleTable
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

///Role Permission relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRolePermission {
    pub id: Option<String>,
    pub role_id: Option<String>,
    pub permission_id: Option<String>,
    pub create_date: Option<DateTime>,
}

///Background user table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

///User role relationship tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserRole {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub create_date: Option<DateTime>,
}

///dictionary table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 新闻 - 动态
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesMoment {
    pub id: Option<i32>,
    pub bigcate: Option<String>,
    pub user_id: Option<i32>,
    pub create_date: Option<DateTime>,
    pub content: Option<String>,
    pub imgusls: Option<String>,
    pub imgwidth: Option<i32>,
    pub imgheight: Option<i32>,
}

/// 新闻 - 新闻
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesNews {
    pub id: Option<i32>,
    pub bigcate: Option<String>,
    pub kindid: Option<String>,
    pub title: Option<String>,
    pub digest: Option<String>,
    pub imgsrc: Option<String>,
    pub detailstr: Option<String>,
    pub ptime: Option<String>,
    pub nid: Option<String>,
    pub source: Option<String>,
    pub detailurl: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 新闻 - 新闻分类
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesNewscate {
    pub id: Option<i32>,
    pub bigcate: Option<String>,
    pub kindids: Option<String>,
    pub kindnames: Option<String>,
}

/// 新闻 - 用户
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesUser {
    pub id: Option<i32>,
    pub bigcate: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub head_url: Option<String>,
    pub tel: Option<String>,
}
