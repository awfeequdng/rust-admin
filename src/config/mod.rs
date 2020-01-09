/// 地址
pub const BIND_HOST: &str = "127.0.0.1";
/// 端口
pub const BIND_PORT: &str = "8081";

// redis连接
//pub const REDIS_CONN_STR: &str = "redis://127.0.0.1";

pub const MYSQL_DB_HOST: &str = "localhost";

pub const MYSQL_DB_NAME: &str = "rust_admin";

pub const MYSQL_DB_USER: &str = "rust_admin";

pub const MYSQL_DB_PASS: &str = "rust-x-lsl";

pub const MYSQL_DB_PORT: &str = "3306";

// 資料庫连接
//pub const MYSQL_CONN_STR: &str = "mysql://rust_admin:rust-x-lsl@localhost:3306/rust_admin";

// 最多允許登錄出錯次數
//pub const LOGIN_ERROR_MAX: usize = 5;

pub fn get_conn_string() -> String { 
    format!("mysql://{}:{}@{}:{}/{}", MYSQL_DB_USER, MYSQL_DB_PASS, MYSQL_DB_HOST, MYSQL_DB_PORT, MYSQL_DB_NAME)
}
