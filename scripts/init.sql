/** 後臺用戶列表 **/
DROP TABLE IF EXISTS admins;
CREATE TABLE IF NOT EXISTS admins ( 
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '登錄名稱',
    password CHAR(32) NOT NULL DEFAULT '' COMMENT '登錄密碼',
    last_ip VARCHAR(32) NOT NULL DEFAULT '' COMMENT '最後IP',
    state TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '狀態',
    login_count INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '登錄次數',
    last_login INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '上次登錄時間',
    role_id INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '角色编号',
    created INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '創建時間',
    updated INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '修改時間',
    INDEX(name),
    PRIMARY KEY(id)
);
INSERT INTO admins (name, password, last_ip, state, login_count, last_login, role_id, created, updated) VALUES 
('admin', md5('qwe123'), '127.0.0.1', 1, 1, UNIX_TIMESTAMP(), 1, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

/** 菜单管理 **/
DROP TABLE IF EXISTS menus;
CREATE TABLE IF NOT EXISTS menus (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    parent_id INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '上级编号',
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '菜单名称',
    level_id TINYINT NOT NULL DEFAULT 0 COMMENT '级别ID,1:主菜单;2:子菜单',
    state TINYINT NOT NULL DEFAULT 0 COMMENT '状态,0:隐藏;1:显示',
    url VARCHAR(50) NOT NULL DEFAULT '' COMMENT '链接地址',
    is_blank TINYINT NOT NULL DEFAULT 0 COMMENT '是否外链,0:否,1:是',
    seq INT NOT NULL DEFAULT 0 COMMENT '排序',
    PRIMARY KEY(id),
    INDEX(parent_id)
);
INSERT INTO menus (parent_id, name, level_id, state, url) VALUES 
(0, '后台权限管理', 1, 1, '#'),
(1, '后台用户列表', 2, 1, '/admins'),
(1, '菜单列表', 2, 1, '/menus'),
(1, '用户角色列表', 2, 1, '/adminRoles');

/** 角色管理 **/
DROP TABLE IF EXISTS admin_roles;
CREATE TABLE IF NOT EXISTS admin_roles (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '角色名称',
    remark VARCHAR(50) NOT NULL DEFAULT '' COMMENT '备注',
    menu_ids JSON COMMENT '菜单编号',
    seq INT NOT NULL DEFAULT 0 COMMENT '排序',
    PRIMARY KEY(id)
);
INSERT INTO admin_roles (name, remark, menu_ids) VALUES 
('系统管理员', '后台用户管理', '[{"id": 1, "menus": [{"id": 2}]}]');

/** 前台用户 **/
DROP TABLE IF EXISTS users;
CREATE TABLE IF NOT EXISTS users (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '用户名称',
    password CHAR(32) NOT NULL DEFAULT '' COMMENT '登錄密碼',
    secret CHAR(32) NOT NULL DEFAULT '' COMMENT '用户密钥',
    mail VARCHAR(100) NOT NULL DEFAULT '' COMMENT '电子邮件',
    last_ip VARCHAR(32) NOT NULL DEFAULT '' COMMENT '最後IP',
    state TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '狀態',
    login_count INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '登錄次數',
    last_login INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '上次登錄時間',
    created INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '創建時間',
    updated INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '修改時間',
    PRIMARY KEY(id),
    INDEX(name)
);
INSERT INTO users (name, password, mail, created, updated) VALUES 
('user', md5('qwe123'), 'abc@gmail.com', UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

/** 视频分类 **/
DROP TABLE IF EXISTS video_categories;
CREATE TABLE IF NOT EXISTS video_categories (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '名称',
    remark VARCHAR(100) NOT NULL DEFAULT '' COMMENT '备注',
    PRIMARY KEY(id)
);
INSERT INTO video_categories (name, remark) VALUES 
('AAA', 'aaa');

/** 视频 **/
DROP TABLE IF EXISTS videos;
CREATE TABLE IF NOT EXISTS videos (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    title VARCHAR(50) NOT NULL DEFAULT '' COMMENT '标题',
    remark VARCHAR(100) NOT NULL DEFAULT '' COMMENT '备注',
    cover_image VARCHAR(200) NOT NULL DEFAULT '' COMMENT '封面',
    duration FLOAT NOT NULL DEFAULT 0 COMMENT '时长',
    content TExT COMMENT '内容',
    PRIMARY KEY(id)
);

/** 评论 **/
DROP TABLE IF EXISTS video_replies;
CREATE TABLE IF NOT EXISTS video_replies (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    video_id INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '视频编号',
    user_id INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '用户编号',
    user_name VARCHAR(200) NOT NULL DEFAULT '' COMMENT '用户名称',
    content TEXT COMMENT '内容',
    PRIMARY KEY(id)
);
