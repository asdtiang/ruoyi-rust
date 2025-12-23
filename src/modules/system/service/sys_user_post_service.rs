use crate::error::Result;
use crate::system::domain::mapper::sys_user_post::SysUserPost;
use macros::replace_pool;
use rbatis::field_name;
use rbs::to_value;

///User Post Service
pub struct SysUserPostService {}

impl SysUserPostService {
    // pub async fn page(&self, arg: &UserPostPageDTO) -> Result<Page<SysUserVO>> {
    //     let vo = CONTEXT
    //         .sys_user_service
    //         .page(&UserPageDTO::from(arg))
    //         .await?;
    //     // if arg.resp_set_post.unwrap_or(true) {
    //     //     let all_post = CONTEXT.sys_post_service.finds_all_map().await?;
    //     //     let user_ids = rbatis::table_field_vec!(&vo.records, id);
    //     //     let user_posts = SysUserPost::select_in_column(pool!(), "id", &user_ids).await?;
    //     //     let user_post_map = rbatis::make_table_field_map!(&user_posts, user_id);
    //     //     let post_ids = rbatis::table_field_vec!(&user_posts, post_id);
    //     //     let posts = CONTEXT.sys_post_service.finds(&post_ids).await?;
    //     //     let posts_map = rbatis::make_table_field_map!(&posts, id);
    //     //     for mut x in &mut vo.records {
    //     //         if let Some(user_post) = user_post_map.get(x.id.as_deref().unwrap_or_default()) {
    //     //             if let Some(post_id) = &user_post.post_id {
    //     //                 let post = posts_map.get(post_id).cloned();
    //     //                 x.post = SysPostVO::from_option(post);
    //     //                 //查找子集角色
    //     //                 if let Some(post_vo) = &mut x.post {
    //     //                     CONTEXT
    //     //                         .sys_post_service
    //     //                         .loop_find_childs(post_vo, &all_post);
    //     //                 }
    //     //             }
    //     //         }
    //     //     }
    //     // }
    //     Ok(vo)
    // }
    //
    // pub async fn add(&self, arg: UserPostDTO) -> Result<u64> {
    //     if arg.user_id.is_none() || arg.post_id.is_none() {
    //         return Err(Error::from("添加角色时用户和角色不能为空！"));
    //     }
    //     let user_id = arg.user_id.as_deref().unwrap().to_string();
    //     let user_post = SysUserPost::from(arg);
    //     self.remove_by_user_id(user_id.as_str()).await?;
    //     Ok(SysUserPost::insert(pool!(), &user_post).await?.rows_affected)
    // }
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
                    to_value!(user_post.user_id.clone()),
                    to_value!(user_post.post_id.clone()),
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
            SysUserPost::delete_by_column(pool!(), field_name!(SysUserPost.post_id), post_id)
                .await?
                .rows_affected,
        )
    }
    #[replace_pool]
    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserPost::delete_by_column(pool!(), field_name!(SysUserPost.user_id), user_id)
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
