use actix_web_static_files::resource_dir;

fn main() {
    // 请依据实际情况填写以下静态文件位置
    resource_dir("./public/static").build().unwrap();
}
