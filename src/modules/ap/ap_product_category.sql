-- 菜单 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69ae99770006bb0300000017', '商品分类', '69a6d22b007bf0cf00000000', '1', 'ProductCategory',
        'ap/ProductCategory/index', 1, 0, 'C', '0', '0', 'ap:ProductCategory:list', '#', 'admin',
        sysdate(), '', null, '商品分类菜单');


-- 按钮 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69ae99770006bb0300000018', '商品分类查询', '69ae99770006bb0300000017', '1', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductCategory:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69ae99770006bb0300000019', '商品分类新增', '69ae99770006bb0300000017', '2', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductCategory:add', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69ae99770006bb030000001a', '商品分类修改', '69ae99770006bb0300000017', '3', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductCategory:edit', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69ae99770006bb030000001b', '商品分类删除', '69ae99770006bb0300000017', '4', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductCategory:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69ae99770006bb030000001c', '商品分类导出', '69ae99770006bb0300000017', '5', '#', '', 1, 0, 'F', '0', '0',
        'ap:ProductCategory:export', '#', 'admin', sysdate(), '', null, '');