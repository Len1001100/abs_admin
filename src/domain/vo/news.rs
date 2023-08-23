use crate::domain::table::NesNewscate;
use crate::domain::table::NesNews;
use crate::domain::table::NesMoment;
use rbatis::rbdc::datetime::DateTime;

///  新闻 - 新闻分类
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesNewsCateVO {
    #[serde(flatten)]
    pub id: Option<i32>,
    pub bigcate: Option<String>,
    pub kindids: Option<String>,
    pub kindnames: Option<String>,
}

impl From<NesNewscate> for NesNewsCateVO {
    fn from(arg: NesNewscate) -> Self {
        Self {
            id: arg.id,
            bigcate: arg.bigcate,
            kindids: arg.kindids,
            kindnames: arg.kindnames,
        }
    }
}

impl NesNewsCateVO {
    // TODO
}

/// 新闻 - 新闻
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesNewsVO {
    #[serde(flatten)]
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

impl From<NesNews> for NesNewsVO {
    fn from(arg: NesNews) -> Self {
        Self {
            id: arg.id,
            bigcate: arg.bigcate,
            kindid: arg.kindid,
            title: arg.title,
            digest: arg.digest,
            imgsrc: arg.imgsrc,
            detailstr: arg.detailstr,
            ptime: arg.ptime,
            nid: arg.nid,
            source: arg.source,
            detailurl: arg.detailurl,
            create_date: arg.create_date,
        }
    }
}

impl NesNewsVO {
    // TODO
}

/// 新闻 - 动态
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NesMomentVO {
    #[serde(flatten)]
    pub id: Option<i32>,
    pub bigcate: Option<String>,
    pub user_id: Option<i32>,
    pub create_date: Option<DateTime>,
    pub content: Option<String>,
    pub imgusls: Option<String>,
    pub imgwidth: Option<i32>,
    pub imgheight: Option<i32>,
}

impl From<NesMoment> for NesMomentVO {
    fn from(arg: NesMoment) -> Self {
        Self {
            id: arg.id,
            bigcate: arg.bigcate,
            user_id: arg.user_id,
            create_date: arg.create_date,
            content: arg.content,
            imgusls: arg.imgusls,
            imgwidth: arg.imgwidth,
            imgheight: arg.imgheight,
        }
    }
}

impl NesMomentVO {
    // TODO
}

