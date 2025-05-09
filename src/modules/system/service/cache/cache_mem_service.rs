use crate::error::Result;
use futures_util::future::BoxFuture;
use parking_lot::Mutex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Sub;
use std::time::{Duration, Instant};
use crate::system::service::cache_service::ICacheService;

///Memory Cache Service
pub struct MemService {
    pub cache: Mutex<HashMap<String, (String, Option<(Instant, Duration)>), RandomState>>,
    pub hget_cache: Mutex<HashMap<String, HashMap<String, String>>>,
}

impl MemService {
    pub fn recycling(&self) {
        let mut map_lock_guard = self.cache.lock();
        map_lock_guard.retain(|_, x| match x.1 {
            Some((i, d)) if i.elapsed() >= d => false,
            _ => true,
        });
    }
}

impl Default for MemService {
    fn default() -> Self {
        Self {
            cache: Default::default(),
            hget_cache: Default::default(),
        }
    }
}

impl ICacheService for MemService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut guard = self.cache.lock();
        guard.insert(k.to_string(), (v.clone(), None));
        Box::pin(async move {
            return Ok(v.to_string());
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let guard = self.cache.lock();
        let mut v = String::new();
        if let Some(r) = guard.get(&k) {
            v = r.0.to_string();
        }
        Box::pin(async move { Ok(v) })
    }

    fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut locked = self.cache.lock();
        let mut e = Option::None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        let inserted = locked.insert(k, (v.clone(), e));
        Box::pin(async move {
            if inserted.is_some() {
                return Ok(v.to_string());
            }
            return Err(crate::error::Error::E(format!(
                "[ruoyi_rust][mem_service]insert fail!"
            )));
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        self.recycling();
        let locked = self.cache.lock();
        let v = locked.get(k).cloned();
        drop(locked);
        let v = match v {
            None => -2,
            Some((_, o)) => match o {
                None => -1,
                Some((i, d)) => {
                    let use_time = i.elapsed();
                    if d > use_time {
                        d.sub(use_time).as_secs() as i64
                    } else {
                        0
                    }
                }
            },
        };
        Box::pin(async move { Ok(v) })
    }

    fn del(&self, k: &str) -> BoxFuture<Result<bool>> {
        self.recycling();
        let mut locked = self.cache.lock();
        let v = locked.remove(k);
        drop(locked);
        let v = match v {
            None => false,
            Some((_,_)) => true,
        };
        Box::pin(async move { Ok(v) })
    }

    fn keys(&self, _k: &str) -> BoxFuture<Result<Vec<String>>> {
        Box::pin(async move {
            let locked = self.cache.lock();
            let r = locked
                .keys()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            Ok(r)
        })
    }
    fn hgetall(&self, k: &str) -> BoxFuture<Result<Vec<String>>> {
        let k = k.to_string();
        let locked = self.hget_cache.lock();
        let r = locked.iter().find(|x| x.0 == &k);
        let r = r
            .map(|x| x.1.iter().map(|x| x.1.to_string()).collect::<Vec<String>>())
            .unwrap_or_default();
        Box::pin(async move { Ok(r) })
    }

    fn expire(&self, k: &str, _time_sec: i32) -> BoxFuture<Result<bool>> {
        let k = k.to_string();
        let mut locked = self.cache.lock();
        locked.retain(|_, x| x.0 != k);
        Box::pin(async move { Ok(true) })
    }

    fn hset(&self, k: &str, f: &str, v: &str) -> BoxFuture<Result<u64>> {
        let k = k.to_string();
        let mut locked = self.hget_cache.lock();
        let mut r: Option<&mut HashMap<String, String>> = locked.get_mut(&k);
        if r.is_none() {
            locked.insert(k.to_string(), HashMap::<String, String>::new());
            r = locked.get_mut(&k);
        }
        let r = r.unwrap();
        r.insert(f.to_string(), v.to_string());
        Box::pin(async move { Ok(1) })
    }

    fn select(&self, _db: &str) -> BoxFuture<Result<()>> {
        Box::pin(async move { Ok(()) })
    }
}
