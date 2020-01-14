use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};

#[derive(Default, Debug, Serialize)]
pub struct VideoReplies { 
    pub id: usize, //编号
    pub video_id: usize, //名称
    pub reply_id: usize, //备注
    pub user_id: usize, //备注
    pub user_name: String, //备注
    pub content: String, //备注
    pub created: u32, //备注
}

impl Model for VideoReplies { 
    fn get_table_name() -> &'static str { "video_replies" }
}

impl ModelBackend for VideoReplies { 

    type M = Self;

    get_fields!(Self, [
        video_id => usize,
        reply_id => usize,
        user_id => usize,
        user_name => String,
        content => String,
        created => u32,
    ]);
}
