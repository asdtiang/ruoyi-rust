use crate::error::Result;
use crate::system::domain::mapper::sys_user_post::SysUserPost;
use macros::replace_pool;

///User Post Service
pub struct SysUserPostService {}

impl SysUserPostService {

    #[replace_pool]
    pub async fn add_user_posts(&self, user_id: &str, post_ids: &Vec<String>) -> Result<u64> {
        let rows = post_ids
            .into_iter()
            .map(|r_id| SysUserPost {
                user_id: user_id.to_string().into(),
                post_id: r_id.to_string().into(),
            })
            .collect::<Vec<_>>();

        Ok(SysUserPost::insert_batch(pool!(), &rows, 20).await?.rows_affected)
    }
    #[replace_pool]
    pub async fn add_users_post(&self, post_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids
            .into_iter()
            .map(|u_id| SysUserPost {
                user_id: u_id.to_string().into(),
                post_id: post_id.to_string().into(),
            })
            .collect::<Vec<_>>();

        Ok(SysUserPost::insert_batch(pool!(), &rows, 20).await?.rows_affected)
    }
    #[replace_pool]
    pub async fn remove(&self, user_post: &SysUserPost) -> Result<u64> {
        let res = pool!()
            .exec(
                "delete from sys_user_post where user_id=? and post_id=?",
                vec![
                    rbs::value!(user_post.user_id.clone()),
                    rbs::value!(user_post.post_id.clone()),
                ],
            )
            .await
            .unwrap();
        Ok(res.rows_affected)
    }
    #[replace_pool]
    pub async fn remove_users_post(&self, post_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids
            .into_iter()
            .map(|u_id| SysUserPost {
                user_id: u_id.to_string().into(),
                post_id: post_id.to_string().into(),
            })
            .collect::<Vec<_>>();

        let mut cnt = 0;
        for r in rows {
            let res = self.remove_tx(&r,tx).await;
            cnt = cnt + res.unwrap();
        }
        Ok(cnt)
    }
    #[replace_pool]
    pub async fn remove_by_post_id(&self, post_id: &str) -> Result<u64> {
        Ok(
            SysUserPost::delete_by_map(pool!(), rbs::value! {"post_id": post_id})
                .await?
                .rows_affected,
        )
    }
    #[replace_pool]
    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserPost::delete_by_map(pool!(), rbs::value! {"user_id": user_id})
                .await?
                .rows_affected,
        )
    }
    #[replace_pool]
    pub async fn reset_through_user_id(&self, user_id: &str, post_ids: &Vec<String>) -> Result<u64> {
        self.remove_by_user_id_tx(user_id,tx).await?;
        if !post_ids.is_empty() {
            self.add_user_posts_tx(user_id, post_ids,tx).await
        } else {
            Ok(0)
        }
    }
}
