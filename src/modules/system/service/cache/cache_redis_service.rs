use std::time::Duration;
use async_trait::async_trait;
use crate::error::{Error, Result};
use crate::system::service::cache_service::ICacheService;
use redis::aio::MultiplexedConnection;
use redis::RedisResult;

///Redis Cache service
#[derive(Debug)]
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Result<Self> {
        println!("[ruoyi_rust]  connect redis ({})...", url);
        let client = redis::Client::open(url).map_err(|e| Error::from(format!("open redis client failed={}", e)))?;
        println!("[ruoyi_rust]  connect redis success!");
        Ok(Self { client })
    }
    pub async fn get_conn(&self) -> Result<MultiplexedConnection> {
        let conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("RedisService connect fail:{}", e))?;
        Ok(conn)
    }
}
#[async_trait]
impl ICacheService for RedisService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let k = k.to_string();
        let v = v.to_string();
        self.set_string_ex(&k, &v, None).await
    }

    async fn get_string(&self, k: &str) -> Result<String> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        let result: RedisResult<Option<String>> = redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
        match result {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => Err(Error::from(format!(
                "RedisService get_string({}) fail:{}",
                k,
                e.to_string()
            ))),
        }
    }

    ///set_string Automatically expire
    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        let k = k.to_string();
        let v = v.to_string();
        let mut conn = self.get_conn().await?;
        if ex.is_none() {
            match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        } else {
            match redis::cmd("SET")
                .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                .query_async(&mut conn)
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        }
    }

    ///get time to live
    async fn ttl(&self, k: &str) -> Result<i64> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService ttl fail:{}", e.to_string()))),
        }
    }
    async fn del(&self, k: &str) -> Result<bool> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("DEL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService del fail:{}", e.to_string()))),
        }
    }

    async fn keys(&self, k: &str) -> Result<Vec<String>> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("KEYS").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService del fail:{}", e.to_string()))),
        }
    }

    async fn hgetall(&self, key: &str) -> Result<Vec<String>> {
        let k = key.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("HGETALL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService HGETALL fail:{}", e.to_string()))),
        }
    }

    async fn expire(&self, k: &str, time_sec: i32) -> Result<bool> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("EXPIRE").arg(k).arg(time_sec).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService del fail:{}", e.to_string()))),
        }
    }

    //从在db插入
    async fn hset(&self, k: &str, f: &str, v: &str) -> Result<u64> {
        let k = k.to_string();
        let f = f.to_string();
        let v = v.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("HSET").arg(&[k, f, v]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService hset fail:{}", e.to_string()))),
        }
    }
    async fn select(&self, db: &str) -> Result<()> {
        let db = db.to_string();
        let mut conn = self.get_conn().await?;
        match redis::cmd("SELECT").arg(&[db]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!("RedisService select fail:{}", e.to_string()))),
        }
    }
}
