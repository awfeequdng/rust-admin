# 基于Rust的后台管理系统

## 功能特点
1. 基于X-admin开发,无缝对接layui前端框架,易于修改。(X-Admin: http://x.xuebingsi.com/)

2. 基于actix-web开发,性能测试常年屠榜地位。(Actix框架: https://actix.rs/, 性能测试: https://www.techempower.com/benchmarks/)

3. MVC 设计模式,快速入门,方便上手。

4. Tera 模板引擎,类django模板引擎,简化代码、模板开发。 (Tera: https://tera.netlify.com/docs/)

5. 基于Rust语言特性,有性能、安全保证,先天优于Go/Java/.Net/Php等带GC语言。

## 界面载图
1. 登录界面
![avatar](/image/login.png)

2. 后台管理
![avatar](/image/right.png)

3. 菜单管理
![avatar](/image/menus.png)

4. 角色管理
![avatar](/image/roles.png)

## 二次开发 & 技术交流
请加入QQ群: 1036231916

## 使用说明
1. 下载代码

```bash
git clone https://gitee.com/houhanting/rust-admin.git
cd rust-admin
```

2. 创建数据库(Mysql)并入导入数据

```sql
CREATE DATABASE rust_admin; /* 创建数据库 */
GRANT ALL PRIVILEGES ON `rust_admin`.* to 'rust_admin'@'%' IDENTIFIED BY 'rust-x-lsl'; /* 设置用户名称密码 */
FLUSH PRIVILEGES;
USE rust_admin; /* 选中数据库 */
SOURCE scripts/init.sql; /* 导入初始化数据库(请依据实际路径) */
```

3. 设置nginx代理

3.1 设置并生成Nginx配置文件
```bash
cp nginx.conf.default nginx.conf #复制nginx配置文件
cat "/nginx.conf" >> .git/info/exclude #忽略nginx配置文件
vim nginx.conf #修改相应的域名、目录、代理地址、端口
```

3.2 修改 src/config/mod.rs 配置文件

修改相应的数据库名称、密码、主机、端口以及nginx相应的地址、端口
```bash
vim src/config/mod.rs
```

4. 运行程序
```bash
cargo run
```
或者
```bash
cargo run --release
```

## 捐助支持

当前正在创业当中, 期盼各位帮助支持

![avatar](/image/wx.png) ![avatar](/image/tb.png)
