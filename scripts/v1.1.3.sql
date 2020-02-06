/** 视频作者 **/
DROP TABLE IF EXISTS video_authors;
CREATE TABLE IF NOT EXISTS video_authors (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL DEFAULT '' COMMENT '名称',
    remark VARCHAR(500) NOT NULL DEFAULT '' COMMENT '备注',
    seq INT NOT NULL DEFAULT 0 COMMENT '排序',
    PRIMARY KEY(id)
) ENGINE=INNODB ENGINE=INNODB DEFAULT CHARSET=UTF8 COLLATE=UTF8_GENERAL_CI;
INSERT INTO video_authors (name, remark) VALUES 
('默认', '默认');

/** 添加菜单选项 **/
set @parent_id = 2; /* 内容管理 */
INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '视频作者', 1, 1, '/video_authors', 1);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '视频作者编辑', 1, 1, '/video_authors/edit/\\d+|/video_authors/save/\\d+', 0);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;

INSERT INTO menus (parent_id, name, level_id, state, url, is_show) VALUES 
(@parent_id, '视频作者删除', 1, 1, '/video_authors/delete/\\d+', 0);
set @last_id = LAST_INSERT_ID();
UPDATE admin_roles SET menu_ids = concat(menu_ids, ',', @last_id) WHERE id = 1;
