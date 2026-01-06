use macros::replace_pool;
use rbatis::{Page, PageRequest};

use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::system::domain::dto::PostPageDTO;
use crate::system::domain::mapper;
use crate::system::domain::mapper::sys_post::SysPost;
use crate::system::domain::mapper::sys_user_post::SysUserPost;
use crate::system::domain::vo::SysPostVO;
use crate::{export_excel_service, pool, remove_batch_tx};

pub struct SysPostService {}

impl SysPostService {
    pub async fn page(&self, arg: &PostPageDTO) -> Result<Page<SysPostVO>> {
        let data = SysPost::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysPostVO>::from(data);

        Ok(page)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysPostVO>> {
        let data = SysPost::select_all(pool!()).await?;
        let mut post_vos = vec![];
        for s in data {
            post_vos.push(SysPostVO::from(s));
        }
        Ok(post_vos)
    }
    pub async fn detail(&self, post_id: &str) -> Result<SysPostVO> {
        let post = SysPost::select_by_map(pool!(), rbs::value! {"post_id": post_id} )
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", post_id)))?;
        let post_vo = SysPostVO::from(post);
        return Ok(post_vo);
    }

    pub async fn add(&self, arg: &SysPost) -> Result<u64> {
        let result = Ok(SysPost::insert(pool!(), &arg).await?.rows_affected);
        result
    }

    pub async fn update(&self, data: SysPost) -> Result<u64> {
        let result = SysPost::update_by_map(pool!(), &data, rbs::value! {"post_id":data. post_id.clone()}).await;
        Ok(result?.rows_affected)
    }
    #[replace_pool]
    pub async fn remove(&self, post_id: &str) -> Result<u64> {
        let targets = SysPost::select_by_map(pool!(), rbs::value! {"post_id": post_id}).await?;

        let r = SysPost::delete_by_map(pool!(), rbs::value! {"post_id": post_id}).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_post", &targets).await?;
            CONTEXT.sys_user_post_service.remove_by_post_id_tx(post_id,tx).await?;
        }
        Ok(r.rows_affected)
    }
    pub async fn finds_post_ids_by_user_id(&self, user_id: &str) -> Result<Vec<String>> {
        let user_posts = SysUserPost::select_by_map(pool!(), rbs::value! {"user_id": user_id}).await?;
        let ids = user_posts.into_iter().map(|r| r.post_id.unwrap_or_default()).collect();

        Ok(ids)
    }
    pub async fn select_post_names_by_user_name(&self, user_name: &str) -> Result<Vec<String>> {
        let user_posts = mapper::sys_post::select_posts_by_user_name(pool!(), user_name).await?;
        let ids = user_posts
            .into_iter()
            .map(|r| r.post_name.unwrap_or_default())
            .collect();

        Ok(ids)
    }
    remove_batch_tx!(post_ids);
    export_excel_service!(PostPageDTO, SysPostVO, SysPost::select_page);
}
