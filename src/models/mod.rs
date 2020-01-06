use std::default::Default;

/// 數據列表操作選項
#[derive(Default)]
pub struct RowOption { 
    pub create: bool, //是否允許添加記錄
    pub update: bool, //是否允許修改記錄
    pub delete: bool, //是否允許刪除記錄
    pub dleete_all: bool, //是否允許一次刪除全部記錄
}

pub trait Backend { 
    
    type M;
    
    fn get_option() -> RowOption { 
        RowOption::default()
    }

    fn get_headers() -> Vec<&'static str>;

    fn get_records() -> Vec<Self::M>;
}

/// 後臺用戶
pub mod admins;
