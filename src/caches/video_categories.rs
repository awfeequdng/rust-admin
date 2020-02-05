use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::{VideoCategories};

lazy_static! { 
    pub static ref VIDEO_CATEGORIES: Mutex<HashMap<usize, String>> = { 
        let rows = get_cache_items();
        Mutex::new(rows)
    };
}

/// 刷新缓存
#[allow(dead_code)]
fn refresh() { 
    let mut list = VIDEO_CATEGORIES.lock().unwrap();
    //(*list).clear();
    *list = get_cache_items();
}

/// 得到所有的视频分类
fn get_cache_items() -> HashMap<usize, String> { 
    let mut conn = db::get_conn();
    let query = query![
        fields => "id, name",
    ];
    let mut list = HashMap::new();
    let rows = VideoCategories::fetch_rows(&mut conn, &query, None);
    for r in rows { 
        let (id, name): (usize, String) = from_row!(r);
        list.insert(id, name);
    }
    list
}
