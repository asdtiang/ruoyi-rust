-- MySQL dump 10.13  Distrib 8.0.45, for macos15 (arm64)
--
-- Host: 127.0.0.1    Database: stock_scan_dev
-- ------------------------------------------------------
-- Server version	9.7.0

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
SET @MYSQLDUMP_TEMP_LOG_BIN = @@SESSION.SQL_LOG_BIN;
SET @@SESSION.SQL_LOG_BIN= 0;

--
-- GTID state at the beginning of the backup 
--

SET @@GLOBAL.GTID_PURGED=/*!80000 '+'*/ '0bf4706f-e975-11f1-b303-6a024b5bcd49:1-248';

--
-- Table structure for table `DATABASECHANGELOG`
--

DROP TABLE IF EXISTS `DATABASECHANGELOG`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `DATABASECHANGELOG` (
  `ID` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `AUTHOR` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `FILENAME` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `DATEEXECUTED` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `MD5SUM` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
  PRIMARY KEY (`ID`,`AUTHOR`,`FILENAME`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `DATABASECHANGELOG`
--

LOCK TABLES `DATABASECHANGELOG` WRITE;
/*!40000 ALTER TABLE `DATABASECHANGELOG` DISABLE KEYS */;
/*!40000 ALTER TABLE `DATABASECHANGELOG` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `gen_table`
--

DROP TABLE IF EXISTS `gen_table`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `gen_table` (
  `table_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '编号',
  `table_name` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '表名称',
  `table_comment` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '表描述',
  `sub_table_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '关联子表的表名',
  `sub_table_fk_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '子表关联的外键名',
  `struct_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '实体类名称',
  `tpl_category` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT 'crud' COMMENT '使用的模板（crud单表操作 tree树表操作）',
  `tpl_web_type` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '前端模板类型（element-ui模版 element-plus模版）',
  `tpl_back_type` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '后台模板类型（rust java）',
  `package_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '生成包路径',
  `module_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '生成模块名',
  `business_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '生成业务名',
  `function_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '生成功能名',
  `function_author` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '生成功能作者',
  `gen_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '生成代码方式（0zip压缩包 1自定义路径）',
  `gen_path_web` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '前端生成路径（不填默认项目路径）',
  `gen_path_back` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '/' COMMENT '后台生成路径（不填默认项目路径）',
  `options` json DEFAULT NULL COMMENT '其它生成选项',
  `switch_opt` json DEFAULT NULL COMMENT '开关选项',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`table_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='代码生成业务表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `gen_table`
--

LOCK TABLES `gen_table` WRITE;
/*!40000 ALTER TABLE `gen_table` DISABLE KEYS */;
/*!40000 ALTER TABLE `gen_table` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `gen_table_column`
--

DROP TABLE IF EXISTS `gen_table_column`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `gen_table_column` (
  `column_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '编号',
  `table_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '归属表编号',
  `column_name` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '列名称',
  `column_comment` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '列描述',
  `column_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '列类型',
  `rust_type` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT 'rust类型',
  `rust_field` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT 'rust字段名',
  `is_pk` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否主键（1是）',
  `is_increment` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否自增（1是）',
  `is_required` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否必填（1是）',
  `is_insert` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否为插入字段（1是）',
  `is_edit` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否编辑字段（1是）',
  `is_list` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否列表字段（1是）',
  `is_table` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否表格显示字段（1是）',
  `is_detail` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否详情字段（1是）',
  `is_export` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '是否导出字段（1是）',
  `is_sortable` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '是否排序字段（1是）',
  `is_query` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '是否查询字段（1是）',
  `query_type` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT 'EQ' COMMENT '查询方式（等于、不等于、大于、小于、范围）',
  `html_type` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '显示类型（文本框、文本域、下拉框、复选框、单选框、日期控件）',
  `dict_type` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字典类型',
  `sort` int DEFAULT NULL COMMENT '排序',
  `more` json DEFAULT NULL COMMENT '正则表达式提示消息',
  `def_val` varchar(400) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '默认值',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`column_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='代码生成业务表字段';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `gen_table_column`
--

LOCK TABLES `gen_table_column` WRITE;
/*!40000 ALTER TABLE `gen_table_column` DISABLE KEYS */;
/*!40000 ALTER TABLE `gen_table_column` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_config`
--

DROP TABLE IF EXISTS `sys_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_config` (
  `config_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '参数主键',
  `config_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '参数名称',
  `config_key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '参数键名',
  `config_value` varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '参数键值',
  `config_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT 'N' COMMENT '系统内置（Y是 N否）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`config_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='参数配置表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_config`
--

LOCK TABLES `sys_config` WRITE;
/*!40000 ALTER TABLE `sys_config` DISABLE KEYS */;
INSERT INTO `sys_config` VALUES ('1','主框架页-默认皮肤样式名称','sys.index.skinName','skin-blue','Y','admin','2026-01-12 20:08:38','',NULL,'蓝色 skin-blue、绿色 skin-green、紫色 skin-purple、红色 skin-red、黄色 skin-yellow'),('2','用户管理-账号初始密码','sys.user.initPassword','123456','Y','admin','2026-01-12 20:08:38','',NULL,'初始化密码 123456'),('3','主框架页-侧边栏主题','sys.index.sideTheme','theme-dark','Y','admin','2026-01-12 20:08:38','',NULL,'深色主题theme-dark，浅色主题theme-light'),('4','账号自助-验证码开关','sys.account.captchaEnabled','true','N','admin','2026-01-12 20:08:38','admin','2026-10-16 23:58:18','是否开启验证码功能（true开启，false关闭）'),('5','账号自助-是否开启用户注册功能','sys.account.registerUser','false','Y','admin','2026-01-12 20:08:38','',NULL,'是否开启注册用户功能（true开启，false关闭）'),('6','用户登录-黑名单列表','sys.login.blackIPList','','Y','admin','2026-01-12 20:08:38','',NULL,'设置登录IP黑名单限制，多个匹配项以;分隔，支持匹配（*通配、网段）'),('7','用户管理-初始密码修改策略','sys.account.initPasswordModify','1','Y','admin','2026-01-12 20:08:38','',NULL,'0：初始密码修改策略关闭，没有任何提示，1：提醒用户，如果未修改初始密码，则在登录时就会提醒修改密码对话框'),('8','用户管理-账号密码更新周期','sys.account.passwordValidateDays','0','Y','admin','2026-01-12 20:08:38','',NULL,'密码更新周期（填写数字，数据初始化值为0不限制，若修改必须为大于0小于365的正整数），如果超过这个周期登录系统时，则在登录时就会提醒修改密码对话框');
/*!40000 ALTER TABLE `sys_config` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dept`
--

DROP TABLE IF EXISTS `sys_dept`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dept` (
  `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '部门id',
  `parent_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '父部门id',
  `ancestors` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '祖级列表',
  `dept_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '部门名称',
  `order_num` int DEFAULT '0' COMMENT '显示顺序',
  `leader` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '负责人',
  `phone` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '联系电话',
  `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '邮箱',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '部门状态（0正常 1停用）',
  `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`dept_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='部门表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dept`
--

LOCK TABLES `sys_dept` WRITE;
/*!40000 ALTER TABLE `sys_dept` DISABLE KEYS */;
INSERT INTO `sys_dept` VALUES ('100','0','0','若依科技',0,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('101','100','0,100','深圳总公司',1,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('102','100','0,100','长沙分公司',2,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('103','101','0,100,101','研发部门',1,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('104','101','0,100,101','市场部门',2,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('105','101','0,100,101','测试部门',3,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('106','101','0,100,101','财务部门',4,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('107','101','0,100,101','运维部门',5,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('108','102','0,100,102','市场部门',1,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('109','102','0,100,102','财务部门',2,'若依','15888888888','ry@qq.com','0','0','admin','2026-01-12 20:01:33','',NULL),('110','100','0,100','宝点会员',5,'','','','0','0','admin','2026-10-17 03:56:01','',NULL);
/*!40000 ALTER TABLE `sys_dept` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dict_data`
--

DROP TABLE IF EXISTS `sys_dict_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dict_data` (
  `dict_code` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '字典编码',
  `dict_sort` int DEFAULT '0' COMMENT '字典排序',
  `dict_label` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字典标签',
  `dict_value` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字典键值',
  `dict_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字典类型',
  `css_class` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '样式属性（其他样式扩展）',
  `list_class` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '表格回显样式',
  `is_default` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT 'N' COMMENT '是否默认（Y是 N否）',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '状态（0正常 1停用）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`dict_code`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='字典数据表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dict_data`
--

LOCK TABLES `sys_dict_data` WRITE;
/*!40000 ALTER TABLE `sys_dict_data` DISABLE KEYS */;
INSERT INTO `sys_dict_data` VALUES ('1',1,'男','0','sys_user_sex','','','Y','0','admin','2026-01-12 20:07:33','',NULL,'性别男'),('10',1,'默认','DEFAULT','sys_job_group','','','Y','0','admin','2026-01-12 20:07:33','',NULL,'默认分组'),('11',2,'系统','SYSTEM','sys_job_group','','','N','0','admin','2026-01-12 20:07:33','',NULL,'系统分组'),('12',1,'是','Y','sys_yes_no','','primary','Y','0','admin','2026-01-12 20:07:33','',NULL,'系统默认是'),('13',2,'否','N','sys_yes_no','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'系统默认否'),('14',1,'通知','1','sys_notice_type','','warning','Y','0','admin','2026-01-12 20:07:33','',NULL,'通知'),('15',2,'公告','2','sys_notice_type','','success','N','0','admin','2026-01-12 20:07:33','',NULL,'公告'),('16',1,'正常','0','sys_notice_status','','primary','Y','0','admin','2026-01-12 20:07:33','',NULL,'正常状态'),('17',2,'关闭','1','sys_notice_status','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'关闭状态'),('18',99,'其他','0','sys_oper_type','','info','N','0','admin','2026-01-12 20:07:33','',NULL,'其他操作'),('19',1,'新增','1','sys_oper_type','','info','N','0','admin','2026-01-12 20:07:33','',NULL,'新增操作'),('2',2,'女','1','sys_user_sex','','','N','0','admin','2026-01-12 20:07:33','',NULL,'性别女'),('20',2,'修改','2','sys_oper_type','','info','N','0','admin','2026-01-12 20:07:33','',NULL,'修改操作'),('21',3,'删除','3','sys_oper_type','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'删除操作'),('22',4,'授权','4','sys_oper_type','','primary','N','0','admin','2026-01-12 20:07:33','',NULL,'授权操作'),('23',5,'导出','5','sys_oper_type','','warning','N','0','admin','2026-01-12 20:07:33','',NULL,'导出操作'),('24',6,'导入','6','sys_oper_type','','warning','N','0','admin','2026-01-12 20:07:33','',NULL,'导入操作'),('25',7,'强退','7','sys_oper_type','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'强退操作'),('26',8,'生成代码','8','sys_oper_type','','warning','N','0','admin','2026-01-12 20:07:33','',NULL,'生成操作'),('27',9,'清空数据','9','sys_oper_type','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'清空操作'),('28',1,'成功','0','sys_common_status','','primary','N','0','admin','2026-01-12 20:07:33','',NULL,'正常状态'),('29',2,'失败','1','sys_common_status','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'停用状态'),('3',3,'未知','2','sys_user_sex','','','N','0','admin','2026-01-12 20:07:33','',NULL,'性别未知'),('4',1,'显示','0','sys_show_hide','','primary','Y','0','admin','2026-01-12 20:07:33','',NULL,'显示菜单'),('5',2,'隐藏','1','sys_show_hide','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'隐藏菜单'),('6',1,'正常','0','sys_normal_disable','','primary','Y','0','admin','2026-01-12 20:07:33','',NULL,'正常状态'),('7',2,'停用','1','sys_normal_disable','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'停用状态'),('8',1,'正常','0','sys_job_status','','primary','Y','0','admin','2026-01-12 20:07:33','',NULL,'正常状态'),('9',2,'暂停','1','sys_job_status','','danger','N','0','admin','2026-01-12 20:07:33','',NULL,'停用状态');
/*!40000 ALTER TABLE `sys_dict_data` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dict_type`
--

DROP TABLE IF EXISTS `sys_dict_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dict_type` (
  `dict_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '字典主键',
  `dict_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字典名称',
  `dict_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '字典类型',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '状态（0正常 1停用）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`dict_id`) USING BTREE,
  UNIQUE KEY `dict_type` (`dict_type`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='字典类型表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dict_type`
--

LOCK TABLES `sys_dict_type` WRITE;
/*!40000 ALTER TABLE `sys_dict_type` DISABLE KEYS */;
INSERT INTO `sys_dict_type` VALUES ('1','用户性别','sys_user_sex','0','admin','2026-01-12 20:07:17','',NULL,'用户性别列表'),('10','系统状态','sys_common_status','0','admin','2026-01-12 20:07:17','',NULL,'登录状态列表'),('2','菜单状态','sys_show_hide','0','admin','2026-01-12 20:07:17','',NULL,'菜单状态列表'),('3','系统开关','sys_normal_disable','0','admin','2026-01-12 20:07:17','',NULL,'系统开关列表'),('4','任务状态','sys_job_status','0','admin','2026-01-12 20:07:17','',NULL,'任务状态列表'),('5','任务分组','sys_job_group','0','admin','2026-01-12 20:07:17','',NULL,'任务分组列表'),('6','系统是否','sys_yes_no','0','admin','2026-01-12 20:07:17','',NULL,'系统是否列表'),('7','通知类型','sys_notice_type','0','admin','2026-01-12 20:07:17','',NULL,'通知类型列表'),('8','通知状态','sys_notice_status','0','admin','2026-01-12 20:07:17','',NULL,'通知状态列表'),('9','操作类型','sys_oper_type','0','admin','2026-01-12 20:07:17','',NULL,'操作类型列表');
/*!40000 ALTER TABLE `sys_dict_type` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_job`
--

DROP TABLE IF EXISTS `sys_job`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_job` (
  `job_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '任务ID',
  `job_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '任务名称',
  `job_group` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT 'DEFAULT' COMMENT '任务组名',
  `invoke_target` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '调用目标字符串',
  `cron_expression` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT 'cron执行表达式',
  `misfire_policy` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '3' COMMENT '计划执行错误策略（1立即执行 2执行一次 3放弃执行）',
  `concurrent` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '1' COMMENT '是否并发执行（0允许 1禁止）',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '状态（0正常 1暂停）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '备注信息',
  PRIMARY KEY (`job_id`,`job_name`,`job_group`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='定时任务调度表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_job`
--

LOCK TABLES `sys_job` WRITE;
/*!40000 ALTER TABLE `sys_job` DISABLE KEYS */;
INSERT INTO `sys_job` VALUES ('1','系统默认（无参）','DEFAULT','ryTask.ryNoParams','0/10 * * * * ?','3','1','1','admin','2021-08-16 10:39:41','',NULL,''),('2','系统默认（有参）','DEFAULT','ryTask.ryParams(\'ry\')','0/15 * * * * ?','3','1','1','admin','2021-08-16 10:39:41','',NULL,''),('3','系统默认（多参）','DEFAULT','ryTask.ryMultipleParams(\'ry\', true, 2000L, 316.50D, 100)','0/20 * * * * ?','3','1','1','admin','2021-08-16 10:39:41','',NULL,'');
/*!40000 ALTER TABLE `sys_job` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_job_log`
--

DROP TABLE IF EXISTS `sys_job_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_job_log` (
  `job_log_id` bigint NOT NULL AUTO_INCREMENT COMMENT '任务日志ID',
  `job_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '任务名称',
  `job_group` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '任务组名',
  `invoke_target` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '调用目标字符串',
  `job_message` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '日志信息',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '执行状态（0正常 1失败）',
  `exception_info` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '异常信息',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`job_log_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='定时任务调度日志表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_job_log`
--

LOCK TABLES `sys_job_log` WRITE;
/*!40000 ALTER TABLE `sys_job_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_job_log` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_logininfor`
--

DROP TABLE IF EXISTS `sys_logininfor`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_logininfor` (
  `info_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '访问ID',
  `user_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '用户账号',
  `ipaddr` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '登录IP地址',
  `login_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '登录地点',
  `browser` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '浏览器类型',
  `os` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '操作系统',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '登录状态（0成功 1失败）',
  `msg` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '提示消息',
  `login_time` datetime DEFAULT NULL COMMENT '访问时间',
  PRIMARY KEY (`info_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='系统访问记录';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_logininfor`
--

LOCK TABLES `sys_logininfor` WRITE;
/*!40000 ALTER TABLE `sys_logininfor` DISABLE KEYS */;
INSERT INTO `sys_logininfor` VALUES ('69a54bae00d5266300000000','admin','127.0.0.1','内网IP','Chrome 133','Windows 10','0','成功','2026-03-02 16:34:54'),('69a64cc70088b58300000000','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-03 10:51:51'),('69a6996800e8634700000000','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-03 16:18:48'),('69a8d0350015670800000000','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-05 08:37:09'),('69a8d15300f9ef3700000003','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 08:41:55'),('69a8d3cd00c5910d00000000','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 08:52:29'),('69a8d4c100c5910d00000003','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-03-05 08:56:33'),('69a8d4ce00c5910d00000004','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 08:56:46'),('69a8d53b00c5910d00000006','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-03-05 08:58:35'),('69a8d54300c5910d00000007','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 08:58:43'),('69a8dca800f890af00000000','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-03-05 09:30:16'),('69a8dcae00f890af00000001','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 09:30:22'),('69a8dd2900f890af00000002','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-03-05 09:32:25'),('69a8dd3000f890af00000003','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 09:32:32'),('69a8dd74004aeaa300000000','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-03-05 09:33:40'),('69a8dd7b004aeaa300000001','ry','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-03-05 09:33:47'),('69a907f9008c8f7200000000','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-05 12:35:05'),('69a952f3008ea2b700000000','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-05 17:54:59'),('69aa4a8600efb36800000000','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-06 11:31:18'),('69aa9c0f00e1c57a00000033','admin','127.0.0.1','内网IP','Chrome 145','Windows 10','0','成功','2026-03-06 17:19:11'),('69ae899d0036288400000000','admin','127.0.0.1','内网IP','Edge 145','Windows 10','0','成功','2026-03-09 16:49:33'),('69b0e0d300f15ee200000000','admin','127.0.0.1','内网IP','Edge 145','Windows 10','0','成功','2026-03-11 11:26:11'),('69b11f1f004f054600000000','admin','127.0.0.1','内网IP','Edge 145','Windows 10','0','成功','2026-03-11 15:51:59'),('6ad2498100444f4600000000','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-16 23:57:53'),('6ad2499e00444f4600000002','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','退出成功','2026-10-16 23:58:22'),('6ad249ae00444f4600000003','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-16 23:58:38'),('6ad249b500444f4600000004','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','退出成功','2026-10-16 23:58:45'),('6ad249bb00444f4600000005','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-16 23:58:51'),('6ad24f0000444f4600000006','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','退出成功','2026-10-17 00:21:20'),('6ad250260014a37400000000','admin','127.0.0.1','内网IP','Mobile Safari 18','iOS 18','0','成功','2026-10-17 00:26:14'),('6ad2e27a00d4e1bb00000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 10:50:34'),('6ad2e28c00d4e1bb00000001','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 10:50:52'),('6ad2e6a900d4e1bb00000002','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:08:25'),('6ad2e6b600d4e1bb00000013','13810825','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:08:38'),('6ad2e6b700d4e1bb00000014','139108251','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:08:39'),('6ad2e6b900d4e1bb00000015','139108252','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:08:41'),('6ad2e6ba00d4e1bb00000016','139108253','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:08:42'),('6ad2e70500d4e1bb00000049','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:09:57'),('6ad2e71100d4e1bb0000005a','13810957','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:10:09'),('6ad2e71200d4e1bb0000005b','139109571','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:10:10'),('6ad2e71400d4e1bb0000005c','139109572','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:10:12'),('6ad2e71500d4e1bb0000005d','139109573','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:10:13'),('6ad2e7b100d4e1bb00000090','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:12:49'),('6ad2e7ec00d4e1bb00000092','13810957','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:13:48'),('6ad2e8da00d4e1bb00000093','13810957','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:17:46'),('6ad2e94900d4e1bb00000094','13810957','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:19:37'),('6ad2eae800d4e1bb00000095','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:26:32'),('6ad2eaf500d4e1bb000000a8','13812632','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:26:45'),('6ad2eaf600d4e1bb000000a9','139126321','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:26:46'),('6ad2eaf700d4e1bb000000aa','139126322','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:26:47'),('6ad2eaf800d4e1bb000000ab','139126323','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:26:48'),('6ad2ebcd00d4e1bb000000e0','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 11:30:21'),('6ad2ebde00d4e1bb000000e1','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 11:30:38'),('6ad2ebeb00d4e1bb000000e2','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 11:30:51'),('6ad2ec3300d4e1bb000000e3','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:32:03'),('6ad2ec6f00d4e1bb000000e4','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:33:03'),('6ad2ec7700d4e1bb000000f7','13813302','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:33:11'),('6ad2ec7800d4e1bb000000f8','139133021','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:33:12'),('6ad2ec7900d4e1bb000000f9','139133022','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:33:13'),('6ad2ec7a00d4e1bb000000fa','139133023','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 11:33:14'),('6ad2eec600d4e1bb00000109','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 11:43:02'),('6ad2eed300d4e1bb0000010a','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 11:43:15'),('6ad2eedd00d4e1bb0000010b','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 11:43:25'),('6ad2f2f5009797c900000000','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:00:53'),('6ad2f312009797c900000001','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:01:22'),('6ad2f47b002ffad000000000','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:07:23'),('6ad2f487002ffad000000001','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:07:35'),('6ad2f491002ffad000000012','13820735','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:07:45'),('6ad2f493002ffad000000013','139207351','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:07:47'),('6ad2f494002ffad000000014','139207352','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:07:48'),('6ad2f495002ffad000000015','139207353','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:07:49'),('6ad2f496002ffad000000016','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:07:50'),('6ad2f497002ffad000000017','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:07:51'),('6ad2f4b0002ffad000000061','13820735','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:08:16'),('6ad2f4b3002ffad000000062','13820735','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:08:19'),('6ad2f4fd00ae6e4900000000','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:09:33'),('6ad2f51300ae6e4900000001','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:09:55'),('6ad2f52500ae6e4900000002','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:10:13'),('6ad2f54300ae6e4900000003','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:10:43'),('6ad2f55b00ae6e4900000004','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:11:07'),('6ad2f57100ae6e4900000005','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:11:29'),('6ad2f57e00ae6e4900000006','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:11:42'),('6ad2f59300ae6e4900000007','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:12:03'),('6ad2f5d400ae6e4900000008','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:13:08'),('6ad2f5df00ae6e4900000009','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:13:19'),('6ad2f61800708dc700000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:14:16'),('6ad2f63300708dc700000001','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:14:43'),('6ad2f63f00708dc700000003','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:14:55'),('6ad2f64900708dc700000005','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:15:05'),('6ad2f65800708dc700000006','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:15:20'),('6ad2f66e00708dc700000007','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:15:42'),('6ad2f67600708dc700000008','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:15:50'),('6ad2f68900708dc700000009','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:16:09'),('6ad2f69400708dc70000000a','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:16:20'),('6ad2f6f900708dc70000000e','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:18:01'),('6ad2f70200708dc700000023','13821800','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:18:10'),('6ad2f70300708dc700000024','139218001','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:18:11'),('6ad2f70400708dc700000025','139218002','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:18:12'),('6ad2f70400708dc700000026','139218003','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:18:12'),('6ad2f76d00708dc70000002f','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:19:57'),('6ad2f77600708dc700000040','13821957','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:20:06'),('6ad2f77700708dc700000041','139219571','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:20:07'),('6ad2f77800708dc700000042','139219572','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:20:08'),('6ad2f77900708dc700000043','139219573','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:20:09'),('6ad2f7e500708dc70000004d','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:21:57'),('6ad2f81500708dc70000004e','13900003333','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:22:45'),('6ad2f83200708dc70000004f','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:14'),('6ad2f83b00708dc700000060','13822314','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:23'),('6ad2f83c00708dc700000061','139223141','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:24'),('6ad2f83d00708dc700000062','139223142','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:25'),('6ad2f83e00708dc700000063','139223143','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:26'),('6ad2f85700708dc7000000b1','13822314','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:51'),('6ad2f85900708dc7000000b2','13822314','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:23:53'),('6ad2f88d00708dc7000000b3','admin','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 12:24:45'),('6ad2f8a200708dc7000000b4','13900006666','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:25:06'),('6ad2f8a300708dc7000000b5','13900006666','127.0.0.1','内网IP','curl 8','Other ','1','账号不存在或者密码错误!','2026-10-17 12:25:07'),('6ad2f8c800708dc7000000b6','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:25:44'),('6ad2f8d200708dc7000000c7','13822544','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:25:54'),('6ad2f8d300708dc7000000c8','139225441','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:25:55'),('6ad2f8d400708dc7000000c9','139225442','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:25:56'),('6ad2f8d500708dc7000000ca','139225443','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:25:57'),('6ad2f8da00708dc7000000d4','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:26:02'),('6ad2f8f100708dc700000118','13822544','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:26:25'),('6ad2f8f400708dc700000119','13822544','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:26:28'),('6ad2f98500708dc70000011b','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:28:53'),('6ad2f99100708dc70000012c','13822853','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:29:05'),('6ad2f99300708dc70000012d','139228531','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:29:07'),('6ad2f99400708dc70000012e','139228532','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:29:08'),('6ad2f99500708dc70000012f','139228533','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:29:09'),('6ad2f9b400708dc70000017f','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:29:40'),('6ad2f9b500708dc700000180','13822853','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:29:41'),('6ad2f9b900708dc700000181','13822853','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:29:45'),('6ad2fa1e00708dc700000182','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:31:26'),('6ad2fa6d00708dc700000183','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:32:45'),('6ad2fab600708dc700000184','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:33:58'),('6ad2fad000708dc700000185','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:34:24'),('6ad2fad900708dc700000186','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:34:33'),('6ad2fb0800708dc700000187','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:35:20'),('6ad2fb2900708dc700000188','admin','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:35:53'),('6ad2fc0200f6124200000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:39:30'),('6ad2fc6600f6124200000005','13900000001','127.0.0.1','内网IP','Mobile Safari 17','iOS 17','0','成功','2026-10-17 12:41:10'),('6ad2fcba00f6124200000009','13800002222','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:42:34'),('6ad2fcd300f612420000000a','13800002222','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:42:59'),('6ad2fcf100f612420000000b','13800002222','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:43:29'),('6ad2fd3900f612420000000c','13800002222','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 12:44:41'),('6ad2fde700f612420000000d','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:47:35'),('6ad2febe00f612420000000e','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:51:10'),('6ad2feec00f612420000000f','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:51:56'),('6ad2fef400f6124200000020','13825156','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:52:04'),('6ad2fef500f6124200000021','139251561','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:52:05'),('6ad2fef600f6124200000022','139251562','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:52:06'),('6ad2fef700f6124200000023','139251563','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:52:07'),('6ad2ff2b00f6124200000028','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:52:59'),('6ad2ff3300f6124200000039','13825258','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:53:07'),('6ad2ff3400f612420000003a','139252581','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:53:08'),('6ad2ff3500f612420000003b','139252582','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:53:09'),('6ad2ff3600f612420000003c','139252583','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:53:10'),('6ad2ff3b00f6124200000040','13825258','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:53:15'),('6ad2ff4000f6124200000041','13825258','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:53:20'),('6ad2ff5a00f6124200000043','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 12:53:46'),('6ad2ffb200f6124200000045','admin','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 12:55:14'),('6ad2ffd700f6124200000046','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:55:51'),('6ad2ffe100f6124200000057','13825551','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:01'),('6ad2ffe200f6124200000058','139255511','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:02'),('6ad2ffe300f6124200000059','139255512','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:03'),('6ad2ffe400f612420000005a','139255513','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:04'),('6ad2fffd00f61242000000aa','13825551','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:29'),('6ad3000000f61242000000ab','13825551','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:32'),('6ad3000100f61242000000ac','139255512','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:33'),('6ad3000400f61242000000ad','139255512','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:36'),('6ad3000b00f61242000000ae','139255511','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:43'),('6ad3000b00f61242000000af','139255511','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:43'),('6ad3000c00f61242000000b0','13825551','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:44'),('6ad3000e00f61242000000b4','13825551','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:56:46'),('6ad3003e00f61242000000b5','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:34'),('6ad3004900f61242000000c6','13825734','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:45'),('6ad3004a00f61242000000c7','139257341','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:46'),('6ad3004a00f61242000000c8','139257342','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:46'),('6ad3004b00f61242000000c9','139257343','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:47'),('6ad3004e00f61242000000cd','13825734','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:50'),('6ad3005600f61242000000ce','13825734','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 12:57:58'),('6ad3005700f61242000000cf','13825734','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:57:59'),('6ad3005800f61242000000d0','139257341','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:00'),('6ad3007a00f61242000000d1','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:34'),('6ad3008200f61242000000d5','13825834','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:42'),('6ad3008300f61242000000d6','139258341','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:43'),('6ad3008500f61242000000d7','139258342','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:45'),('6ad3008500f61242000000d8','139258343','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:45'),('6ad3008500f61242000000d9','13900000001','127.0.0.1','内网IP','Mobile Safari 17','iOS 17','0','成功','2026-10-17 12:58:45'),('6ad3008800f61242000000dd','13825834','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 12:58:48'),('6ad3008900f61242000000de','13825834','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:58:49'),('6ad300bb00f61242000000df','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:59:39'),('6ad300c400f61242000000f0','13825939','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:59:48'),('6ad300c500f61242000000f1','139259391','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:59:49'),('6ad300c600f61242000000f2','139259392','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:59:50'),('6ad300c700f61242000000f3','139259393','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 12:59:51'),('6ad300e200f6124200000143','13825939','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:18'),('6ad300e500f6124200000144','13825939','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:21'),('6ad300e600f6124200000145','139259392','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:22'),('6ad300e900f6124200000146','139259392','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:25'),('6ad300ee00f6124200000147','139259393','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:30'),('6ad300f100f6124200000148','139259392','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 13:00:33'),('6ad300f200f6124200000149','139259391','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:34'),('6ad300f300f612420000014a','139259391','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:35'),('6ad300f400f612420000014b','13825939','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:36'),('6ad300f600f612420000014f','13825939','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:00:38'),('6ad3013500f6124200000150','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:01:41'),('6ad3013e00f6124200000154','13830141','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:01:50'),('6ad3013f00f6124200000155','139301411','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:01:51'),('6ad3014000f6124200000156','139301412','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:01:52'),('6ad3014100f6124200000157','139301413','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:01:53'),('6ad3014400f612420000015c','13830141','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:01:56'),('6ad3018300f612420000015d','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:02:59'),('6ad3018c00f612420000016e','13830259','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:08'),('6ad3018d00f612420000016f','139302591','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:09'),('6ad3018e00f6124200000170','139302592','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:10'),('6ad3018f00f6124200000171','139302593','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:11'),('6ad301a800f61242000001c3','13830259','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:36'),('6ad301aa00f61242000001c4','13830259','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:38'),('6ad301ab00f61242000001c5','139302592','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:39'),('6ad301ad00f61242000001c6','139302592','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 13:03:41'),('6ad3153c0042a7a000000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 14:27:08'),('6ad315640042a7a000000002','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 14:27:48'),('6ad315960042a7a000000003','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:28:38'),('6ad3167c00be850c00000000','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:32:28'),('6ad3168400be850c00000011','13843228','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:32:36'),('6ad3168400be850c00000012','139432281','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:32:36'),('6ad3168500be850c00000013','139432282','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:32:37'),('6ad3168600be850c00000014','139432283','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:32:38'),('6ad3169c00be850c00000066','13843228','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:00'),('6ad3169d00be850c00000067','13843228','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:01'),('6ad3169e00be850c00000068','139432282','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:02'),('6ad316a000be850c00000069','139432282','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:04'),('6ad316a500be850c0000006a','139432283','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:09'),('6ad316a600be850c0000006b','139432282','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 14:33:10'),('6ad316a700be850c0000006c','139432281','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:11'),('6ad316a800be850c0000006d','139432281','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:12'),('6ad316a800be850c0000006e','13843228','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:12'),('6ad316aa00be850c00000072','13843228','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:33:14'),('6ad316fa00be850c00000073','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:34:34'),('6ad316fb00be850c00000074','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:34:35'),('6ad3170c00be850c00000075','13992218891','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:34:52'),('6ad3177a00be850c00000076','13992219001','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:36:42'),('6ad3177b00be850c00000077','13992219001','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:36:43'),('6ad3181500cb7b6800000000','13992219156','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:17'),('6ad3181600cb7b6800000001','13992219156','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:18'),('6ad3181900cb7b6800000002','13892219160','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:21'),('6ad3181a00cb7b6800000003','13892219160','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:22'),('6ad3182a00cb7b6800000004','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:38'),('6ad3183100cb7b6800000015','13843937','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:45'),('6ad3183200cb7b6800000016','139439371','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:46'),('6ad3183300cb7b6800000017','139439372','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:47'),('6ad3183400cb7b6800000018','139439373','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:39:48'),('6ad3184b00cb7b680000006a','13843937','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:11'),('6ad3184d00cb7b680000006b','13843937','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:13'),('6ad3184d00cb7b680000006c','139439372','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:13'),('6ad3184f00cb7b680000006d','139439372','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:15'),('6ad3185400cb7b680000006e','139439373','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:20'),('6ad3185600cb7b680000006f','139439372','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 14:40:22'),('6ad3185700cb7b6800000070','139439371','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:23'),('6ad3185700cb7b6800000071','139439371','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:23'),('6ad3185800cb7b6800000072','13843937','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:24'),('6ad3185900cb7b6800000076','13843937','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:40:25'),('6ad318d700cb7b6800000077','store9','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 14:42:31'),('6ad318f800cb7b6800000078','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 14:43:04'),('6ad3192000cb7b6800000084','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:43:44'),('6ad31a2e00cb7b680000008d','store9','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 14:48:14'),('6ad31f6200cb7b680000008e','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:26'),('6ad31f6900cb7b680000009f','13851025','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:33'),('6ad31f6a00cb7b68000000a0','139510251','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:34'),('6ad31f6a00cb7b68000000a1','139510252','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:34'),('6ad31f6b00cb7b68000000a2','139510253','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:35'),('6ad31f8100cb7b68000000f4','13851025','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:57'),('6ad31f8300cb7b68000000f5','13851025','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:10:59'),('6ad31f8400cb7b68000000f6','139510252','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:00'),('6ad31f8600cb7b68000000f7','139510252','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:02'),('6ad31f8a00cb7b68000000f8','139510253','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:06'),('6ad31f8c00cb7b68000000f9','139510252','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:11:08'),('6ad31f8d00cb7b68000000fa','139510251','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:09'),('6ad31f8e00cb7b68000000fb','139510251','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:10'),('6ad31f8f00cb7b68000000fc','13851025','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:11'),('6ad31f9000cb7b6800000100','13851025','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:12'),('6ad31f9100cb7b6800000101','139510251','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:11:13'),('6ad31f9200cb7b6800000102','139510253','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:11:14'),('6ad3202c00cb7b680000010d','13851025','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:13:48'),('6ad3202d00cb7b680000010e','139510251','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:13:49'),('6ad3202d00cb7b680000010f','139510252','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:13:49'),('6ad3202e00cb7b6800000110','139510253','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:13:50'),('6ad320e700cb7b6800000111','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:16:55'),('6ad320ee00cb7b6800000122','13851654','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:02'),('6ad320ef00cb7b6800000123','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:03'),('6ad320f000cb7b6800000124','139516542','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:04'),('6ad320f000cb7b6800000125','139516543','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:04'),('6ad3210800cb7b6800000177','13851654','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:28'),('6ad3210a00cb7b6800000178','13851654','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:30'),('6ad3210b00cb7b6800000179','139516542','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:31'),('6ad3210d00cb7b680000017a','139516542','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:33'),('6ad3211200cb7b680000017b','139516543','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:38'),('6ad3211400cb7b680000017c','139516542','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:17:40'),('6ad3211500cb7b680000017d','139516542','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:17:41'),('6ad3211500cb7b680000017e','139516542','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:41'),('6ad3211800cb7b680000017f','139516543','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:17:44'),('6ad3211800cb7b6800000180','139516543','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:44'),('6ad3211a00cb7b6800000181','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:46'),('6ad3211b00cb7b6800000182','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:47'),('6ad3211c00cb7b6800000183','13851654','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:48'),('6ad3211e00cb7b6800000187','13851654','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:50'),('6ad3211e00cb7b6800000188','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:50'),('6ad3211f00cb7b6800000189','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:51'),('6ad3212100cb7b680000018a','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 15:17:53'),('6ad3212200cb7b680000018b','139516541','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:17:54'),('6ad3216f00cb7b68000001a1','store9','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 15:19:11'),('6ad3217400cb7b68000001a2','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:19:16'),('6ad3217600cb7b68000001ad','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:19:18'),('6ad321d000cb7b68000001ae','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:20:48'),('6ad3239c00cb7b68000001af','store9','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 15:28:28'),('6ad3256700cb7b68000001b0','store9','127.0.0.1','内网IP','HeadlessChrome 155','Windows 10','0','成功','2026-10-17 15:36:07'),('6ad3260800cb7b68000001b1','store9','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 15:38:48'),('6ad326b100cb7b68000001b2','store9','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:41:37'),('6ad326d200cb7b68000001b3','store9','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:42:10'),('6ad326e700cb7b68000001b4','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:42:31'),('6ad327b400cb7b68000001b5','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:45:56'),('6ad327c500cb7b68000001b6','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:46:13'),('6ad3287a00f7886b00000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:49:14'),('6ad3288100f7886b00000001','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 15:49:21'),('6ad32a5d00f7886b00000011','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:57:17'),('6ad32a6b00f7886b00000012','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 15:57:31'),('6ad32b9f00f7886b00000013','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:02:39'),('6ad32d0600f7886b00000017','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:08:38'),('6ad32dc600f7886b00000019','admin','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 16:11:50'),('6ad32e5600f7886b0000001a','store9','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 16:14:14'),('6ad32e8f009733ae00000000','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:15:11'),('6ad32edb009733ae00000010','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:27'),('6ad32ee8009733ae00000012','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:40'),('6ad32ef0009733ae00000023','13861640','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:48'),('6ad32ef1009733ae00000024','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:49'),('6ad32ef2009733ae00000025','139616402','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:50'),('6ad32ef3009733ae00000026','139616403','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:51'),('6ad32ef3009733ae00000027','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:16:51'),('6ad32f0b009733ae0000007a','13861640','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:15'),('6ad32f0d009733ae0000007b','13861640','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:17'),('6ad32f0e009733ae0000007c','139616402','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:18'),('6ad32f11009733ae0000007d','139616402','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:21'),('6ad32f17009733ae0000007e','139616403','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:27'),('6ad32f19009733ae0000007f','139616402','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 16:17:29'),('6ad32f1b009733ae00000080','139616402','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 16:17:31'),('6ad32f1c009733ae00000081','139616402','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:32'),('6ad32f1e009733ae00000082','139616403','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 16:17:34'),('6ad32f1f009733ae00000083','139616403','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:35'),('6ad32f22009733ae00000084','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:38'),('6ad32f23009733ae00000085','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:39'),('6ad32f25009733ae00000086','13861640','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:41'),('6ad32f26009733ae0000008a','13861640','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:42'),('6ad32f27009733ae0000008b','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:43'),('6ad32f28009733ae0000008c','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:44'),('6ad32f2c009733ae0000008d','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','1','账号不存在或者密码错误!','2026-10-17 16:17:48'),('6ad32f2d009733ae0000008e','139616401','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:17:49'),('6ad32f79009733ae000000a5','admin','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 16:19:05'),('6ad32fa3009733ae000000a6','admin','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:19:47'),('6ad3300d009733ae000000a7','store9','127.0.0.1','内网IP','Python Requests 2','Other ','0','成功','2026-10-17 16:21:33'),('6ad3304b009733ae000000a8','store9','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 16:22:35'),('6ad330be009733ae000000a9','store9','127.0.0.1','内网IP','HeadlessChrome 147','Windows 10','0','成功','2026-10-17 16:24:30'),('6ad3322b009733ae000000aa','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-17 16:30:35'),('6ad33fd90008a36400000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 17:28:57'),('6ad33fda0008a36400000001','admin','127.0.0.1','内网IP','curl 8','Other ','1','账号不存在或者密码错误!','2026-10-17 17:28:58'),('6ad33fda0008a36400000002','admin','127.0.0.1','内网IP','curl 8','Other ','1','账号不存在或者密码错误!','2026-10-17 17:28:58'),('6ad33fe20008a36400000003','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 17:29:06'),('6ad350920087abc100000000','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-17 18:40:18'),('6ad3529e004a793600000001','18190910296','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-10-17 18:49:02'),('6ad352b7004a793600000003','18190910296','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-10-17 18:49:27'),('6ad3564d00dbb66f00000003','18190910296','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-10-17 19:04:45'),('6ad3565100dbb66f00000004','139108251','127.0.0.1','内网IP','Chrome 132','Windows 10','1','账号不存在或者密码错误!','2026-10-17 19:04:49'),('6ad3565b00dbb66f00000005','139108251','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-10-17 19:04:59'),('6ad356f600dbb66f00000006','139108251','127.0.0.1','内网IP','Chrome 132','Windows 10','0','退出成功','2026-10-17 19:07:34'),('6ad356fe00dbb66f00000007','18190910296','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-10-17 19:07:42'),('6ad38bd100301b2c00000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:53:05'),('6ad38bdd00301b2c00000001','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:53:17'),('6ad38bec00301b2c00000002','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:53:32'),('6ad38c1200301b2c00000003','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:54:10'),('6ad38c4a00301b2c00000004','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:55:06'),('6ad38c7500301b2c00000005','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:55:49'),('6ad38c9900301b2c00000006','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:56:25'),('6ad38cac00301b2c00000007','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:56:44'),('6ad38cbd00301b2c00000008','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-17 22:57:01'),('6ad493e50062da3500000000','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-18 17:39:49'),('6ad494490062da3500000001','18190910296','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-10-18 17:41:29'),('6ad495d80062da3500000007','18190910292','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-18 17:48:08'),('6ad49bc90062da350000000a','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:13:29'),('6ad49bd90062da350000000b','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:13:45'),('6ad49c1a0062da350000000c','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:14:50'),('6ad49c430062da350000000d','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:15:31'),('6ad49c97006fb96a00000000','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:16:55'),('6ad49e12006fb96a00000001','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:23:14'),('6ad49e1f006fb96a00000002','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:23:27'),('6ad49e31006fb96a00000003','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:23:45'),('6ad49e3a006fb96a00000004','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:23:54'),('6ad49e45006fb96a00000005','admin','127.0.0.1','内网IP','curl 8','Other ','0','成功','2026-10-18 18:24:05'),('6ad4a9ae006fb96a00000006','admin','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-18 19:12:46'),('6ad4a9ca006fb96a00000007','18190910296','127.0.0.1','内网IP','Chrome 132','Windows 10','0','成功','2026-10-18 19:13:14'),('6ad4ab9b006fb96a00000009','18190910296','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-18 19:20:59'),('6ad4afb20067055900000000','18190910293','127.0.0.1','内网IP','Chrome 147','Windows 10','0','成功','2026-10-18 19:38:26'),('6ad4be380019902500000002','18190910296','127.0.0.1','内网IP','Edge 155','Windows 10','0','成功','2026-10-18 20:40:24'),('6b08e245007de8a200000000','admin','127.0.0.1','','Chrome 157','Mac OS X 10','0','成功','2026-11-27 09:52:05'),('6b08e28c007de8a200000001','admin','127.0.0.1','','Chrome 157','Mac OS X 10','0','退出成功','2026-11-27 09:53:16'),('6b08e293007de8a200000002','admin','127.0.0.1','','Chrome 157','Mac OS X 10','0','成功','2026-11-27 09:53:23');
/*!40000 ALTER TABLE `sys_logininfor` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_menu`
--

DROP TABLE IF EXISTS `sys_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_menu` (
  `menu_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单ID',
  `menu_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单名称',
  `parent_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '父菜单ID',
  `order_num` int DEFAULT '0' COMMENT '显示顺序',
  `path` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '路由地址',
  `component` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '组件路径',
  `query` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '路由参数',
  `route_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '路由名称',
  `is_frame` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '1' COMMENT '是否为外链（0是 1否）',
  `is_cache` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '是否缓存（0缓存 1不缓存）',
  `menu_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '菜单类型（M目录 C菜单 F按钮）',
  `visible` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '菜单状态（0显示 1隐藏）',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '菜单状态（0正常 1停用）',
  `perms` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '权限标识',
  `icon` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '#' COMMENT '菜单图标',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '备注',
  PRIMARY KEY (`menu_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='菜单权限表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_menu`
--

LOCK TABLES `sys_menu` WRITE;
/*!40000 ALTER TABLE `sys_menu` DISABLE KEYS */;
INSERT INTO `sys_menu` VALUES ('1','系统管理','0',1,'system',NULL,'','','1','0','M','0','0','','system','admin','2025-03-30 10:29:39','',NULL,'系统管理目录'),('100','用户管理','1',1,'user','system/user/index','','','1','0','C','0','0','system:user:list','user','admin','2025-03-30 10:29:39','',NULL,'用户管理菜单'),('1000','用户查询','100',1,'','','','','1','0','F','0','0','system:user:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1001','用户新增','100',2,'','','','','1','0','F','0','0','system:user:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1002','用户修改','100',3,'','','','','1','0','F','0','0','system:user:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1003','用户删除','100',4,'','','','','1','0','F','0','0','system:user:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1004','用户导出','100',5,'','','','','1','0','F','0','0','system:user:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1005','用户导入','100',6,'','','','','1','0','F','0','0','system:user:import','#','admin','2025-03-30 10:29:39','',NULL,''),('1006','重置密码','100',7,'','','','','1','0','F','0','0','system:user:resetPwd','#','admin','2025-03-30 10:29:39','',NULL,''),('1007','角色查询','101',1,'','','','','1','0','F','0','0','system:role:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1008','角色新增','101',2,'','','','','1','0','F','0','0','system:role:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1009','角色修改','101',3,'','','','','1','0','F','0','0','system:role:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('101','角色管理','1',2,'role','system/role/index','','','1','0','C','0','0','system:role:list','peoples','admin','2025-03-30 10:29:39','',NULL,'角色管理菜单'),('1010','角色删除','101',4,'','','','','1','0','F','0','0','system:role:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1011','角色导出','101',5,'','','','','1','0','F','0','0','system:role:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1012','菜单查询','102',1,'','','','','1','0','F','0','0','system:menu:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1013','菜单新增','102',2,'','','','','1','0','F','0','0','system:menu:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1014','菜单修改','102',3,'','','','','1','0','F','0','0','system:menu:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1015','菜单删除','102',4,'','','','','1','0','F','0','0','system:menu:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1016','部门查询','103',1,'','','','','1','0','F','0','0','system:dept:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1017','部门新增','103',2,'','','','','1','0','F','0','0','system:dept:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1018','部门修改','103',3,'','','','','1','0','F','0','0','system:dept:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1019','部门删除','103',4,'','','','','1','0','F','0','0','system:dept:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('102','菜单管理','1',3,'menu','system/menu/index','','','1','0','C','0','0','system:menu:list','tree-table','admin','2025-03-30 10:29:39','',NULL,'菜单管理菜单'),('1020','岗位查询','104',1,'','','','','1','0','F','0','0','system:post:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1021','岗位新增','104',2,'','','','','1','0','F','0','0','system:post:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1022','岗位修改','104',3,'','','','','1','0','F','0','0','system:post:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1023','岗位删除','104',4,'','','','','1','0','F','0','0','system:post:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1024','岗位导出','104',5,'','','','','1','0','F','0','0','system:post:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1025','字典查询','105',1,'#','','','','1','0','F','0','0','system:dict:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1026','字典新增','105',2,'#','','','','1','0','F','0','0','system:dict:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1027','字典修改','105',3,'#','','','','1','0','F','0','0','system:dict:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1028','字典删除','105',4,'#','','','','1','0','F','0','0','system:dict:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1029','字典导出','105',5,'#','','','','1','0','F','0','0','system:dict:export','#','admin','2025-03-30 10:29:39','',NULL,''),('103','部门管理','1',4,'dept','system/dept/index','','','1','0','C','0','0','system:dept:list','tree','admin','2025-03-30 10:29:39','',NULL,'部门管理菜单'),('1030','参数查询','106',1,'#','','','','1','0','F','0','0','system:config:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1031','参数新增','106',2,'#','','','','1','0','F','0','0','system:config:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1032','参数修改','106',3,'#','','','','1','0','F','0','0','system:config:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1033','参数删除','106',4,'#','','','','1','0','F','0','0','system:config:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1034','参数导出','106',5,'#','','','','1','0','F','0','0','system:config:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1035','公告查询','107',1,'#','','','','1','0','F','0','0','system:notice:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1036','公告新增','107',2,'#','','','','1','0','F','0','0','system:notice:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1037','公告修改','107',3,'#','','','','1','0','F','0','0','system:notice:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1038','公告删除','107',4,'#','','','','1','0','F','0','0','system:notice:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1039','操作查询','500',1,'#','','','','1','0','F','0','0','monitor:operlog:query','#','admin','2025-03-30 10:29:39','',NULL,''),('104','岗位管理','1',5,'post','system/post/index','','','1','0','C','0','0','system:post:list','post','admin','2025-03-30 10:29:39','',NULL,'岗位管理菜单'),('1040','操作删除','500',2,'#','','','','1','0','F','0','0','monitor:operlog:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1041','日志导出','500',3,'#','','','','1','0','F','0','0','monitor:operlog:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1042','登录查询','501',1,'#','','','','1','0','F','0','0','monitor:logininfor:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1043','登录删除','501',2,'#','','','','1','0','F','0','0','monitor:logininfor:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1044','日志导出','501',3,'#','','','','1','0','F','0','0','monitor:logininfor:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1045','账户解锁','501',4,'#','','','','1','0','F','0','0','monitor:logininfor:unlock','#','admin','2025-03-30 10:29:39','',NULL,''),('1046','在线查询','109',1,'#','','','','1','0','F','0','0','monitor:online:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1047','批量强退','109',2,'#','','','','1','0','F','0','0','monitor:online:batchLogout','#','admin','2025-03-30 10:29:39','',NULL,''),('1048','单条强退','109',3,'#','','','','1','0','F','0','0','monitor:online:forceLogout','#','admin','2025-03-30 10:29:39','',NULL,''),('1049','任务查询','110',1,'#','','','','1','0','F','0','0','monitor:job:query','#','admin','2025-03-30 10:29:39','',NULL,''),('105','字典管理','1',6,'dict','system/dict/index','','','1','0','C','0','0','system:dict:list','dict','admin','2025-03-30 10:29:39','',NULL,'字典管理菜单'),('1050','任务新增','110',2,'#','','','','1','0','F','0','0','monitor:job:add','#','admin','2025-03-30 10:29:39','',NULL,''),('1051','任务修改','110',3,'#','','','','1','0','F','0','0','monitor:job:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1052','任务删除','110',4,'#','','','','1','0','F','0','0','monitor:job:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1053','状态修改','110',5,'#','','','','1','0','F','0','0','monitor:job:changeStatus','#','admin','2025-03-30 10:29:39','',NULL,''),('1054','任务导出','110',6,'#','','','','1','0','F','0','0','monitor:job:export','#','admin','2025-03-30 10:29:39','',NULL,''),('1055','生成查询','116',1,'#','','','','1','0','F','0','0','tool:gen:query','#','admin','2025-03-30 10:29:39','',NULL,''),('1056','生成修改','116',2,'#','','','','1','0','F','0','0','tool:gen:edit','#','admin','2025-03-30 10:29:39','',NULL,''),('1057','生成删除','116',3,'#','','','','1','0','F','0','0','tool:gen:remove','#','admin','2025-03-30 10:29:39','',NULL,''),('1058','导入代码','116',4,'#','','','','1','0','F','0','0','tool:gen:import','#','admin','2025-03-30 10:29:39','',NULL,''),('1059','预览代码','116',5,'#','','','','1','0','F','0','0','tool:gen:preview','#','admin','2025-03-30 10:29:39','',NULL,''),('106','参数设置','1',7,'config','system/config/index','','','1','0','C','0','0','system:config:list','edit','admin','2025-03-30 10:29:39','',NULL,'参数设置菜单'),('1060','生成代码','116',6,'#','','','','1','0','F','0','0','tool:gen:code','#','admin','2025-03-30 10:29:39','',NULL,''),('107','通知公告','1',8,'notice','system/notice/index','','','1','0','C','0','0','system:notice:list','message','admin','2025-03-30 10:29:39','',NULL,'通知公告菜单'),('108','日志管理','1',9,'log','','','','1','0','M','0','0','','log','admin','2025-03-30 10:29:39','',NULL,'日志管理菜单'),('109','在线用户','2',1,'online','monitor/online/index','','','1','0','C','0','0','monitor:online:list','online','admin','2025-03-30 10:29:39','',NULL,'在线用户菜单'),('110','定时任务','2',2,'job','monitor/job/index','','','1','0','C','0','0','monitor:job:list','job','admin','2025-03-30 10:29:39','',NULL,'定时任务菜单'),('111','数据监控','2',3,'druid','monitor/druid/index','','','1','0','C','0','0','monitor:druid:list','druid','admin','2025-03-30 10:29:39','',NULL,'数据监控菜单'),('112','服务监控','2',4,'server','monitor/server/index','','','1','0','C','0','0','monitor:server:list','server','admin','2025-03-30 10:29:39','',NULL,'服务监控菜单'),('113','缓存监控','2',5,'cache','monitor/cache/index','','','1','0','C','0','0','monitor:cache:list','redis','admin','2025-03-30 10:29:39','',NULL,'缓存监控菜单'),('114','缓存列表','2',6,'cacheList','monitor/cache/list','','','1','0','C','0','0','monitor:cache:list','redis-list','admin','2025-03-30 10:29:39','',NULL,'缓存列表菜单'),('115','表单构建','3',1,'build','tool/build/index','','','1','0','C','0','0','tool:build:list','build','admin','2025-03-30 10:29:39','',NULL,'表单构建菜单'),('116','代码生成','3',2,'gen','tool/gen/index','','','1','0','C','0','0','tool:gen:list','code','admin','2025-03-30 10:29:39','',NULL,'代码生成菜单'),('117','系统接口','3',3,'swagger','tool/swagger/index','','','1','0','C','0','0','tool:swagger:list','swagger','admin','2025-03-30 10:29:39','',NULL,'系统接口菜单'),('2','系统监控','0',2,'monitor',NULL,'','','1','0','M','0','0','','monitor','admin','2025-03-30 10:29:39','',NULL,'系统监控目录'),('3','系统工具','0',3,'tool',NULL,'','','1','0','M','0','0','','tool','admin','2025-03-30 10:29:39','',NULL,'系统工具目录'),('500','操作日志','108',1,'operlog','monitor/operlog/index','','','1','0','C','0','0','monitor:operlog:list','form','admin','2025-03-30 10:29:39','',NULL,'操作日志菜单'),('501','登录日志','108',2,'logininfor','monitor/logininfor/index','','','1','0','C','0','0','monitor:logininfor:list','logininfor','admin','2025-03-30 10:29:39','',NULL,'登录日志菜单');
/*!40000 ALTER TABLE `sys_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_notice`
--

DROP TABLE IF EXISTS `sys_notice`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_notice` (
  `notice_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告ID',
  `notice_title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告标题',
  `notice_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告类型（1通知 2公告）',
  `notice_content` longblob COMMENT '公告内容',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '公告状态（0正常 1关闭）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`notice_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='通知公告表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_notice`
--

LOCK TABLES `sys_notice` WRITE;
/*!40000 ALTER TABLE `sys_notice` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_notice` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_oper_log`
--

DROP TABLE IF EXISTS `sys_oper_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_oper_log` (
  `oper_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '日志主键',
  `title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '模块标题',
  `business_type` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '业务类型',
  `method` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '方法名称',
  `request_method` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '请求方式',
  `operator_type` int DEFAULT '0' COMMENT '操作类别（0其它 1后台用户 2手机端用户）',
  `oper_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '操作人员',
  `dept_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '部门名称',
  `oper_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '请求URL',
  `oper_ip` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '主机地址',
  `oper_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '操作地点',
  `oper_param` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '请求参数',
  `json_result` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '返回参数',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '操作状态（0正常 1异常）',
  `error_msg` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '错误消息',
  `oper_time` datetime DEFAULT NULL COMMENT '操作时间',
  `cost_time` bigint DEFAULT NULL COMMENT '消耗时间',
  PRIMARY KEY (`oper_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='操作日志记录';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_oper_log`
--

LOCK TABLES `sys_oper_log` WRITE;
/*!40000 ALTER TABLE `sys_oper_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_oper_log` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_post`
--

DROP TABLE IF EXISTS `sys_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_post` (
  `post_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位ID',
  `post_code` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位编码',
  `post_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位名称',
  `post_sort` int NOT NULL COMMENT '显示顺序',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '状态（0正常 1停用）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`post_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='岗位信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_post`
--

LOCK TABLES `sys_post` WRITE;
/*!40000 ALTER TABLE `sys_post` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_post` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role`
--

DROP TABLE IF EXISTS `sys_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role` (
  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
  `role_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色名称',
  `role_key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色权限字符串',
  `role_sort` int NOT NULL COMMENT '显示顺序',
  `data_scope` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '1' COMMENT '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
  `menu_check_strictly` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '1' COMMENT '菜单树选择项是否关联显示',
  `dept_check_strictly` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '1' COMMENT '部门树选择项是否关联显示',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色状态（0正常 1停用）',
  `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  `store_id` bigint DEFAULT NULL COMMENT '所属门店；NULL 为平台角色',
  PRIMARY KEY (`role_id`) USING BTREE,
  KEY `ix_sys_role_store` (`store_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='角色信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role`
--

LOCK TABLES `sys_role` WRITE;
/*!40000 ALTER TABLE `sys_role` DISABLE KEYS */;
INSERT INTO `sys_role` VALUES ('1','超级管理员','admin',1,'1','1','1','0','0','admin','2021-08-16 10:39:40','',NULL,'超级管理员',NULL);
/*!40000 ALTER TABLE `sys_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_dept`
--

DROP TABLE IF EXISTS `sys_role_dept`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_dept` (
  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
  `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '部门ID',
  PRIMARY KEY (`role_id`,`dept_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='角色和部门关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_dept`
--

LOCK TABLES `sys_role_dept` WRITE;
/*!40000 ALTER TABLE `sys_role_dept` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_role_dept` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_menu`
--

DROP TABLE IF EXISTS `sys_role_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_menu` (
  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
  `menu_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单ID',
  PRIMARY KEY (`role_id`,`menu_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='角色和菜单关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_menu`
--

LOCK TABLES `sys_role_menu` WRITE;
/*!40000 ALTER TABLE `sys_role_menu` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_role_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_trash`
--

DROP TABLE IF EXISTS `sys_trash`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_trash` (
  `id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `table_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `data` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci,
  `create_date` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_trash`
--

LOCK TABLES `sys_trash` WRITE;
/*!40000 ALTER TABLE `sys_trash` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_trash` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user`
--

DROP TABLE IF EXISTS `sys_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user` (
  `user_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
  `dept_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '部门ID',
  `user_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户账号',
  `nick_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户昵称',
  `user_type` varchar(2) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '00' COMMENT '用户类型（00系统用户）',
  `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '用户邮箱',
  `phonenumber` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '手机号码',
  `sex` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '用户性别（0男 1女 2未知）',
  `avatar` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '头像地址',
  `password` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '密码',
  `status` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '账号状态（0正常 1停用）',
  `last_chn_pwd_time` datetime DEFAULT NULL COMMENT '最后更改密码时间',
  `del_flag` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）',
  `login_ip` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '最后登录IP',
  `login_date` datetime DEFAULT NULL COMMENT '最后登录时间',
  `pwd_update_date` datetime DEFAULT NULL COMMENT '密码最后更新时间',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '' COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '备注',
  `store_id` bigint DEFAULT NULL COMMENT '当前所在门店',
  `store_name` varchar(255) DEFAULT NULL COMMENT '当前所在门店名称',
  `points_balance` bigint NOT NULL DEFAULT '0' COMMENT '积分余额',
  `invite_code` varchar(8) DEFAULT NULL COMMENT '6位唯一邀请码（字母+数字），用户创建时自动生成',
  PRIMARY KEY (`user_id`) USING BTREE,
  UNIQUE KEY `uk_sys_user_invite_code` (`invite_code`),
  KEY `ix_sys_user_store` (`store_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='用户信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES ('1','103','admin','若依','00','ry@163.com','15888888888','1','','$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2','0','2026-01-12 20:13:04','0','127.0.0.1','2026-01-12 20:04:42','2026-01-12 20:04:42','admin','2026-01-12 20:04:42','',NULL,'管理员',NULL,NULL,0,'XEMTKU'),('2','105','ry','若依','00','ry@qq.com','15666666666','1','','$2b$12$cp6uaR5tVMCdqXnUSQ23..s.8nM34sZGM4vyNsd0pSt2Dm3iZDf8q','0','2026-03-05 08:52:13','0','127.0.0.1','2026-01-12 20:04:42','2026-01-12 20:04:42','admin','2026-01-12 20:04:42','admin','2026-03-05 08:41:33','测试员',NULL,NULL,0,'BJVSL7');
/*!40000 ALTER TABLE `sys_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_post`
--

DROP TABLE IF EXISTS `sys_user_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_post` (
  `user_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
  `post_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位ID',
  PRIMARY KEY (`user_id`,`post_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='用户与岗位关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_post`
--

LOCK TABLES `sys_user_post` WRITE;
/*!40000 ALTER TABLE `sys_user_post` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_user_post` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_role`
--

DROP TABLE IF EXISTS `sys_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_role` (
  `user_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户ID',
  `role_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`user_id`,`role_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci ROW_FORMAT=DYNAMIC COMMENT='用户和角色关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_role`
--

LOCK TABLES `sys_user_role` WRITE;
/*!40000 ALTER TABLE `sys_user_role` DISABLE KEYS */;
INSERT INTO `sys_user_role` VALUES ('1','1');
/*!40000 ALTER TABLE `sys_user_role` ENABLE KEYS */;
UNLOCK TABLES;
SET @@SESSION.SQL_LOG_BIN = @MYSQLDUMP_TEMP_LOG_BIN;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2026-11-27 10:05:42
