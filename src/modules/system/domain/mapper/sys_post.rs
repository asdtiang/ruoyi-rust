use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page, py_sql};
use rbs::Error;

crud!(SysPost {});


impl_select_page!(SysPost{select_page(dto: &crate::system::domain::dto::PostPageDTO) =>
    "`where 1=1 `
    if dto.postName != '':
      ` and post_name like #{'%'+dto.postName+'%'}`
    if dto.postCode != '':
      ` and post_code like #{'%'+dto.postCode+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if do_count == false:
     ` order by post_sort`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysPost {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}



#[py_sql(
    "`select p.post_id, p.post_name, p.post_code`
		` from sys_post p`
			 ` left join sys_user_post up on up.post_id = p.post_id`
			 ` left join sys_user u on u.user_id = up.user_id`
		` where u.user_name = #{user_name}`"
)]
pub async fn select_posts_by_user_name(rb: &dyn Executor, user_name: &str) ->  Result<Vec<SysPost>, Error> {
    impled!()
}
