use crate::config::config::ApplicationConfig;
use crate::system::service::cache_service::CacheService;
use crate::system::service::*;
use log::LevelFilter;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use std::sync::LazyLock;
use std::time::Duration;

/// Service CONTEXT
pub static CONTEXT: LazyLock<ServiceContext> = LazyLock::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::context::CONTEXT.rb
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub cache_service: CacheService,
    pub sys_auth_service: SysAuthService,
    pub sys_menu_service: SysMenuService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_menu_service: SysRoleMenuService,
    pub sys_role_dept_service: SysRoleDeptService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_user_post_service: SysUserPostService,
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_data_service: SysDictDataService,
    pub sys_config_service: SysConfigService,
    pub sys_dept_service: SysDeptService,
    pub sys_trash_service: SysTrashService,
    pub sys_logininfor_service: SysLogininforService,
    pub sys_oper_log_service: SysOperLogService,
    pub sys_post_service: SysPostService,
    pub sys_notice_service: SysNoticeService,
    pub sys_user_online_service: SysUserOnlineService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_database(&self) {
        log::info!("[ruoyi_rust] rbatis pool init ({})...", self.config.db_url);
        //include auto choose driver struct by 'config.db_url'
        self.rb
            .link(MysqlDriver {}, &self.config.db_url)
            .await
            .expect("[ruoyi_rust] rbatis pool init fail!");
        //fixme 暂时删除
        // self.rb.intercepts.push(Arc::new(SysTrashService::new()));
        let pool = self.rb.get_pool().unwrap();
        //level
        self.rb
            .get_intercept::<LogInterceptor>()
            .expect("rbatis LogInterceptor init fail!")
            .set_level_filter(LevelFilter::Debug);
        //max connections
        pool.set_max_open_conns(self.config.db_pool_len as u64).await;
        //max timeout
        pool.set_timeout(Some(Duration::from_secs(self.config.db_pool_timeout as u64)))
            .await;
        log::info!(
            "[ruoyi_rust] rbatis pool init success! pool state = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let mut config = ApplicationConfig::default();
        ServiceContext {
            rb: {
                let rb = RBatis::new();
                if cfg!(debug_assertions) == false && config.debug.eq(&true) {
                    config.debug = false;
                }
                rb
            },
            cache_service: CacheService::new(&config).unwrap(),
            sys_auth_service: SysAuthService {},
            sys_menu_service: SysMenuService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_menu_service: SysRoleMenuService {},
            sys_role_dept_service: SysRoleDeptService {},
            sys_user_role_service: SysUserRoleService {},
            sys_user_post_service: SysUserPostService {},
            sys_dict_type_service: SysDictTypeService {},
            sys_dict_data_service: SysDictDataService {},
            sys_config_service: SysConfigService {},
            sys_dept_service: SysDeptService {},
            sys_trash_service: SysTrashService {
                recycle_date: Default::default(),
            },
            sys_logininfor_service: SysLogininforService {},
            sys_oper_log_service: SysOperLogService {},
            sys_post_service: SysPostService {},
            sys_notice_service: SysNoticeService {},
            sys_user_online_service: SysUserOnlineService {},
            config,
        }
    }
}
