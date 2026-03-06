-- 菜单 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531800e1c57a0000000a', '供应商', '', '1', 'ApSupplier',
        'ap/ApSupplier/index', 1, 0, 'C', '0', '0', 'ap:ApSupplier:list', '#', 'admin',
        sysdate(), '', null, '供应商菜单');


-- 按钮 SQL
insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531800e1c57a0000000b', '供应商查询', '69aa531800e1c57a0000000a', '1', '#', '', 1, 0, 'F', '0', '0',
        'ap:ApSupplier:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531800e1c57a0000000c', '供应商新增', '69aa531800e1c57a0000000a', '2', '#', '', 1, 0, 'F', '0', '0',
        'ap:ApSupplier:add', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531800e1c57a0000000d', '供应商修改', '69aa531800e1c57a0000000a', '3', '#', '', 1, 0, 'F', '0', '0',
        'ap:ApSupplier:edit', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531800e1c57a0000000e', '供应商删除', '69aa531800e1c57a0000000a', '4', '#', '', 1, 0, 'F', '0', '0',
        'ap:ApSupplier:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible,
                      status, perms, icon, create_by, create_time, update_by, update_time, remark)
values ('69aa531800e1c57a0000000f', '供应商导出', '69aa531800e1c57a0000000a', '5', '#', '', 1, 0, 'F', '0', '0',
        'ap:ApSupplier:export', '#', 'admin', sysdate(), '', null, '');