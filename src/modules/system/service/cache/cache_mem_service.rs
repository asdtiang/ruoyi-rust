use crate::error::Result;
use crate::system::service::cache_service::ICacheService;
use parking_lot::Mutex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Sub;
use std::time::{Duration, Instant};
use async_trait::async_trait;

///Memory Cache Service
#[derive(Debug)]
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
#[async_trait]
impl ICacheService for MemService {
    async  fn set_string(&self, k: &str, v: &str) -> Result<String > {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut guard = self.cache.lock();
        guard.insert(k.to_string(), (v.clone(), None));
      
            return Ok(v.to_string())
     
    }

    async fn get_string(&self, k: &str) -> Result<String > {
        self.recycling();
        let k = k.to_string();
        let guard = self.cache.lock();
        let mut v = String::new();
        if let Some(r) = guard.get(&k) {
            v = r.0.to_string();
        }
        Ok(v)
    }

    async  fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> Result<String > {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut locked = self.cache.lock();
        let mut e = Option::None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        let inserted = locked.insert(k, (v.clone(), e));
            if inserted.is_some() {
                return Ok(v.to_string());
            }
            return Err(crate::error::Error::E("[ruoyi_rust][mem_service]insert fail!".to_string()));
    }

    async   fn ttl(&self, k: &str) -> Result<i64 > {
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
        Ok(v) 
    }

    async    fn del(&self, k: &str) -> Result<bool > {
        self.recycling();
        let mut locked = self.cache.lock();
        let v = locked.remove(k);
        drop(locked);
        let v = match v {
            None => false,
            Some((_,_)) => true,
        };
        Ok(v) 
    }

    async  fn keys(&self, _k: &str) -> Result<Vec<String> > {
            let locked = self.cache.lock();
            let r = locked
                .keys()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            Ok(r)
    }
    async  fn hgetall(&self, k: &str) -> Result<Vec<String> > {
        let k = k.to_string();
        let locked = self.hget_cache.lock();
        let r = locked.iter().find(|x| x.0 == &k);
        let r = r
            .map(|x| x.1.iter().map(|x| x.1.to_string()).collect::<Vec<String>>())
            .unwrap_or_default();
        Ok(r) 
    }

    async   fn expire(&self, k: &str, _time_sec: i32) -> Result<bool > {
        let k = k.to_string();
        let mut locked = self.cache.lock();
        locked.retain(|_, x| x.0 != k);
        Ok(true) 
    }

    async fn hset(&self, k: &str, f: &str, v: &str) -> Result<u64 > {
        let k = k.to_string();
        let mut locked = self.hget_cache.lock();
        let mut r: Option<&mut HashMap<String, String>> = locked.get_mut(&k);
        if r.is_none() {
            locked.insert(k.to_string(), HashMap::<String, String>::new());
            r = locked.get_mut(&k);
        }
        let r = r.unwrap();
        r.insert(f.to_string(), v.to_string());
        Ok(1) 
    }

    async  fn select(&self, _db: &str) -> Result<() > {
        Ok(())
    }
}
