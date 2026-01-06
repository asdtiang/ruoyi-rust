use std::fmt::Debug;
use crate::config::config::ApplicationConfig;
use crate::error::{Error, Result};
use crate::system::service::cache::{MemService, RedisService};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::format;
use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait ICacheService: Sync + Send + Debug  {
    async fn set_string(&self, k: &str, v: &str) -> Result<String>;

    async fn get_string(&self, k: &str) -> Result<String>;

    async  fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String>;

    async  fn ttl(&self, k: &str) -> Result<i64>;

    async fn del(&self, k: &str) -> Result<bool>;

    async fn keys(&self, k: &str) -> Result<Vec<String>>;

    async  fn hgetall(&self, k: &str) -> Result<Vec<String>>;

    async  fn expire(&self, k: &str, time_sec: i32) -> Result<bool>;
    //从在db插入
    async  fn hset(&self, k: &str, f: &str, v: &str) -> Result<u64>;
    //切换Db
    async  fn select(&self, db: &str) -> Result<()>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>
}

impl CacheService {
    pub fn new(cfg: &ApplicationConfig) -> Result<Self> {
        let cache = cfg.cache.as_str();
        if cache == "mem" {
            println!("[ruoyi_rust]  cache_type: mem");
            return Ok(Self {
                inner: Box::new(MemService::default()),
            });
        } else if cache.starts_with("redis") {
           // #[cfg(feature = "cache_redis")]
            {
                println!("[ruoyi_rust]  cache_type: redis");
                return Ok(Self {
                    inner: Box::new(RedisService::new(&cache)?),
                });
            }
        }
        Err(Error::from(format!(
            "[ruoyi_rust]  unknown of cache: \"{}\",current support 'mem' or 'redis'",
            cache
        )))
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
        where
            T: Serialize + Sync,
    {
        self.set_json_ex(k, v, None).await
    }

    pub async fn set_json_ex<T>(&self, k: &str, v: &T, ex: Option<Duration>) -> Result<String>
        where
            T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(Error::from(format!(
                "CacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string_ex(k, data.unwrap().as_str(), ex).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
        where
            T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }
    pub async fn del(&self, k: &str) -> Result<bool> {
        self.inner.del(k).await
    }
    pub async fn keys(&self, k: &str) -> Result<Vec<String>> {
        self.inner.keys(k).await
    }
    pub async  fn expire(&self, k: &str, time_sec: i32) -> Result<bool> {
        self.inner.expire(k,time_sec).await
    }
}
