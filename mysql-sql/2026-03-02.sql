CREATE TABLE `ap_supplier` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'id',
    `name` varchar(128) NOT NULL COMMENT '供应商名称',
    `system_code` varchar(255) NOT NULL COMMENT '系统编码',
    `short_name` varchar(255) NOT NULL COMMENT '简称',
    `contact_man` varchar(255) DEFAULT NULL COMMENT '联系人',
    `contact_phone` varchar(255) DEFAULT NULL COMMENT '联系方式',
    `contact_info` varchar(1000) DEFAULT NULL COMMENT '联系信息',
    `contact_wechat` varchar(50) DEFAULT NULL COMMENT '联系微信',
    `url_one` varchar(255) DEFAULT NULL COMMENT '链接一',
    `url_two` varchar(255) DEFAULT NULL COMMENT '链接二',

    `email` varchar(50) DEFAULT NULL COMMENT '邮箱',
    `address` varchar(50) DEFAULT NULL COMMENT '地址',
    `create_by` varchar(64) DEFAULT '' COMMENT '创建者',
    `create_time` datetime DEFAULT NULL COMMENT '创建时间',
    `update_by` varchar(64) DEFAULT '' COMMENT '更新者',
    `update_time` datetime DEFAULT NULL COMMENT '更新时间',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    PRIMARY KEY (`id`),
    UNIQUE KEY `system_code` (`system_code`)
) ENGINE=InnoDB AUTO_INCREMENT=103 DEFAULT CHARSET=utf8mb4 COMMENT='供应商';

CREATE TABLE `ap_product_brand` (
        `id` bigint(20) NOT NULL AUTO_INCREMENT,
        `name` varchar(255) DEFAULT '' COMMENT '名称',
        `logo` varchar(255) DEFAULT '' COMMENT 'logo',
        `create_id` bigint(20) NOT NULL COMMENT '创建者ID',
        `create_by` varchar(64) DEFAULT '' COMMENT '创建者',
        `create_header_img` varchar(255) DEFAULT '' COMMENT '创建人头像',
        `create_time` datetime DEFAULT NULL COMMENT '创建时间',
        `update_id` bigint(20) NOT NULL COMMENT '更新者ID',
        `update_by` varchar(64) DEFAULT '' COMMENT '更新者',
        `update_time` datetime DEFAULT NULL COMMENT '更新时间',
        `remark` varchar(500) DEFAULT NULL COMMENT '备注',
        PRIMARY KEY (`id`),
        UNIQUE KEY `ap_product_brand_name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=106 DEFAULT CHARSET=utf8mb4 COMMENT='品牌库';

CREATE TABLE `ap_product_category` (
       `id` bigint(20) NOT NULL AUTO_INCREMENT,
       `name` varchar(255) NOT NULL COMMENT '名称',
       `parent_id` int(11) DEFAULT NULL COMMENT '上级分类id',
       `tenant_id` bigint(20) NOT NULL COMMENT '租户id',
       `tenant_name` varchar(255) DEFAULT '' COMMENT '租户简称',
       `create_id` bigint(20) NOT NULL COMMENT '创建者ID',
       `create_by` varchar(64) DEFAULT '' COMMENT '创建者',
       `create_time` datetime DEFAULT NULL COMMENT '创建时间',
       `update_id` bigint(20) NOT NULL COMMENT '更新者ID',
       `update_by` varchar(64) DEFAULT '' COMMENT '更新者',
       `update_time` datetime DEFAULT NULL COMMENT '更新时间',
       `remark` varchar(500) DEFAULT NULL COMMENT '备注',
       `order_num` int(11) DEFAULT '0' COMMENT '排序',
       PRIMARY KEY (`id`),
       KEY `ap_product_category_fk_4` (`parent_id`),
       UNIQUE KEY `ap_product_category_name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=1195 DEFAULT CHARSET=utf8mb4 COMMENT='商品分类';

CREATE TABLE `ap_product` (
      `id` bigint(20) NOT NULL AUTO_INCREMENT,
      `name` varchar(255) NOT NULL COMMENT '名称',
      `product_status` varchar(32) NOT NULL COMMENT '状态',
      `system_code` varchar(255) DEFAULT NULL COMMENT '编码',
      `supplier_code` varchar(255) DEFAULT NULL COMMENT '供应商编码',

      `main_img` varchar(500) DEFAULT NULL COMMENT '主图',
      `show_img` longtext DEFAULT NULL COMMENT '轮播图',
      `main_content` longtext DEFAULT NULL COMMENT '详情',

      `has_pdd_online` bit(1) DEFAULT b'0' COMMENT '拼多多',
      `pdd_id` varchar(255) DEFAULT NULL COMMENT '拼多多id',
      `pdd_edit_url` varchar(255) DEFAULT NULL COMMENT '拼多多id',
      `pdd_sell_count` int(10) DEFAULT '0' COMMENT '销量',
      `pdd_code` varchar(255) DEFAULT NULL COMMENT '商品编码',


      `category_one_id` bigint(20) DEFAULT NULL COMMENT '分类一id',
      `category_one_name` varchar(255) DEFAULT NULL COMMENT '分类一名称',
      `category_two_id` bigint(20) DEFAULT NULL COMMENT '分类二id',
      `category_two_name` varchar(255) DEFAULT NULL COMMENT '分类二名称',

      `unit_id` bigint(20) DEFAULT NULL COMMENT '单位id',
      `unit_name` varchar(255) DEFAULT NULL COMMENT '单位名称',
      `brand_id` bigint(20) DEFAULT NULL COMMENT '品牌id',
      `brand_name` varchar(255) DEFAULT NULL COMMENT '品牌名称',

      `price_min` decimal(8,2) DEFAULT '0.00' COMMENT '最低价',
      `price_max` decimal(8,2) DEFAULT '0.00' COMMENT '最高价',

      `create_by` varchar(64) DEFAULT '' COMMENT '创建者',
      `create_time` datetime DEFAULT NULL COMMENT '创建时间',
      `update_by` varchar(64) DEFAULT '' COMMENT '更新者',
      `update_time` datetime DEFAULT NULL COMMENT '更新时间',
      `remark` varchar(500) DEFAULT NULL COMMENT '备注',
      `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商id',
      `supplier_name` varchar(255) DEFAULT NULL COMMENT '供应商名称',
      PRIMARY KEY (`id`),
      UNIQUE KEY `ap_product_name` (`name`),
      UNIQUE KEY `ap_product_supplier_code` (`supplier_code`),
      UNIQUE KEY `ap_product_pdd_id` (`pdd_id`),
      UNIQUE KEY `ap_product_system_code` (`system_code`),
      UNIQUE KEY `ap_product_pdd_code` (`pdd_code`)
) ENGINE=InnoDB AUTO_INCREMENT=6866 DEFAULT CHARSET=utf8mb4 COMMENT='商品';




CREATE TABLE `ap_product_sku` (
      `id` bigint(20) NOT NULL AUTO_INCREMENT,
      `name` varchar(255) NOT NULL COMMENT '名称',
      `system_code` varchar(255) DEFAULT NULL COMMENT '编码',
      `product_id` bigint(20) DEFAULT NULL COMMENT '价目id',

      `cost_price` decimal(12,2) NOT NULL DEFAULT '0.00' COMMENT '进价',
      `retail_price` decimal(12,2) DEFAULT '0.00' COMMENT '零售价',
      `control_price` decimal(12,2) DEFAULT '0.00' COMMENT '分销控价',
      `marked_price` decimal(12,2) DEFAULT '0.00' COMMENT '标价',
      `min_price` decimal(12,2) DEFAULT '0.00' COMMENT '最低价',
      `profit` decimal(12,2) DEFAULT '0.00' COMMENT '利润',
      `express_price` decimal(12,2) DEFAULT '0.00' COMMENT '物流费用',

      `stock` int(10) DEFAULT '0' COMMENT '库存',
      `sell_count` int(10) DEFAULT '0' COMMENT '已售',
      `has_del` bit(1) DEFAULT b'0' COMMENT '标记删除',

      `search_cache` varchar(5000) DEFAULT NULL COMMENT '搜索缓存',
      `create_by` varchar(64) DEFAULT '' COMMENT '创建者',
      `create_time` datetime DEFAULT NULL COMMENT '创建时间',
      `update_by` varchar(64) DEFAULT '' COMMENT '更新者',
      `update_time` datetime DEFAULT NULL COMMENT '更新时间',
      `remark` varchar(500) DEFAULT NULL COMMENT '备注',
      UNIQUE KEY `ap_product_sku_name` (`product_id`,`name` ),
      UNIQUE KEY `ap_product_sku_code` (`product_id`,`system_code` ),

      PRIMARY KEY (`id`),

      KEY `repair_product_sku_fk_product_idx` (`product_id`),
      CONSTRAINT `repair_product_sku_fk_product` FOREIGN KEY (`product_id`) REFERENCES `ap_product` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=8003 DEFAULT CHARSET=utf8mb4 COMMENT='商品规格';

CREATE TABLE `ap_product_unit` (
   `id` bigint(20) NOT NULL AUTO_INCREMENT,
   `name` varchar(255) DEFAULT '' COMMENT '名称',
   `create_by` varchar(64) DEFAULT '' COMMENT '创建者',
   `create_time` datetime DEFAULT NULL COMMENT '创建时间',
   `update_by` varchar(64) DEFAULT '' COMMENT '更新者',
   `update_time` datetime DEFAULT NULL COMMENT '更新时间',
   `remark` varchar(500) DEFAULT NULL COMMENT '备注',
   PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=155 DEFAULT CHARSET=utf8mb4 COMMENT='单位';
