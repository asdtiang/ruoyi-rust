use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::modules::system::constants::SYS_YES;
use crate::system::domain::dto::ConfigPageDTO;
use crate::system::domain::mapper::sys_config::SysConfig;
use crate::system::domain::vo::SysConfigVO;
use crate::{check_unique, export_excel_service, pool, remove_batch_tx};
use macros::replace_pool;
use rbatis::{field_name, Page, PageRequest};

const SYS_CONFIG_KEY: &'static str = "sys_config:";

pub struct SysConfigService {}

impl SysConfigService {
    pub async fn page(&self, arg: &ConfigPageDTO) -> Result<Page<SysConfig>> {
        let data = SysConfig::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }

    pub async fn detail(&self, config_id: &str) -> Result<SysConfig> {
        let config =
            SysConfig::select_by_column(pool!(), field_name!(SysConfig.config_id), config_id)
                .await?
                .into_iter()
                .next()
                .ok_or_else(|| Error::from(format!("不存在:{} ！", config_id)))?;
        Ok(config)
    }


    pub async fn add(&self, config: SysConfig) -> Result<u64> {
        self.check_config_key_unique(&None, config.config_key.clone().unwrap_or_default())
            .await?;
        let result = SysConfig::insert(pool!(), &config).await?.rows_affected;
        if result == 1 {
            self.save_to_cache(&config).await?;
        }
        Ok(result)
    }

    pub async fn update(&self, config: SysConfig) -> Result<u64> {
        self.check_config_key_unique(&None, config.config_key.clone().unwrap_or_default())
            .await?;
        let result = SysConfig::update_by_column(pool!(), &config, "config_id")
            .await?
            .rows_affected;
        if result == 1 {
            self.save_to_cache(&config).await?;
        }
        Ok(result)
    }

    remove_batch_tx!(config_ids);

    #[replace_pool(tx)]
    pub async fn remove(&self, config_id: &str) -> Result<u64> {
        let targets = SysConfig::select_by_column(pool!(), "config_id", config_id)
            .await?
            .into_iter()
            .next();
        match targets {
            None => Ok(0),
            Some(cf) => {
                if cf.config_type.eq(&Some(SYS_YES)) {
                    return Err(Error::from(format!(
                        "内置参数【{}】不能删除！",
                        cf.config_key.unwrap_or_default()
                    )));
                }

                let r = SysConfig::delete_by_column(pool!(), "config_id", config_id).await?;
                if r.rows_affected > 0 {
                    //copy data to trash
                    let config_key = cf.config_key.clone().unwrap_or_default();
                    CONTEXT.sys_trash_service.add("sys_config", &[cf]).await?;
                    let _ = CONTEXT
                        .cache_service
                        .del(&self.get_cache_key(&config_key))
                        .await?;
                }
                Ok(r.rows_affected)
            }
        }
    }

    /**
     * 获取验证码开关，默认不打开
     *
     * @return true开启，false关闭
     */
    pub async fn select_captcha_enabled(&self) -> Result<bool> {
        let captcha_enabled = self
            .select_config_by_key("sys.account.captchaEnabled")
            .await;
        Ok(captcha_enabled.map(|c| c.eq("true")).unwrap_or(false))
    }

    /**
     * 根据键名查询参数配置信息
     *
     */
    pub async fn select_config_by_key(&self, config_key: &str) -> Result<String> {
        let config_value = CONTEXT
            .cache_service
            .get_string(&self.get_cache_key(config_key))
            .await?;
        if !config_value.is_empty() {
            return Ok(config_value);
        }
        let config = SysConfig::select_by_column(pool!(), "config_key", config_key).await?;
        match config.into_iter().next() {
            None => {
                Err(Error::from(format!("未找到配置参数：{}",config_key)))
            },
            Some(c) => {
                let config_value = c.config_value.clone().unwrap_or_default();
                self.save_to_cache(&c).await?;
                Ok(config_value)
            }
        }
    }

    /*
     清空参数缓存数据
    */
    pub async fn clear_config_cache(&self) -> Result<()> {
        let _ = CONTEXT.cache_service.del(&self.get_cache_key("*")).await?;
        Ok(())
    }

    /**
     * 重置参数缓存数据
     */

    pub async fn reset_config_cache(&self) -> Result<()> {
        let _ = self.clear_config_cache().await?;
        let _ = self.load_config_and_save_to_cache().await?;
        Ok(())
    }
    //加载所有的到redis
    pub async fn load_config_and_save_to_cache(&self) -> Result<u64> {
        let config_list = SysConfig::select_all(pool!()).await?;
        for config in config_list {
            self.save_to_cache(&config).await?;
        }
        Ok(1)
    }
    async fn save_to_cache(&self, config: &SysConfig) -> Result<()> {
        CONTEXT
            .cache_service
            .set_string(
                &self.get_cache_key(config.config_key.as_ref().unwrap()),
                config.config_value.as_ref().unwrap(),
            )
            .await?;
        Ok(())
    }
    //对config_key进行处理
    fn get_cache_key(&self, origin: &str) -> String {
        format!("{}{}", SYS_CONFIG_KEY, origin)
    }
    check_unique!(
        check_config_key_unique,
        "sys_config",
        config_key,
        config_id,
        "参数键名重复"
    );
    export_excel_service!(ConfigPageDTO, SysConfigVO,SysConfig::select_page);
}
