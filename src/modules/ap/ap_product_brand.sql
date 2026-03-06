-- 菜单 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531a00e1c57a00000014', '品牌库', '69a6d22b007bf0cf00000000', '1', 'ProductBrand',
        'ap/ProductBrand/index', 1, 0, 'C', '0', '0', 'ap:ProductBrand:list', '#', 'admin',
        sysdate(), '', null, '品牌库菜单');


-- 按钮 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531a00e1c57a00000015', '品牌库查询', '69aa531a00e1c57a00000014', '1', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductBrand:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531a00e1c57a00000016', '品牌库新增', '69aa531a00e1c57a00000014', '2', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductBrand:add', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531a00e1c57a00000017', '品牌库修改', '69aa531a00e1c57a00000014', '3', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductBrand:edit', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531a00e1c57a00000018', '品牌库删除', '69aa531a00e1c57a00000014', '4', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductBrand:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531a00e1c57a00000019', '品牌库导出', '69aa531a00e1c57a00000014', '5', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductBrand:export', '#', 'admin', sysdate(), '', null, '');