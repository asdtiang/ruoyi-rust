use crate::config::global_constants::{ADMIN_USERID, STATUS_FORBIDDEN};
use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::system::domain::dto::UserPageDTO;
use crate::system::domain::mapper::sys_user;
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::vo::SysUserVO;
use crate::utils::password_encoder::PasswordEncoder;
use crate::{check_unique, export_excel_service, pool};
use macros::{data_scope, transactional};
use rbatis::page::{Page, PageRequest};

pub struct SysUserService {}

impl SysUserService {
    #[data_scope(deptAlias = "d", userAlias = "u", login_user_key)]
    pub async fn page(&self, dto: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> = sys_user::select_page(pool!(), &PageRequest::from(&dto), &dto).await?;
        let page = Page::<SysUserVO>::from(sys_user_page);

        Ok(page)
    }

    ///user details
    pub async fn detail(&self, user_id: &str) -> Result<SysUser> {
        let user = self.find_by_user_id(&user_id).await?;
        Ok(user)
    }

    pub async fn find_by_user_id(&self, user_id: &str) -> Result<SysUser> {
        SysUser::select_by_map(pool!(), rbs::value! {"user_id": user_id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from("找不到此用户"))
    }

    #[transactional(tx)]
    pub async fn add(&self, user: &SysUser, role_ids: &Vec<String>, post_ids: &Vec<String>) -> Result<u64> {
        self.check_user_name_unique(&None, user.user_name.clone().unwrap_or_default())
            .await?;
        self.check_phonenumber_unique(&None, user.phonenumber.clone().unwrap_or_default())
            .await?;
        self.check_email_unique(&None, user.email.clone().unwrap_or_default())
            .await?;

        let user_id = user.user_id.clone().unwrap_or_default();
        let res = SysUser::insert(&tx, &user).await?;
        if res.rows_affected > 0 {
            if role_ids.len() > 0 {
                CONTEXT
                    .sys_user_role_service
                    .add_user_roles_tx(&user_id, &role_ids, &tx)
                    .await?;
            }
            if post_ids.len() > 0 {
                CONTEXT
                    .sys_user_post_service
                    .add_user_posts_tx(&user_id, &post_ids, &tx)
                    .await?;
            }
        }
        Ok(res.rows_affected)
    }
    #[transactional(tx)]
    pub async fn update(&self, user: &SysUser, role_ids: &Vec<String>, post_ids: &Vec<String>) -> Result<u64> {
        let user_id = user.user_id.clone();
        self.check_phonenumber_unique(&user_id, user.phonenumber.clone().unwrap_or_default())
            .await?;
        let user_id = user_id.unwrap_or_default();
        self.check_user_allowed(&user_id).await?;
        // fixme  self.check_user_data_scope(&user_id, user_).await?;

        CONTEXT
            .sys_user_role_service
            .reset_through_user_id_tx(&user_id, role_ids, &tx)
            .await?;
        CONTEXT
            .sys_user_post_service
            .reset_through_user_id_tx(&user_id, post_ids, &tx)
            .await?;

        let res = SysUser::update_by_map(&tx, &user, rbs::value! {"user_id": user_id.clone()})
            .await?
            .rows_affected;
        if res > 0 {
            CONTEXT
                .sys_user_online_service
                .force_logout_by_user_id(&user_id)
                .await?;
        }

        Ok(res)
    }

    pub async fn update_profile(&self, sys_user: SysUser) -> Result<u64> {
        let user_id = sys_user.user_id.clone();
        self.check_phonenumber_unique(&user_id, sys_user.phonenumber.clone().unwrap_or_default())
            .await?;
        self.check_email_unique(&user_id, sys_user.phonenumber.clone().unwrap_or_default())
            .await?;
        Ok(SysUser::update_by_map(
            pool!(),
            &sys_user,
            rbs::value! {"user_id": user_id,
            "column":vec!["nick_name","email","phonenumber","sex"]},
        )
        .await?
        .rows_affected)
    }
    #[transactional(tx)]
    pub async fn remove(&self, user_id: &str, user_cache: &crate::UserCache) -> Result<u64> {
        if user_cache.user_id.eq(user_id) {
            return Err(Error::from("不能删除自己！"));
        }
        if user_id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }
        self.check_user_allowed(user_id).await?;
        self.check_user_data_scope(user_id, user_cache).await?;
        let r = &tx
            .exec(
                "update sys_user set del_flag = '2' where user_id = ?",
                vec![rbs::value!(user_id)],
            )
            .await?;
        if r.rows_affected > 0 {
            CONTEXT.sys_user_role_service.remove_by_user_id_tx(user_id, &tx).await?;
            CONTEXT.sys_user_post_service.remove_by_user_id_tx(user_id, &tx).await?;
            CONTEXT.sys_user_online_service.force_logout_by_user_id(user_id).await?;
        }

        Ok(r.rows_affected)
    }

    pub async fn reset_pwd(&self, user_id: &str, raw_pwd: &str, user_cache: &crate::UserCache) -> Result<u64> {
        self.check_user_data_scope(user_id, &user_cache).await?;
        self.check_user_allowed(&user_id).await?;
        let new_password_enc = Some(PasswordEncoder::encode(&raw_pwd));
        let res = pool!()
            .exec(
                "update sys_user set password = ?,last_chn_pwd_time=null where user_id = ?",
                vec![rbs::value!(new_password_enc), rbs::value!(user_id)],
            )
            .await?
            .rows_affected;
        if res > 0 {
            CONTEXT.sys_user_online_service.force_logout_by_user_id(user_id).await?;
        }
        Ok(res)
    }
    pub async fn change_pwd(
        &self,
        new_password: &str,
        old_password: &str,
        user_cache: &crate::UserCache,
    ) -> Result<u64> {
        if new_password.eq(old_password) {
            return Err(Error::from("新密码不能与旧密码相同"));
        }
        let user_id = &user_cache.user_id;
        let user = CONTEXT.sys_user_service.find_by_user_id(user_id).await?;

        if !PasswordEncoder::verify(&user.password.unwrap_or_default(), &old_password) {
            return Err(Error::from("修改密码失败，旧密码错误"));
        }
        let new_password_enc = Some(PasswordEncoder::encode(new_password));
        let res = pool!()
            .exec(
                "update sys_user set password = ?,last_chn_pwd_time=NOW() where user_id = ?",
                vec![rbs::value!(new_password_enc), rbs::value!(user_id.clone())],
            )
            .await?
            .rows_affected;
        if res > 0 {
            CONTEXT
                .sys_user_online_service
                .force_logout_by_user_id(&user_id)
                .await?;
        }
        Ok(res)
    }

    pub async fn update_status(&self, user_id: &str, status: char) -> Result<u64> {
        self.check_user_allowed(user_id).await?;
        let res = pool!()
            .exec(
                "update sys_user set status = ? where user_id = ?",
                vec![rbs::value!(status), rbs::value!(user_id)],
            )
            .await?;
        if status == STATUS_FORBIDDEN {
            CONTEXT.sys_user_online_service.force_logout_by_user_id(user_id).await?;
        }
        Ok(res.rows_affected)
    }

    /**
     * 校验用户是否允许操作 fixme 是否采用user_id
     *
     * @param user 用户信息
     */
    pub async fn check_user_allowed(&self, user_id: &str) -> Result<()> {
        if user_id.eq(ADMIN_USERID) {
            return Err(Error::from("不允许操作超级管理员用户"));
        }
        Ok(())
    }

    check_unique!(
        check_user_name_unique,
        "sys_user",
        user_id,
        user_name,
        "用户名已经存在！"
    );
    check_unique!(
        check_phonenumber_unique,
        "sys_user",
        user_id,
        phonenumber,
        "手机号码重复！"
    );
    check_unique!(check_email_unique, "sys_user", user_id, email, "邮箱账号已存在！");
    /**
     * 校验用户是否有数据权限 fixme
     *
     * @param userId 用户id
     */
    pub async fn check_user_data_scope(&self, user_id: &str, user: &crate::UserCache) -> Result<()> {
        if !user.is_admin() {
            let mut dto = UserPageDTO::default();
            dto.page_no = Some(1);
            dto.page_size = Some(10);
            dto.user_id = Some(user_id.to_string());
            let res = self.page(&dto, &user).await?;
            if res.records.is_empty() {
                return Err(Error::from("没有权限访问用户数据！"));
            }
        }
        Ok(())
    }
    pub async fn check_user_exist(&self, user_id: &str) -> Result<SysUser> {
        SysUser::select_by_map(pool!(), rbs::value! {"user_id": user_id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from("找不到此用户"))
    }
    pub async fn remove_batch(&self, user_ids: &str, user: &crate::UserCache) -> Result<u64> {
        let user_ids = user_ids.split(",").collect::<Vec<&str>>();
        for id in user_ids {
            self.remove(id, user).await?;
        }
        Ok(1)
    }
    export_excel_service!(UserPageDTO, SysUserVO, sys_user::select_page);
}
