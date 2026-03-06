-- 菜单 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531d00e1c57a00000028', '单位', '69a6d22b007bf0cf00000000', '1', 'productUnit',
        'ap/productUnit/index', 1, 0, 'C', '0', '0', 'ap:productUnit:list', '#', 'admin',
        sysdate(), '', null, '单位菜单');


-- 按钮 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531d00e1c57a00000029', '单位查询', '69aa531d00e1c57a00000028', '1', '#', '', 1, 0, 'F', '0', '0',
        'ap:productUnit:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531d00e1c57a0000002a', '单位新增', '69aa531d00e1c57a00000028', '2', '#', '', 1, 0, 'F', '0', '0',
        'ap:productUnit:add', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531d00e1c57a0000002b', '单位修改', '69aa531d00e1c57a00000028', '3', '#', '', 1, 0, 'F', '0', '0',
        'ap:productUnit:edit', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531d00e1c57a0000002c', '单位删除', '69aa531d00e1c57a00000028', '4', '#', '', 1, 0, 'F', '0', '0',
        'ap:productUnit:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531d00e1c57a0000002d', '单位导出', '69aa531d00e1c57a00000028', '5', '#', '', 1, 0, 'F', '0', '0',
        'ap:productUnit:export', '#', 'admin', sysdate(), '', null, '');