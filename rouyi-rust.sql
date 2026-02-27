/*
 Navicat Premium Data Transfer

 Source Server         : localhost_3306
 Source Server Type    : MySQL
 Source Server Version : 80044
 Source Host           : localhost:3306
 Source Schema         : rust

 Target Server Type    : MySQL
 Target Server Version : 80044
 File Encoding         : 65001

 Date: 27/02/2026 17:16:01
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for fa_feat
-- ----------------------------
DROP TABLE IF EXISTS `fa_feat`;
CREATE TABLE `fa_feat`  (
                            `feat_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'feat ID',
                            `feat_code` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'feat编码',
                            `feat_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'feat名称',
                            `feat_sort` int(0) NOT NULL COMMENT '显示顺序',
                            `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '状态（0正常 1停用）',
                            `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                            `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                            `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                            `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                            `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                            PRIMARY KEY (`feat_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = 'FEAT信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_feat
-- ----------------------------
INSERT INTO `fa_feat` VALUES ('69a15e1a002a0c2c00000002', 'asdfa', '3213245', 1, '1', 'admin', '2026-02-27 17:04:26', '', NULL, NULL);

-- ----------------------------
-- Table structure for fa_pan
-- ----------------------------
DROP TABLE IF EXISTS `fa_pan`;
CREATE TABLE `fa_pan`  (
                           `pan_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'pan ID',
                           `pan_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'pan账号',
                           `pan_nick_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'pan昵称',
                           `user_type` varchar(2) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '00' COMMENT 'pan类型（00系统用户）',
                           `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT 'pan邮箱',
                           `phonenumber` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '手机号码',
                           `sex` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '用户性别（0男 1女 2未知）',
                           `avatar` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '头像地址',
                           `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '帐号状态（0正常 1停用）',
                           `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
                           `login_date` datetime(0) NULL DEFAULT NULL COMMENT '最后登录时间',
                           `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                           `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                           `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                           `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                           `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                           PRIMARY KEY (`pan_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = 'PAN信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_pan
-- ----------------------------
INSERT INTO `fa_pan` VALUES ('69a15e07002a0c2c00000000', 'asdf', 'asdfasdf', '00', '', '', '0', '', '0', '0', NULL, 'admin', '2026-02-27 17:04:07', 'admin', '2026-02-27 17:04:51', NULL);

-- ----------------------------
-- Table structure for fa_pan_feat
-- ----------------------------
DROP TABLE IF EXISTS `fa_pan_feat`;
CREATE TABLE `fa_pan_feat`  (
                                `pan_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'pan ID',
                                `feat_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'feat ID',
                                PRIMARY KEY (`pan_id`, `feat_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = 'PAN与FEAT关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_pan_feat
-- ----------------------------
INSERT INTO `fa_pan_feat` VALUES ('69a15e07002a0c2c00000000', '69a15e1a002a0c2c00000002');

-- ----------------------------
-- Table structure for fa_renwu
-- ----------------------------
DROP TABLE IF EXISTS `fa_renwu`;
CREATE TABLE `fa_renwu`  (
                             `rw_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
                             `rw_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户账号',
                             `rw_nick_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户昵称',
                             `user_type` varchar(2) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '00' COMMENT '用户类型（00系统用户）',
                             `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '用户邮箱',
                             `phonenumber` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '手机号码',
                             `sex` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '用户性别（0男 1女 2未知）',
                             `avatar` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '头像地址',
                             `password` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '密码',
                             `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '帐号状态（0正常 1停用）',
                             `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
                             `login_date` datetime(0) NULL DEFAULT NULL COMMENT '最后登录时间',
                             `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                             `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                             `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                             `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                             `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                             PRIMARY KEY (`rw_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '人物表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_renwu
-- ----------------------------
INSERT INTO `fa_renwu` VALUES ('69a10a61008190e000000000', 'aserqwer', 'qwer', '00', '', '', '0', '', '', '0', '0', NULL, 'admin', '2026-02-27 11:07:13', 'admin', '2026-02-27 11:07:23', NULL);

-- ----------------------------
-- Table structure for fa_renwu_exp
-- ----------------------------
DROP TABLE IF EXISTS `fa_renwu_exp`;
CREATE TABLE `fa_renwu_exp`  (
                                 `rw_exp_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '主键',
                                 `p_id` varbinary(32) NOT NULL COMMENT '用户id',
                                 `date_range` varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '时间区域',
                                 `unit` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '单位',
                                 `post` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '职位',
                                 `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL DEFAULT '' COMMENT '创建者',
                                 `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                                 `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL DEFAULT '' COMMENT '更新者',
                                 `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                                 `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL DEFAULT NULL COMMENT '备注',
                                 PRIMARY KEY (`rw_exp_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '人物经历表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_renwu_exp
-- ----------------------------
INSERT INTO `fa_renwu_exp` VALUES ('69a10a61008190e000000001', 0x363961313061363130303831393065303030303030303030, 'asdf', 'asdf', 'asdfasfd', 'admin', '2026-02-27 11:07:13', 'admin', '2026-02-27 11:07:23', 'asdfasedqwe');
INSERT INTO `fa_renwu_exp` VALUES ('69a10a6b008190e000000003', 0x363961313061363130303831393065303030303030303030, '2333', '1234', '1234', 'admin', '2026-02-27 11:07:23', '', NULL, 'qw3');

-- ----------------------------
-- Table structure for fa_tree
-- ----------------------------
DROP TABLE IF EXISTS `fa_tree`;
CREATE TABLE `fa_tree`  (
                            `acct_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '主键',
                            `acc_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '名称',
                            `root_type` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '根类型',
                            `parent_account` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '父节点名称',
                            `account_type` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '科目类型',
                            `is_group` bit(1) NULL DEFAULT NULL COMMENT '是否分组',
                            `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '创建者',
                            `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                            `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '更新者',
                            `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                            `lft` int(0) NULL DEFAULT NULL COMMENT '未知1',
                            `rgt` int(0) NULL DEFAULT NULL COMMENT '未知2',
                            PRIMARY KEY (`acct_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '树形科目' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_tree
-- ----------------------------
INSERT INTO `fa_tree` VALUES ('69a022ed0052bed000000022', 'aseqwer', NULL, '0', NULL, b'1', 'admin', '2026-02-26 18:39:41', NULL, NULL, NULL, NULL);
INSERT INTO `fa_tree` VALUES ('69a0257300352f0700000000', 'ASD', NULL, '69a022ed0052bed000000022', NULL, NULL, 'admin', '2026-02-26 18:50:27', NULL, NULL, NULL, NULL);
INSERT INTO `fa_tree` VALUES ('69a02753001f56570000000a', 'ZASDF', NULL, '69a022ed0052bed000000022', '', b'0', 'admin', '2026-02-26 18:58:27', NULL, NULL, 1, 3);
INSERT INTO `fa_tree` VALUES ('69a027b5001f56570000000c', 'eeee', NULL, '0', NULL, NULL, 'admin', '2026-02-26 19:00:05', NULL, NULL, NULL, NULL);
INSERT INTO `fa_tree` VALUES ('69a03734006e6a2000000050', 'qqwer', '', '69a022ed0052bed000000022', NULL, NULL, 'admin', '2026-02-26 20:06:12', NULL, NULL, NULL, NULL);
INSERT INTO `fa_tree` VALUES ('69a0e840006e6a20000000b8', '阿斯蒂芬', NULL, '69a027b5001f56570000000c', NULL, NULL, 'admin', '2026-02-27 08:41:36', NULL, NULL, NULL, NULL);

-- ----------------------------
-- Table structure for fa_weifa
-- ----------------------------
DROP TABLE IF EXISTS `fa_weifa`;
CREATE TABLE `fa_weifa`  (
                             `wf_id` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '主键',
                             `case_no` char(12) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '案件编号',
                             `case_level` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '入库级别',
                             `deal_date` date NOT NULL COMMENT '查处时间',
                             `happen_datetime` datetime(0) NULL DEFAULT NULL COMMENT '违建时间',
                             `wf_address` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '违建地址',
                             `wf_structure` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '建筑结构',
                             `cstt_area` float(6, 2) NOT NULL COMMENT '违建面积',
  `land_usage` char(5) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '用地类型',
  `land_area` float(6, 2) NULL DEFAULT NULL COMMENT '占地面积',
  `party_name` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '当事人姓名',
  `party_id` char(18) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '身份证号码',
  `party_contact` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '联系方式',
  `party_address` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '家庭住址',
  `party_unit` varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '单位名称',
  `cstt_exec_status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '执行情况',
  `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '0' COMMENT '删除标志',
  `cstt_docs` json NULL COMMENT '文件',
  `cstt_pictures` json NULL COMMENT '照片',
  `notice_no` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '通报单编号',
  `notice_date` date NULL DEFAULT NULL COMMENT '通报日期',
  `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '创建单位',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '状态',
  `town_reported` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '是否已报告镇街',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
  `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
  `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '备注信息',
  PRIMARY KEY (`wf_id`) USING BTREE,
  UNIQUE INDEX `case_no_idx`(`case_no`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '违法' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_weifa
-- ----------------------------
INSERT INTO `fa_weifa` VALUES ('69a1423700e9c65d00000002', 'Z001', '0', '2026-02-14', '2026-02-12 00:00:00', 'dhrtdtre', 'ert', 3.00, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, '0', NULL, NULL, NULL, NULL, NULL, NULL, '0', 'admin', '2026-02-27 15:05:27', 'admin', '2026-02-27 15:11:59', '');

-- ----------------------------
-- Table structure for fa_weifa_item
-- ----------------------------
DROP TABLE IF EXISTS `fa_weifa_item`;
CREATE TABLE `fa_weifa_item`  (
                                  `wf_item_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '主键',
                                  `apply_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '申请人姓名',
                                  `apply_id` char(18) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '申请人身份证',
                                  `apply_addr` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '申请人住址',
                                  `apply_unit` varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '申请人单位名称',
                                  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                                  `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                                  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                                  `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                                  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '备注信息',
                                  PRIMARY KEY (`wf_item_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '违法扩展' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of fa_weifa_item
-- ----------------------------
INSERT INTO `fa_weifa_item` VALUES ('69a1423700e9c65d00000002', 'acasdrqwer', 'asdf', 'asdf', 'qwr1wq', '', NULL, '', NULL, '');

-- ----------------------------
-- Table structure for gen_table
-- ----------------------------
DROP TABLE IF EXISTS `gen_table`;
CREATE TABLE `gen_table`  (
                              `table_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '编号',
                              `table_name` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '表名称',
                              `table_comment` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '表描述',
                              `sub_table_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '关联子表的表名',
                              `sub_table_fk_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '子表关联的外键名',
                              `struct_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '实体类名称',
                              `tpl_category` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT 'crud' COMMENT '使用的模板（crud单表操作 tree树表操作）',
                              `tpl_web_type` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '前端模板类型（element-ui模版 element-plus模版）',
                              `tpl_back_type` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '后台模板类型（rust java）',
                              `package_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '生成包路径',
                              `module_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '生成模块名',
                              `business_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '生成业务名',
                              `function_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '生成功能名',
                              `function_author` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '生成功能作者',
                              `gen_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '生成代码方式（0zip压缩包 1自定义路径）',
                              `gen_path_web` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '前端生成路径（不填默认项目路径）',
                              `gen_path_back` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '/' COMMENT '后台生成路径（不填默认项目路径）',
                              `options` json NULL COMMENT '其它生成选项',
                              `switch_opt` json NULL COMMENT '开关选项',
                              `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                              `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                              `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                              `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                              `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                              PRIMARY KEY (`table_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '代码生成业务表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of gen_table
-- ----------------------------
INSERT INTO `gen_table` VALUES ('69a0023900fa876d00000001', 'fa_tree', '树形科目', NULL, NULL, 'FaTree', 'tree', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'tree', '树形科目', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"treeCode\": \"acct_id\", \"treeName\": \"acc_name\", \"parentMenuId\": \"6981e9a700349acd00000000\", \"treeParentCode\": \"parent_account\"}', '{\"notExport\": true}', 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37', NULL);
INSERT INTO `gen_table` VALUES ('69a0ee1d006e6a20000000ba', 'fa_renwu', '人物表', NULL, NULL, 'FaRenwu', 'o2m', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'renwu', '人物', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"subTableName\": \"fa_renwu_exp\", \"subTableColName\": \"p_id\"}', '{}', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07', NULL);
INSERT INTO `gen_table` VALUES ('69a0ee1d006e6a20000000cc', 'fa_renwu_exp', '人物经历表', NULL, NULL, 'FaRenwuExp', 'subTable', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'renwuExp', '人物经历', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"parentMenuId\": \"6981e9a700349acd00000000\", \"subTableName\": \"fa_renwu\", \"mainTableDisLabel\": \"用户账号\", \"mainTableDisColName\": \"rw_name\"}', '{\"notInsert\": true}', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08', NULL);
INSERT INTO `gen_table` VALUES ('69a1229e00b848fe00000011', 'fa_weifa', '违法', NULL, NULL, 'FaWeifa', 'o2o', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'weifa', '违法', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"parentMenuId\": \"6981e9a700349acd00000000\", \"subTableName\": \"fa_weifa_item\", \"subTableColName\": \"wf_item_id\"}', '{}', 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16', NULL);
INSERT INTO `gen_table` VALUES ('69a122a400b848fe0000002f', 'fa_weifa_item', '违法扩展', NULL, NULL, 'FaWeifaItem', 'subTable', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'weifaItem', '违法扩展', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"parentMenuId\": \"6981e9a700349acd00000000\", \"subTableName\": \"fa_weifa\", \"mainTableDisLabel\": \"案件编号\", \"mainTableDisColName\": \"case_no\"}', '{\"notInsert\": true}', 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18', NULL);
INSERT INTO `gen_table` VALUES ('69a144b600ee053e00000028', 'fa_pan', 'PAN信息表', NULL, NULL, 'FaPan', 'm2m', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'pan', 'PAN信息', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"subTableName\": \"fa_feat\", \"joinTableName\": \"fa_pan_feat\", \"joinLeftColName\": \"pan_id\", \"subTableDisColName\": \"feat_name\", \"joinSubTableColName\": \"feat_id\"}', '{}', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20', NULL);
INSERT INTO `gen_table` VALUES ('69a144bd00ee053e00000039', 'fa_feat', 'FEAT信息表', NULL, NULL, 'FaFeat', 'm2m', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'feat', 'FEAT信息', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"subTableName\": \"fa_pan\", \"joinTableName\": \"fa_pan_feat\", \"joinLeftColName\": \"feat_id\", \"subTableDisColName\": \"pan_name\", \"joinSubTableColName\": \"pan_id\"}', '{}', 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47', NULL);
INSERT INTO `gen_table` VALUES ('69a144c200ee053e00000044', 'fa_pan_feat', 'PAN与FEAT关联表', NULL, NULL, 'FaPanFeat', 'join', 'element-plus', 'rust', 'com.ruoyi.oa', 'fa', 'panFeat', 'PAN与FEAT关联', 'wizount', '1', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-vue3-rust\\src', 'D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\src\\modules', '{\"parentMenuId\": \"6981e9a700349acd00000000\"}', '{}', 'admin', '2026-02-27 15:16:18', 'admin', '2026-02-27 15:17:46', NULL);

-- ----------------------------
-- Table structure for gen_table_column
-- ----------------------------
DROP TABLE IF EXISTS `gen_table_column`;
CREATE TABLE `gen_table_column`  (
                                     `column_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '编号',
                                     `table_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '归属表编号',
                                     `column_name` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '列名称',
                                     `column_comment` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '列描述',
                                     `column_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '列类型',
                                     `rust_type` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT 'rust类型',
                                     `rust_field` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT 'rust字段名',
                                     `is_pk` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否主键（1是）',
                                     `is_increment` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否自增（1是）',
                                     `is_required` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否必填（1是）',
                                     `is_insert` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否为插入字段（1是）',
                                     `is_edit` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否编辑字段（1是）',
                                     `is_list` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否列表字段（1是）',
                                     `is_table` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否表格显示字段（1是）',
                                     `is_detail` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否详情字段（1是）',
                                     `is_export` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '是否导出字段（1是）',
                                     `is_sortable` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '是否排序字段（1是）',
                                     `is_query` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否查询字段（1是）',
                                     `query_type` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT 'EQ' COMMENT '查询方式（等于、不等于、大于、小于、范围）',
                                     `html_type` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '显示类型（文本框、文本域、下拉框、复选框、单选框、日期控件）',
                                     `dict_type` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '字典类型',
                                     `sort` int(0) NULL DEFAULT NULL COMMENT '排序',
                                     `more` json NULL COMMENT '正则表达式提示消息',
                                     `def_val` varchar(400) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '默认值',
                                     `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                                     `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                                     `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                                     `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                                     PRIMARY KEY (`column_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '代码生成业务表字段' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of gen_table_column
-- ----------------------------
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000002', '69a0023900fa876d00000001', 'acct_id', '主键', 'varchar(32)', 'String', 'acct_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000003', '69a0023900fa876d00000001', 'acc_name', '名称', 'varchar(50)', 'String', 'acc_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000004', '69a0023900fa876d00000001', 'root_type', '根类型', 'varchar(50)', 'String', 'root_type', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'select', '', 3, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000005', '69a0023900fa876d00000001', 'parent_account', '父节点名称', 'varchar(32)', 'String', 'parent_account', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '1', 'EQ', 'input', '', 4, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000006', '69a0023900fa876d00000001', 'account_type', '科目类型', 'varchar(20)', 'String', 'account_type', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'select', '', 5, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000007', '69a0023900fa876d00000001', 'is_group', '是否分组', 'bit(1)', 'bool', 'is_group', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'radio', '', 6, NULL, NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000008', '69a0023900fa876d00000001', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 7, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d00000009', '69a0023900fa876d00000001', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 8, NULL, NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d0000000a', '69a0023900fa876d00000001', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 9, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d0000000b', '69a0023900fa876d00000001', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 10, NULL, NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d0000000c', '69a0023900fa876d00000001', 'lft', '未知1', 'int', 'i32', 'lft', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'number', '', 11, NULL, NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0023900fa876d0000000d', '69a0023900fa876d00000001', 'rgt', '未知2', 'int', 'i32', 'rgt', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'number', '', 12, NULL, NULL, 'admin', '2026-02-26 16:20:09', 'admin', '2026-02-27 08:22:37');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000bb', '69a0ee1d006e6a20000000ba', 'rw_id', '用户ID', 'varchar(32)', 'String', 'rw_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000bc', '69a0ee1d006e6a20000000ba', 'rw_name', '用户账号', 'varchar(30)', 'String', 'rw_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000bd', '69a0ee1d006e6a20000000ba', 'rw_nick_name', '用户昵称', 'varchar(30)', 'String', 'rw_nick_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 3, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000be', '69a0ee1d006e6a20000000ba', 'user_type', '用户类型（00系统用户）', 'varchar(2)', 'String', 'user_type', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'select', '', 4, '{\"checkLength\": \"1\"}', '00', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000bf', '69a0ee1d006e6a20000000ba', 'email', '用户邮箱', 'varchar(50)', 'String', 'email', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 5, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c0', '69a0ee1d006e6a20000000ba', 'phonenumber', '手机号码', 'varchar(11)', 'String', 'phonenumber', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 6, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c1', '69a0ee1d006e6a20000000ba', 'sex', '用户性别（0男 1女 2未知）', 'char(1)', 'char', 'sex', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'select', '', 7, NULL, '0', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c2', '69a0ee1d006e6a20000000ba', 'avatar', '头像地址', 'varchar(100)', 'String', 'avatar', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 8, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c3', '69a0ee1d006e6a20000000ba', 'password', '密码', 'varchar(100)', 'String', 'password', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 9, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c4', '69a0ee1d006e6a20000000ba', 'status', '帐号状态（0正常 1停用）', 'char(1)', 'char', 'status', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'radio', '', 10, NULL, '0', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c5', '69a0ee1d006e6a20000000ba', 'del_flag', '删除标志（0代表存在 2代表删除）', 'char(1)', 'char', 'del_flag', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 11, NULL, '0', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c6', '69a0ee1d006e6a20000000ba', 'login_date', '最后登录时间', 'datetime', 'DateTime', 'login_date', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'datetime', '', 12, NULL, NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c7', '69a0ee1d006e6a20000000ba', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 13, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c8', '69a0ee1d006e6a20000000ba', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 14, NULL, NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000c9', '69a0ee1d006e6a20000000ba', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 15, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000ca', '69a0ee1d006e6a20000000ba', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 16, NULL, NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000cb', '69a0ee1d006e6a20000000ba', 'remark', '备注', 'varchar(500)', 'String', 'remark', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 17, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:46:07');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000cd', '69a0ee1d006e6a20000000cc', 'rw_exp_id', '主键', 'varchar(32)', 'String', 'rw_exp_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000ce', '69a0ee1d006e6a20000000cc', 'p_id', '用户id', 'varbinary(32)', 'String', 'p_id', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 2, NULL, NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000cf', '69a0ee1d006e6a20000000cc', 'date_range', '时间区域', 'varchar(40)', 'String', 'date_range', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 3, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d0', '69a0ee1d006e6a20000000cc', 'unit', '单位', 'varchar(200)', 'String', 'unit', '0', '0', '1', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 4, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d1', '69a0ee1d006e6a20000000cc', 'post', '职位', 'varchar(20)', 'String', 'post', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 5, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d2', '69a0ee1d006e6a20000000cc', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 6, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d3', '69a0ee1d006e6a20000000cc', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 7, NULL, NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d4', '69a0ee1d006e6a20000000cc', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 8, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d5', '69a0ee1d006e6a20000000cc', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 9, NULL, NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a0ee1d006e6a20000000d6', '69a0ee1d006e6a20000000cc', 'remark', '备注', 'varchar(500)', 'String', 'remark', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 10, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 09:06:37', 'admin', '2026-02-27 10:21:08');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000012', '69a1229e00b848fe00000011', 'wf_id', '主键', 'char(32)', 'String', 'wf_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000013', '69a1229e00b848fe00000011', 'case_no', '案件编号', 'char(12)', 'String', 'case_no', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000014', '69a1229e00b848fe00000011', 'case_level', '入库级别', 'char(1)', 'char', 'case_level', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 3, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000015', '69a1229e00b848fe00000011', 'deal_date', '查处时间', 'date', 'Date', 'deal_date', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'date', '', 4, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000016', '69a1229e00b848fe00000011', 'happen_datetime', '违建时间', 'datetime', 'DateTime', 'happen_datetime', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'datetime', '', 5, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000017', '69a1229e00b848fe00000011', 'wf_address', '违建地址', 'varchar(500)', 'String', 'wf_address', '0', '0', '1', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 6, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000018', '69a1229e00b848fe00000011', 'wf_structure', '建筑结构', 'varchar(30)', 'String', 'wf_structure', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 7, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000019', '69a1229e00b848fe00000011', 'cstt_area', '违建面积', 'float(6,2)', 'f64', 'cstt_area', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'number', '', 8, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000001a', '69a1229e00b848fe00000011', 'land_usage', '用地类型', 'char(5)', 'String', 'land_usage', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 9, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000001b', '69a1229e00b848fe00000011', 'land_area', '占地面积', 'float(6,2)', 'f64', 'land_area', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'number', '', 10, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000001c', '69a1229e00b848fe00000011', 'party_name', '当事人姓名', 'varchar(20)', 'String', 'party_name', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 11, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000001d', '69a1229e00b848fe00000011', 'party_id', '身份证号码', 'char(18)', 'String', 'party_id', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 12, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000001e', '69a1229e00b848fe00000011', 'party_contact', '联系方式', 'varchar(20)', 'String', 'party_contact', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 13, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000001f', '69a1229e00b848fe00000011', 'party_address', '家庭住址', 'varchar(500)', 'String', 'party_address', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 14, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000020', '69a1229e00b848fe00000011', 'party_unit', '单位名称', 'varchar(40)', 'String', 'party_unit', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 15, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000021', '69a1229e00b848fe00000011', 'cstt_exec_status', '执行情况', 'char(1)', 'char', 'cstt_exec_status', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'radio', '', 16, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000022', '69a1229e00b848fe00000011', 'del_flag', '删除标志', 'char(1)', 'char', 'del_flag', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 17, NULL, '0', 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000023', '69a1229e00b848fe00000011', 'cstt_docs', '文件', 'json', 'serde_json::Value', 'cstt_docs', '0', '0', '0', '1', '1', '1', '1', '1', '0', '0', '0', 'EQ', 'input', '', 18, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000024', '69a1229e00b848fe00000011', 'cstt_pictures', '照片', 'json', 'serde_json::Value', 'cstt_pictures', '0', '0', '0', '1', '1', '1', '1', '1', '0', '0', '0', 'EQ', 'imageUpload', '', 19, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000025', '69a1229e00b848fe00000011', 'notice_no', '通报单编号', 'varchar(20)', 'String', 'notice_no', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 20, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000026', '69a1229e00b848fe00000011', 'notice_date', '通报日期', 'date', 'Date', 'notice_date', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'date', '', 21, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000027', '69a1229e00b848fe00000011', 'dept_id', '创建单位', 'varchar(32)', 'String', 'dept_id', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 22, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000028', '69a1229e00b848fe00000011', 'status', '状态', 'char(1)', 'char', 'status', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'radio', '', 23, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe00000029', '69a1229e00b848fe00000011', 'town_reported', '是否已报告镇街', 'char(1)', 'char', 'town_reported', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 24, NULL, '0', 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000002a', '69a1229e00b848fe00000011', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 25, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000002b', '69a1229e00b848fe00000011', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 26, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000002c', '69a1229e00b848fe00000011', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 27, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000002d', '69a1229e00b848fe00000011', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 28, NULL, NULL, 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a1229e00b848fe0000002e', '69a1229e00b848fe00000011', 'remark', '备注信息', 'varchar(500)', 'String', 'remark', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 29, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 12:50:38', 'admin', '2026-02-27 13:01:16');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000030', '69a122a400b848fe0000002f', 'wf_item_id', '主键', 'varchar(32)', 'String', 'wf_item_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000031', '69a122a400b848fe0000002f', 'apply_name', '申请人姓名', 'varchar(100)', 'String', 'apply_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000032', '69a122a400b848fe0000002f', 'apply_id', '申请人身份证', 'char(18)', 'String', 'apply_id', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 3, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000033', '69a122a400b848fe0000002f', 'apply_addr', '申请人住址', 'varchar(255)', 'String', 'apply_addr', '0', '0', '1', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 4, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000034', '69a122a400b848fe0000002f', 'apply_unit', '申请人单位名称', 'varchar(40)', 'String', 'apply_unit', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 5, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000035', '69a122a400b848fe0000002f', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 6, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000036', '69a122a400b848fe0000002f', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 7, NULL, NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000037', '69a122a400b848fe0000002f', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 8, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000038', '69a122a400b848fe0000002f', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 9, NULL, NULL, 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a122a400b848fe00000039', '69a122a400b848fe0000002f', 'remark', '备注信息', 'varchar(500)', 'String', 'remark', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 10, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 12:50:44', 'admin', '2026-02-27 15:06:18');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000029', '69a144b600ee053e00000028', 'pan_id', 'pan ID', 'varchar(32)', 'String', 'pan_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e0000002a', '69a144b600ee053e00000028', 'pan_name', 'pan账号', 'varchar(30)', 'String', 'pan_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e0000002b', '69a144b600ee053e00000028', 'pan_nick_name', 'pan昵称', 'varchar(30)', 'String', 'pan_nick_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 3, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e0000002c', '69a144b600ee053e00000028', 'user_type', 'pan类型（00系统用户）', 'varchar(2)', 'String', 'user_type', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'select', '', 4, '{\"checkLength\": \"1\"}', '00', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e0000002d', '69a144b600ee053e00000028', 'email', 'pan邮箱', 'varchar(50)', 'String', 'email', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 5, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e0000002e', '69a144b600ee053e00000028', 'phonenumber', '手机号码', 'varchar(11)', 'String', 'phonenumber', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 6, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e0000002f', '69a144b600ee053e00000028', 'sex', '用户性别（0男 1女 2未知）', 'char(1)', 'char', 'sex', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'select', '', 7, NULL, '0', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000030', '69a144b600ee053e00000028', 'avatar', '头像地址', 'varchar(100)', 'String', 'avatar', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 8, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000031', '69a144b600ee053e00000028', 'status', '帐号状态（0正常 1停用）', 'char(1)', 'char', 'status', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'radio', '', 9, NULL, '0', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000032', '69a144b600ee053e00000028', 'del_flag', '删除标志（0代表存在 2代表删除）', 'char(1)', 'char', 'del_flag', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 10, NULL, '0', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000033', '69a144b600ee053e00000028', 'login_date', '最后登录时间', 'datetime', 'DateTime', 'login_date', '0', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'datetime', '', 11, NULL, NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000034', '69a144b600ee053e00000028', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 12, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000035', '69a144b600ee053e00000028', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 13, NULL, NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000036', '69a144b600ee053e00000028', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 14, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000037', '69a144b600ee053e00000028', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 15, NULL, NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144b600ee053e00000038', '69a144b600ee053e00000028', 'remark', '备注', 'varchar(500)', 'String', 'remark', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 16, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:06', 'admin', '2026-02-27 15:19:20');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e0000003a', '69a144bd00ee053e00000039', 'feat_id', 'feat ID', 'varchar(32)', 'String', 'feat_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e0000003b', '69a144bd00ee053e00000039', 'feat_code', 'feat编码', 'varchar(64)', 'String', 'feat_code', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e0000003c', '69a144bd00ee053e00000039', 'feat_name', 'feat名称', 'varchar(50)', 'String', 'feat_name', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'LIKE', 'input', '', 3, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e0000003d', '69a144bd00ee053e00000039', 'feat_sort', '显示顺序', 'int', 'i32', 'feat_sort', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'BETWEEN', 'number', '', 4, NULL, NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e0000003e', '69a144bd00ee053e00000039', 'status', '状态（0正常 1停用）', 'char(1)', 'char', 'status', '0', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', 'EQ', 'radio', '', 5, NULL, NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e0000003f', '69a144bd00ee053e00000039', 'create_by', '创建者', 'varchar(64)', 'String', 'create_by', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 6, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e00000040', '69a144bd00ee053e00000039', 'create_time', '创建时间', 'datetime', 'DateTime', 'create_time', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 7, NULL, NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e00000041', '69a144bd00ee053e00000039', 'update_by', '更新者', 'varchar(64)', 'String', 'update_by', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 8, '{\"checkLength\": \"1\"}', '', 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e00000042', '69a144bd00ee053e00000039', 'update_time', '更新时间', 'datetime', 'DateTime', 'update_time', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', 'BETWEEN', 'datetime', '', 9, NULL, NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144bd00ee053e00000043', '69a144bd00ee053e00000039', 'remark', '备注', 'varchar(500)', 'String', 'remark', '0', '0', '0', '1', '1', '0', '0', '1', '1', '0', '0', 'LIKE', 'textarea', '', 10, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:13', 'admin', '2026-02-27 15:19:47');
INSERT INTO `gen_table_column` VALUES ('69a144c200ee053e00000045', '69a144c200ee053e00000044', 'pan_id', 'pan ID', 'varchar(32)', 'String', 'pan_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 1, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:18', 'admin', '2026-02-27 15:17:46');
INSERT INTO `gen_table_column` VALUES ('69a144c200ee053e00000046', '69a144c200ee053e00000044', 'feat_id', 'feat ID', 'varchar(32)', 'String', 'feat_id', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', 'EQ', 'input', '', 2, '{\"checkLength\": \"1\"}', NULL, 'admin', '2026-02-27 15:16:18', 'admin', '2026-02-27 15:17:46');

-- ----------------------------
-- Table structure for sys_config
-- ----------------------------
DROP TABLE IF EXISTS `sys_config`;
CREATE TABLE `sys_config`  (
                               `config_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '参数主键',
                               `config_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '参数名称',
                               `config_key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '参数键名',
                               `config_value` varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '参数键值',
                               `config_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT 'N' COMMENT '系统内置（Y是 N否）',
                               `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                               `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                               `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                               `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                               `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                               PRIMARY KEY (`config_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '参数配置表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_config
-- ----------------------------
INSERT INTO `sys_config` VALUES ('1', '主框架页-默认皮肤样式名称', 'sys.index.skinName', 'skin-blue', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '蓝色 skin-blue、绿色 skin-green、紫色 skin-purple、红色 skin-red、黄色 skin-yellow');
INSERT INTO `sys_config` VALUES ('2', '用户管理-账号初始密码', 'sys.user.initPassword', '123456', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '初始化密码 123456');
INSERT INTO `sys_config` VALUES ('3', '主框架页-侧边栏主题', 'sys.index.sideTheme', 'theme-dark', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '深色主题theme-dark，浅色主题theme-light');
INSERT INTO `sys_config` VALUES ('4', '账号自助-验证码开关', 'sys.account.captchaEnabled', 'true', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '是否开启验证码功能（true开启，false关闭）');
INSERT INTO `sys_config` VALUES ('5', '账号自助-是否开启用户注册功能', 'sys.account.registerUser', 'false', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '是否开启注册用户功能（true开启，false关闭）');
INSERT INTO `sys_config` VALUES ('6', '用户登录-黑名单列表', 'sys.login.blackIPList', '', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '设置登录IP黑名单限制，多个匹配项以;分隔，支持匹配（*通配、网段）');
INSERT INTO `sys_config` VALUES ('7', '用户管理-初始密码修改策略', 'sys.account.initPasswordModify', '1', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '0：初始密码修改策略关闭，没有任何提示，1：提醒用户，如果未修改初始密码，则在登录时就会提醒修改密码对话框');
INSERT INTO `sys_config` VALUES ('8', '用户管理-账号密码更新周期', 'sys.account.passwordValidateDays', '0', 'Y', 'admin', '2026-01-12 20:08:38', '', NULL, '密码更新周期（填写数字，数据初始化值为0不限制，若修改必须为大于0小于365的正整数），如果超过这个周期登录系统时，则在登录时就会提醒修改密码对话框');

-- ----------------------------
-- Table structure for sys_dept
-- ----------------------------
DROP TABLE IF EXISTS `sys_dept`;
CREATE TABLE `sys_dept`  (
                             `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '部门id',
                             `parent_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '父部门id',
                             `ancestors` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '祖级列表',
                             `dept_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '部门名称',
                             `order_num` int(0) NULL DEFAULT 0 COMMENT '显示顺序',
                             `leader` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '负责人',
                             `phone` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '联系电话',
                             `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '邮箱',
                             `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '部门状态（0正常 1停用）',
                             `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
                             `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                             `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                             `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                             `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                             PRIMARY KEY (`dept_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '部门表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_dept
-- ----------------------------
INSERT INTO `sys_dept` VALUES ('100', '0', '0', '若依科技', 0, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('101', '100', '0,100', '深圳总公司', 1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('102', '100', '0,100', '长沙分公司', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('103', '101', '0,100,101', '研发部门', 1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('104', '101', '0,100,101', '市场部门', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('105', '101', '0,100,101', '测试部门', 3, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('106', '101', '0,100,101', '财务部门', 4, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('107', '101', '0,100,101', '运维部门', 5, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('108', '102', '0,100,102', '市场部门', 1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);
INSERT INTO `sys_dept` VALUES ('109', '102', '0,100,102', '财务部门', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2026-01-12 20:01:33', '', NULL);

-- ----------------------------
-- Table structure for sys_dict_data
-- ----------------------------
DROP TABLE IF EXISTS `sys_dict_data`;
CREATE TABLE `sys_dict_data`  (
                                  `dict_code` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '字典编码',
                                  `dict_sort` int(0) NULL DEFAULT 0 COMMENT '字典排序',
                                  `dict_label` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '字典标签',
                                  `dict_value` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '字典键值',
                                  `dict_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '字典类型',
                                  `css_class` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '样式属性（其他样式扩展）',
                                  `list_class` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '表格回显样式',
                                  `is_default` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT 'N' COMMENT '是否默认（Y是 N否）',
                                  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
                                  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                                  `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                                  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                                  `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                                  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                                  PRIMARY KEY (`dict_code`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '字典数据表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_dict_data
-- ----------------------------
INSERT INTO `sys_dict_data` VALUES ('1', 1, '男', '0', 'sys_user_sex', '', '', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '性别男');
INSERT INTO `sys_dict_data` VALUES ('10', 1, '默认', 'DEFAULT', 'sys_job_group', '', '', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '默认分组');
INSERT INTO `sys_dict_data` VALUES ('11', 2, '系统', 'SYSTEM', 'sys_job_group', '', '', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '系统分组');
INSERT INTO `sys_dict_data` VALUES ('12', 1, '是', 'Y', 'sys_yes_no', '', 'primary', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '系统默认是');
INSERT INTO `sys_dict_data` VALUES ('13', 2, '否', 'N', 'sys_yes_no', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '系统默认否');
INSERT INTO `sys_dict_data` VALUES ('14', 1, '通知', '1', 'sys_notice_type', '', 'warning', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '通知');
INSERT INTO `sys_dict_data` VALUES ('15', 2, '公告', '2', 'sys_notice_type', '', 'success', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '公告');
INSERT INTO `sys_dict_data` VALUES ('16', 1, '正常', '0', 'sys_notice_status', '', 'primary', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '正常状态');
INSERT INTO `sys_dict_data` VALUES ('17', 2, '关闭', '1', 'sys_notice_status', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '关闭状态');
INSERT INTO `sys_dict_data` VALUES ('18', 99, '其他', '0', 'sys_oper_type', '', 'info', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '其他操作');
INSERT INTO `sys_dict_data` VALUES ('19', 1, '新增', '1', 'sys_oper_type', '', 'info', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '新增操作');
INSERT INTO `sys_dict_data` VALUES ('2', 2, '女', '1', 'sys_user_sex', '', '', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '性别女');
INSERT INTO `sys_dict_data` VALUES ('20', 2, '修改', '2', 'sys_oper_type', '', 'info', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '修改操作');
INSERT INTO `sys_dict_data` VALUES ('21', 3, '删除', '3', 'sys_oper_type', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '删除操作');
INSERT INTO `sys_dict_data` VALUES ('22', 4, '授权', '4', 'sys_oper_type', '', 'primary', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '授权操作');
INSERT INTO `sys_dict_data` VALUES ('23', 5, '导出', '5', 'sys_oper_type', '', 'warning', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '导出操作');
INSERT INTO `sys_dict_data` VALUES ('24', 6, '导入', '6', 'sys_oper_type', '', 'warning', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '导入操作');
INSERT INTO `sys_dict_data` VALUES ('25', 7, '强退', '7', 'sys_oper_type', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '强退操作');
INSERT INTO `sys_dict_data` VALUES ('26', 8, '生成代码', '8', 'sys_oper_type', '', 'warning', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '生成操作');
INSERT INTO `sys_dict_data` VALUES ('27', 9, '清空数据', '9', 'sys_oper_type', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '清空操作');
INSERT INTO `sys_dict_data` VALUES ('28', 1, '成功', '0', 'sys_common_status', '', 'primary', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '正常状态');
INSERT INTO `sys_dict_data` VALUES ('29', 2, '失败', '1', 'sys_common_status', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '停用状态');
INSERT INTO `sys_dict_data` VALUES ('3', 3, '未知', '2', 'sys_user_sex', '', '', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '性别未知');
INSERT INTO `sys_dict_data` VALUES ('4', 1, '显示', '0', 'sys_show_hide', '', 'primary', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '显示菜单');
INSERT INTO `sys_dict_data` VALUES ('5', 2, '隐藏', '1', 'sys_show_hide', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '隐藏菜单');
INSERT INTO `sys_dict_data` VALUES ('6', 1, '正常', '0', 'sys_normal_disable', '', 'primary', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '正常状态');
INSERT INTO `sys_dict_data` VALUES ('7', 2, '停用', '1', 'sys_normal_disable', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '停用状态');
INSERT INTO `sys_dict_data` VALUES ('8', 1, '正常', '0', 'sys_job_status', '', 'primary', 'Y', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '正常状态');
INSERT INTO `sys_dict_data` VALUES ('9', 2, '暂停', '1', 'sys_job_status', '', 'danger', 'N', '0', 'admin', '2026-01-12 20:07:33', '', NULL, '停用状态');

-- ----------------------------
-- Table structure for sys_dict_type
-- ----------------------------
DROP TABLE IF EXISTS `sys_dict_type`;
CREATE TABLE `sys_dict_type`  (
                                  `dict_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '字典主键',
                                  `dict_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '字典名称',
                                  `dict_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '字典类型',
                                  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
                                  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                                  `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                                  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                                  `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                                  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                                  PRIMARY KEY (`dict_id`) USING BTREE,
                                  UNIQUE INDEX `dict_type`(`dict_type`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '字典类型表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_dict_type
-- ----------------------------
INSERT INTO `sys_dict_type` VALUES ('1', '用户性别', 'sys_user_sex', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '用户性别列表');
INSERT INTO `sys_dict_type` VALUES ('10', '系统状态', 'sys_common_status', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '登录状态列表');
INSERT INTO `sys_dict_type` VALUES ('2', '菜单状态', 'sys_show_hide', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '菜单状态列表');
INSERT INTO `sys_dict_type` VALUES ('3', '系统开关', 'sys_normal_disable', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '系统开关列表');
INSERT INTO `sys_dict_type` VALUES ('4', '任务状态', 'sys_job_status', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '任务状态列表');
INSERT INTO `sys_dict_type` VALUES ('5', '任务分组', 'sys_job_group', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '任务分组列表');
INSERT INTO `sys_dict_type` VALUES ('6', '系统是否', 'sys_yes_no', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '系统是否列表');
INSERT INTO `sys_dict_type` VALUES ('7', '通知类型', 'sys_notice_type', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '通知类型列表');
INSERT INTO `sys_dict_type` VALUES ('8', '通知状态', 'sys_notice_status', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '通知状态列表');
INSERT INTO `sys_dict_type` VALUES ('9', '操作类型', 'sys_oper_type', '0', 'admin', '2026-01-12 20:07:17', '', NULL, '操作类型列表');

-- ----------------------------
-- Table structure for sys_job
-- ----------------------------
DROP TABLE IF EXISTS `sys_job`;
CREATE TABLE `sys_job`  (
                            `job_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '任务ID',
                            `job_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '任务名称',
                            `job_group` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT 'DEFAULT' COMMENT '任务组名',
                            `invoke_target` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '调用目标字符串',
                            `cron_expression` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT 'cron执行表达式',
                            `misfire_policy` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '3' COMMENT '计划执行错误策略（1立即执行 2执行一次 3放弃执行）',
                            `concurrent` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '1' COMMENT '是否并发执行（0允许 1禁止）',
                            `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '状态（0正常 1暂停）',
                            `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                            `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                            `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                            `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                            `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '备注信息',
                            PRIMARY KEY (`job_id`, `job_name`, `job_group`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '定时任务调度表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_job
-- ----------------------------
INSERT INTO `sys_job` VALUES ('1', '系统默认（无参）', 'DEFAULT', 'ryTask.ryNoParams', '0/10 * * * * ?', '3', '1', '1', 'admin', '2021-08-16 10:39:41', '', NULL, '');
INSERT INTO `sys_job` VALUES ('2', '系统默认（有参）', 'DEFAULT', 'ryTask.ryParams(\'ry\')', '0/15 * * * * ?', '3', '1', '1', 'admin', '2021-08-16 10:39:41', '', NULL, '');
INSERT INTO `sys_job` VALUES ('3', '系统默认（多参）', 'DEFAULT', 'ryTask.ryMultipleParams(\'ry\', true, 2000L, 316.50D, 100)', '0/20 * * * * ?', '3', '1', '1', 'admin', '2021-08-16 10:39:41', '', NULL, '');

-- ----------------------------
-- Table structure for sys_job_log
-- ----------------------------
DROP TABLE IF EXISTS `sys_job_log`;
CREATE TABLE `sys_job_log`  (
                                `job_log_id` bigint(0) NOT NULL AUTO_INCREMENT COMMENT '任务日志ID',
                                `job_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '任务名称',
                                `job_group` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '任务组名',
                                `invoke_target` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '调用目标字符串',
                                `job_message` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '日志信息',
                                `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '执行状态（0正常 1失败）',
                                `exception_info` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '异常信息',
                                `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                                PRIMARY KEY (`job_log_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '定时任务调度日志表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_job_log
-- ----------------------------

-- ----------------------------
-- Table structure for sys_logininfor
-- ----------------------------
DROP TABLE IF EXISTS `sys_logininfor`;
CREATE TABLE `sys_logininfor`  (
                                   `info_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '访问ID',
                                   `user_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '用户账号',
                                   `ipaddr` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '登录IP地址',
                                   `login_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '登录地点',
                                   `browser` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '浏览器类型',
                                   `os` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '操作系统',
                                   `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '登录状态（0成功 1失败）',
                                   `msg` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '提示消息',
                                   `login_time` datetime(0) NULL DEFAULT NULL COMMENT '访问时间',
                                   PRIMARY KEY (`info_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '系统访问记录' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_logininfor
-- ----------------------------

-- ----------------------------
-- Table structure for sys_menu
-- ----------------------------
DROP TABLE IF EXISTS `sys_menu`;
CREATE TABLE `sys_menu`  (
                             `menu_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单ID',
                             `menu_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单名称',
                             `parent_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '父菜单ID',
                             `order_num` int(0) NULL DEFAULT 0 COMMENT '显示顺序',
                             `path` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '路由地址',
                             `component` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '组件路径',
                             `query` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '路由参数',
                             `route_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '路由名称',
                             `is_frame` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '1' COMMENT '是否为外链（0是 1否）',
                             `is_cache` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '是否缓存（0缓存 1不缓存）',
                             `menu_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '菜单类型（M目录 C菜单 F按钮）',
                             `visible` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '菜单状态（0显示 1隐藏）',
                             `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '菜单状态（0正常 1停用）',
                             `perms` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '权限标识',
                             `icon` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '#' COMMENT '菜单图标',
                             `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                             `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                             `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                             `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                             `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '备注',
                             PRIMARY KEY (`menu_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '菜单权限表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_menu
-- ----------------------------
INSERT INTO `sys_menu` VALUES ('1', '系统管理', '0', 1, 'system', NULL, '', '', '1', '0', 'M', '0', '0', '', 'system', 'admin', '2025-03-30 10:29:39', '', NULL, '系统管理目录');
INSERT INTO `sys_menu` VALUES ('100', '用户管理', '1', 1, 'user', 'system/user/index', '', '', '1', '0', 'C', '0', '0', 'system:user:list', 'user', 'admin', '2025-03-30 10:29:39', '', NULL, '用户管理菜单');
INSERT INTO `sys_menu` VALUES ('1000', '用户查询', '100', 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1001', '用户新增', '100', 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1002', '用户修改', '100', 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1003', '用户删除', '100', 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1004', '用户导出', '100', 5, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1005', '用户导入', '100', 6, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:import', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1006', '重置密码', '100', 7, '', '', '', '', '1', '0', 'F', '0', '0', 'system:user:resetPwd', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1007', '角色查询', '101', 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:role:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1008', '角色新增', '101', 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:role:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1009', '角色修改', '101', 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:role:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('101', '角色管理', '1', 2, 'role', 'system/role/index', '', '', '1', '0', 'C', '0', '0', 'system:role:list', 'peoples', 'admin', '2025-03-30 10:29:39', '', NULL, '角色管理菜单');
INSERT INTO `sys_menu` VALUES ('1010', '角色删除', '101', 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:role:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1011', '角色导出', '101', 5, '', '', '', '', '1', '0', 'F', '0', '0', 'system:role:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1012', '菜单查询', '102', 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1013', '菜单新增', '102', 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1014', '菜单修改', '102', 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1015', '菜单删除', '102', 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1016', '部门查询', '103', 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1017', '部门新增', '103', 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1018', '部门修改', '103', 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1019', '部门删除', '103', 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('102', '菜单管理', '1', 3, 'menu', 'system/menu/index', '', '', '1', '0', 'C', '0', '0', 'system:menu:list', 'tree-table', 'admin', '2025-03-30 10:29:39', '', NULL, '菜单管理菜单');
INSERT INTO `sys_menu` VALUES ('1020', '岗位查询', '104', 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1021', '岗位新增', '104', 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1022', '岗位修改', '104', 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1023', '岗位删除', '104', 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1024', '岗位导出', '104', 5, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1025', '字典查询', '105', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1026', '字典新增', '105', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1027', '字典修改', '105', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1028', '字典删除', '105', 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1029', '字典导出', '105', 5, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('103', '部门管理', '1', 4, 'dept', 'system/dept/index', '', '', '1', '0', 'C', '0', '0', 'system:dept:list', 'tree', 'admin', '2025-03-30 10:29:39', '', NULL, '部门管理菜单');
INSERT INTO `sys_menu` VALUES ('1030', '参数查询', '106', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1031', '参数新增', '106', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1032', '参数修改', '106', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1033', '参数删除', '106', 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1034', '参数导出', '106', 5, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1035', '公告查询', '107', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1036', '公告新增', '107', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1037', '公告修改', '107', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1038', '公告删除', '107', 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1039', '操作查询', '500', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('104', '岗位管理', '1', 5, 'post', 'system/post/index', '', '', '1', '0', 'C', '0', '0', 'system:post:list', 'post', 'admin', '2025-03-30 10:29:39', '', NULL, '岗位管理菜单');
INSERT INTO `sys_menu` VALUES ('1040', '操作删除', '500', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1041', '日志导出', '500', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1042', '登录查询', '501', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1043', '登录删除', '501', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1044', '日志导出', '501', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1045', '账户解锁', '501', 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:unlock', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1046', '在线查询', '109', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:online:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1047', '批量强退', '109', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:online:batchLogout', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1048', '单条强退', '109', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:online:forceLogout', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1049', '任务查询', '110', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:job:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('105', '字典管理', '1', 6, 'dict', 'system/dict/index', '', '', '1', '0', 'C', '0', '0', 'system:dict:list', 'dict', 'admin', '2025-03-30 10:29:39', '', NULL, '字典管理菜单');
INSERT INTO `sys_menu` VALUES ('1050', '任务新增', '110', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:job:add', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1051', '任务修改', '110', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:job:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1052', '任务删除', '110', 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:job:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1053', '状态修改', '110', 5, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:job:changeStatus', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1054', '任务导出', '110', 6, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:job:export', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1055', '生成查询', '116', 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'tool:gen:query', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1056', '生成修改', '116', 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'tool:gen:edit', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1057', '生成删除', '116', 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'tool:gen:remove', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1058', '导入代码', '116', 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'tool:gen:import', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('1059', '预览代码', '116', 5, '#', '', '', '', '1', '0', 'F', '0', '0', 'tool:gen:preview', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('106', '参数设置', '1', 7, 'config', 'system/config/index', '', '', '1', '0', 'C', '0', '0', 'system:config:list', 'edit', 'admin', '2025-03-30 10:29:39', '', NULL, '参数设置菜单');
INSERT INTO `sys_menu` VALUES ('1060', '生成代码', '116', 6, '#', '', '', '', '1', '0', 'F', '0', '0', 'tool:gen:code', '#', 'admin', '2025-03-30 10:29:39', '', NULL, '');
INSERT INTO `sys_menu` VALUES ('107', '通知公告', '1', 8, 'notice', 'system/notice/index', '', '', '1', '0', 'C', '0', '0', 'system:notice:list', 'message', 'admin', '2025-03-30 10:29:39', '', NULL, '通知公告菜单');
INSERT INTO `sys_menu` VALUES ('108', '日志管理', '1', 9, 'log', '', '', '', '1', '0', 'M', '0', '0', '', 'log', 'admin', '2025-03-30 10:29:39', '', NULL, '日志管理菜单');
INSERT INTO `sys_menu` VALUES ('109', '在线用户', '2', 1, 'online', 'monitor/online/index', '', '', '1', '0', 'C', '0', '0', 'monitor:online:list', 'online', 'admin', '2025-03-30 10:29:39', '', NULL, '在线用户菜单');
INSERT INTO `sys_menu` VALUES ('110', '定时任务', '2', 2, 'job', 'monitor/job/index', '', '', '1', '0', 'C', '0', '0', 'monitor:job:list', 'job', 'admin', '2025-03-30 10:29:39', '', NULL, '定时任务菜单');
INSERT INTO `sys_menu` VALUES ('111', '数据监控', '2', 3, 'druid', 'monitor/druid/index', '', '', '1', '0', 'C', '0', '0', 'monitor:druid:list', 'druid', 'admin', '2025-03-30 10:29:39', '', NULL, '数据监控菜单');
INSERT INTO `sys_menu` VALUES ('112', '服务监控', '2', 4, 'server', 'monitor/server/index', '', '', '1', '0', 'C', '0', '0', 'monitor:server:list', 'server', 'admin', '2025-03-30 10:29:39', '', NULL, '服务监控菜单');
INSERT INTO `sys_menu` VALUES ('113', '缓存监控', '2', 5, 'cache', 'monitor/cache/index', '', '', '1', '0', 'C', '0', '0', 'monitor:cache:list', 'redis', 'admin', '2025-03-30 10:29:39', '', NULL, '缓存监控菜单');
INSERT INTO `sys_menu` VALUES ('114', '缓存列表', '2', 6, 'cacheList', 'monitor/cache/list', '', '', '1', '0', 'C', '0', '0', 'monitor:cache:list', 'redis-list', 'admin', '2025-03-30 10:29:39', '', NULL, '缓存列表菜单');
INSERT INTO `sys_menu` VALUES ('115', '表单构建', '3', 1, 'build', 'tool/build/index', '', '', '1', '0', 'C', '0', '0', 'tool:build:list', 'build', 'admin', '2025-03-30 10:29:39', '', NULL, '表单构建菜单');
INSERT INTO `sys_menu` VALUES ('116', '代码生成', '3', 2, 'gen', 'tool/gen/index', '', '', '1', '0', 'C', '0', '0', 'tool:gen:list', 'code', 'admin', '2025-03-30 10:29:39', '', NULL, '代码生成菜单');
INSERT INTO `sys_menu` VALUES ('117', '系统接口', '3', 3, 'swagger', 'tool/swagger/index', '', '', '1', '0', 'C', '0', '0', 'tool:swagger:list', 'swagger', 'admin', '2025-03-30 10:29:39', '', NULL, '系统接口菜单');
INSERT INTO `sys_menu` VALUES ('2', '系统监控', '0', 2, 'monitor', NULL, '', '', '1', '0', 'M', '0', '0', '', 'monitor', 'admin', '2025-03-30 10:29:39', '', NULL, '系统监控目录');
INSERT INTO `sys_menu` VALUES ('3', '系统工具', '0', 3, 'tool', NULL, '', '', '1', '0', 'M', '0', '0', '', 'tool', 'admin', '2025-03-30 10:29:39', '', NULL, '系统工具目录');
INSERT INTO `sys_menu` VALUES ('500', '操作日志', '108', 1, 'operlog', 'monitor/operlog/index', '', '', '1', '0', 'C', '0', '0', 'monitor:operlog:list', 'form', 'admin', '2025-03-30 10:29:39', '', NULL, '操作日志菜单');
INSERT INTO `sys_menu` VALUES ('501', '登录日志', '108', 2, 'logininfor', 'monitor/logininfor/index', '', '', '1', '0', 'C', '0', '0', 'monitor:logininfor:list', 'logininfor', 'admin', '2025-03-30 10:29:39', '', NULL, '登录日志菜单');

-- ----------------------------
-- Table structure for sys_notice
-- ----------------------------
DROP TABLE IF EXISTS `sys_notice`;
CREATE TABLE `sys_notice`  (
                               `notice_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告ID',
                               `notice_title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告标题',
                               `notice_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告类型（1通知 2公告）',
                               `notice_content` longblob NULL COMMENT '公告内容',
                               `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '公告状态（0正常 1关闭）',
                               `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                               `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                               `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                               `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                               `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                               PRIMARY KEY (`notice_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '通知公告表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_notice
-- ----------------------------

-- ----------------------------
-- Table structure for sys_oper_log
-- ----------------------------
DROP TABLE IF EXISTS `sys_oper_log`;
CREATE TABLE `sys_oper_log`  (
                                 `oper_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '日志主键',
                                 `title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '模块标题',
                                 `business_type` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '业务类型',
                                 `method` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '方法名称',
                                 `request_method` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '请求方式',
                                 `operator_type` int(0) NULL DEFAULT 0 COMMENT '操作类别（0其它 1后台用户 2手机端用户）',
                                 `oper_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '操作人员',
                                 `dept_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '部门名称',
                                 `oper_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '请求URL',
                                 `oper_ip` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '主机地址',
                                 `oper_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '操作地点',
                                 `oper_param` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '请求参数',
                                 `json_result` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '返回参数',
                                 `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '操作状态（0正常 1异常）',
                                 `error_msg` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '错误消息',
                                 `oper_time` datetime(0) NULL DEFAULT NULL COMMENT '操作时间',
                                 `cost_time` bigint(0) NULL DEFAULT NULL COMMENT '消耗时间',
                                 PRIMARY KEY (`oper_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '操作日志记录' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_oper_log
-- ----------------------------

-- ----------------------------
-- Table structure for sys_post
-- ----------------------------
DROP TABLE IF EXISTS `sys_post`;
CREATE TABLE `sys_post`  (
                             `post_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位ID',
                             `post_code` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位编码',
                             `post_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位名称',
                             `post_sort` int(0) NOT NULL COMMENT '显示顺序',
                             `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '状态（0正常 1停用）',
                             `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                             `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                             `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                             `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                             `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                             PRIMARY KEY (`post_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '岗位信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_post
-- ----------------------------

-- ----------------------------
-- Table structure for sys_role
-- ----------------------------
DROP TABLE IF EXISTS `sys_role`;
CREATE TABLE `sys_role`  (
                             `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
                             `role_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色名称',
                             `role_key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色权限字符串',
                             `role_sort` int(0) NOT NULL COMMENT '显示顺序',
                             `data_scope` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '1' COMMENT '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
                             `menu_check_strictly` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '1' COMMENT '菜单树选择项是否关联显示',
                             `dept_check_strictly` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '1' COMMENT '部门树选择项是否关联显示',
                             `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色状态（0正常 1停用）',
                             `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
                             `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                             `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                             `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                             `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                             `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                             PRIMARY KEY (`role_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role
-- ----------------------------
INSERT INTO `sys_role` VALUES ('1', '超级管理员', 'admin', 1, '1', '1', '1', '0', '0', 'admin', '2021-08-16 10:39:40', '', NULL, '超级管理员');

-- ----------------------------
-- Table structure for sys_role_dept
-- ----------------------------
DROP TABLE IF EXISTS `sys_role_dept`;
CREATE TABLE `sys_role_dept`  (
                                  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
                                  `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '部门ID',
                                  PRIMARY KEY (`role_id`, `dept_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色和部门关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role_dept
-- ----------------------------

-- ----------------------------
-- Table structure for sys_role_menu
-- ----------------------------
DROP TABLE IF EXISTS `sys_role_menu`;
CREATE TABLE `sys_role_menu`  (
                                  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
                                  `menu_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单ID',
                                  PRIMARY KEY (`role_id`, `menu_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色和菜单关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role_menu
-- ----------------------------

-- ----------------------------
-- Table structure for sys_trash
-- ----------------------------
DROP TABLE IF EXISTS `sys_trash`;
CREATE TABLE `sys_trash`  (
                              `id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
                              `table_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
                              `data` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL,
                              `create_date` datetime(0) NULL DEFAULT NULL
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_trash
-- ----------------------------

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user`  (
                             `user_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
                             `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '部门ID',
                             `user_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户账号',
                             `nick_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户昵称',
                             `user_type` varchar(2) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '00' COMMENT '用户类型（00系统用户）',
                             `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '用户邮箱',
                             `phonenumber` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '手机号码',
                             `sex` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '用户性别（0男 1女 2未知）',
                             `avatar` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '头像地址',
                             `password` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '密码',
                             `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '账号状态（0正常 1停用）',
                             `last_chn_pwd_time` datetime(0) NULL DEFAULT NULL COMMENT '最后更改密码时间',
                             `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
                             `login_ip` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '最后登录IP',
                             `login_date` datetime(0) NULL DEFAULT NULL COMMENT '最后登录时间',
                             `pwd_update_date` datetime(0) NULL DEFAULT NULL COMMENT '密码最后更新时间',
                             `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
                             `create_time` datetime(0) NULL DEFAULT NULL COMMENT '创建时间',
                             `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
                             `update_time` datetime(0) NULL DEFAULT NULL COMMENT '更新时间',
                             `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
                             PRIMARY KEY (`user_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 100 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO `sys_user` VALUES ('1', '103', 'admin', '若依', '00', 'ry@163.com', '15888888888', '1', '', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2', '0', '2026-01-12 20:13:04', '0', '127.0.0.1', '2026-01-12 20:04:42', '2026-01-12 20:04:42', 'admin', '2026-01-12 20:04:42', '', NULL, '管理员');
INSERT INTO `sys_user` VALUES ('2', '105', 'ry', '若依', '00', 'ry@qq.com', '15666666666', '1', '', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2', '0', '2026-01-12 20:13:09', '0', '127.0.0.1', '2026-01-12 20:04:42', '2026-01-12 20:04:42', 'admin', '2026-01-12 20:04:42', '', NULL, '测试员');

-- ----------------------------
-- Table structure for sys_user_post
-- ----------------------------
DROP TABLE IF EXISTS `sys_user_post`;
CREATE TABLE `sys_user_post`  (
                                  `user_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
                                  `post_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位ID',
                                  PRIMARY KEY (`user_id`, `post_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户与岗位关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user_post
-- ----------------------------

-- ----------------------------
-- Table structure for sys_user_role
-- ----------------------------
DROP TABLE IF EXISTS `sys_user_role`;
CREATE TABLE `sys_user_role`  (
                                  `user_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
                                  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
                                  PRIMARY KEY (`user_id`, `role_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户和角色关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user_role
-- ----------------------------
INSERT INTO `sys_user_role` VALUES ('1', '1');

SET FOREIGN_KEY_CHECKS = 1;
