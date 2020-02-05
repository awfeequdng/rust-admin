use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::{VideoTags, VideoTagCacheItem as Item};

lazy_static! { 
    pub static ref VIDEO_TAGS: Mutex<Vec<Item>> = { 
        let rows = get_cache_items();
        Mutex::new(rows)
    };
}

/// 刷新缓存
#[allow(dead_code)]
fn refresh() { 
    let mut list = VIDEO_TAGS.lock().unwrap();
    //(*list).clear();
    *list = get_cache_items();
}

/// 得到所有的视频分类
fn get_cache_items() -> Vec<Item> { 
    let mut conn = db::get_conn();
    let query = query![
        fields => "id, name",
    ];
    let mut list = vec![];
    let rows = VideoTags::fetch_rows(&mut conn, &query, None);
    for r in rows { 
        let (id, name): (usize, String) = from_row!(r);
        list.push(Item{ id, name});
    }
    list
}
