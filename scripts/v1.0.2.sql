/** 添加菜单选项 **/
INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(0, '系统管理', 0, 1, '#', 1);
set @last_id = LAST_INSERT_ID();
set @parent_id = @last_id;
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '网站设置', 1, 1, '/configs/edit/1', 1);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '网站设置保存', 1, 1, '/configs/save/1', 0);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '网站导航', 1, 1, '/navs', 1);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '网站导航编辑', 1, 1, '/navs/edit/\\d+|/navs/save/\\d+', 0);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '网站导航删除', 1, 1, '/navs/delete/\\d+', 0);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

/** 导航 **/
DROP TABLE IF EXISTS navs;
CREATE TABLE IF NOT EXISTS navs (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '名称',
    url VARCHAR(200) NOT NULL DEFAULT '' COMMENT '链接地址',
    is_blank TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '是否外链',
    remark VARCHAR(100) NOT NULL DEFAULT '' COMMENT '说明',
    seq INT NOT NULL DEFAULT 0 COMMENT '排序',
    PRIMARY KEY(id)
) ENGINE=INNODB DEFAULT CHARSET=UTF8 COLLATE=UTF8_GENERAL_CI;

/** 系统配置 **/
DROP TABLE IF EXISTS configs;
CREATE TABLE IF NOT EXISTS configs (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    site_name VARCHAR(50) NOT NULL DEFAULT '' COMMENT '站点名称',
    site_url VARCHAR(200) NOT NULL DEFAULT '' COMMENT '主页地址',
    seo_keyword VARCHAR(250) NOT NULL DEFAULT '' COMMENT 'SEO关键字',
    seo_desc VARCHAR(250) NOT NULL DEFAULT '' COMMENT 'SEO描述',
    copyright VARCHAR(200) NOT NULL DEFAULT '' COMMENT '版权',
    PRIMARY KEY(id)
) ENGINE=INNODB DEFAULT CHARSET=UTF8 COLLATE=UTF8_GENERAL_CI;
INSERT INTO configs (site_name, site_url, seo_keyword, seo_desc, copyright) VALUES 
('网站名称', 'http://site.cn/', '用于SEO的网站关键字', '用于SEO的网站描述', '网站版权信息');
