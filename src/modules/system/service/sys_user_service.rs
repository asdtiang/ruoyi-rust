use macros::data_scope;
use rbatis::page::{Page, PageRequest};
use rbatis::{field_name, IPage};
use rbs::to_value;
use std::collections::HashMap;

use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::config::global_variables::{ADMIN_NAME, ADMIN_USERID};
use crate::context::CONTEXT;
use  crate::system::domain::dto::{DeptQueryDTO, UserAddDTO, UserPageDTO, UserUpdateDTO};
use  crate::system::domain::mapper::sys_user;
use  crate::system::domain::mapper::sys_user::SysUser;
use  crate::system::domain::vo::user::SysUserVO;
use  crate::system::domain::vo::{JWTToken, SysDeptVO, UserCache};
use crate::error::Error;
use crate::error::Result;
use crate::utils::password_encoder::PasswordEncoder;
use crate::web_data::{get_token, get_user_name};
use crate::{check_unique, pool, remove_batch};

pub struct SysUserService {}

impl SysUserService {
    #[data_scope(deptAlias = "d", userAlias = "u")]
    pub async fn page(&self, dto: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> =
            sys_user::select_page(pool!(), &PageRequest::from(&dto), &dto).await?;
        let mut page = Page::<SysUserVO>::from(sys_user_page);

        let all_depts = CONTEXT
            .sys_dept_service
            .list(&DeptQueryDTO::default())
            .await?;
        let mut dept_map = HashMap::new();
        all_depts.iter().for_each(|dept| {
            dept_map.insert(dept.dept_id.clone().unwrap_or_default(), dept.clone());
        });
        let mut new_records = vec![];
        for mut record in page.records().clone() {
            let dept_id = &record.dept_id.clone().unwrap_or_default();
            if dept_map.contains_key(dept_id) {
                let dept = dept_map.get(dept_id).map(|r| r.clone());
                record.dept = dept.map(|d| SysDeptVO::from(d));
                new_records.push(record);
            }
        }

        page.records = new_records;
        Ok(page)
    }

    ///user details
    pub async fn detail(&self, user_id: &str) -> Result<SysUser> {
        let user = self.find_by_user_id(&user_id).await?;
        Ok(user)
    }

    pub async fn find_by_user_id(&self, user_id: &str) -> Result<SysUser> {
        SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), user_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from("找不到此用户"))
    }

    pub async fn add(&self, dto: &UserAddDTO) -> Result<u64> {
        self.check_user_name_unique(&None, &dto.user_name.clone().unwrap_or_default())
            .await?;
        self.check_phonenumber_unique(&None, &dto.phonenumber.clone().unwrap_or_default())
            .await?;
        self.check_email_unique(&None, &dto.email.clone().unwrap_or_default())
            .await?;

        let role_ids = dto.role_ids.clone().unwrap_or_default();
        let post_ids = dto.post_ids.clone().unwrap_or_default();
        let mut password = dto.password.clone().unwrap_or_default();
        let mut user = SysUser::from(dto.clone());

        //todo 检查密码安全性
        if password.is_empty() {
            //默认密码
            password = "123456".to_string();
        }

        user.password = Some(PasswordEncoder::encode(&password));
        user.create_by = Some(get_user_name());
        let user_id = user.user_id.clone().unwrap_or_default();
        let res = SysUser::insert(pool!(), &user).await?;
        if res.rows_affected > 0 {
            if role_ids.len() > 0 {
                CONTEXT
                    .sys_user_role_service
                    .add_user_roles(&user_id, &role_ids)
                    .await?;
            }
            if post_ids.len() > 0 {
                CONTEXT
                    .sys_user_post_service
                    .add_user_posts(&user_id, &post_ids)
                    .await?;
            }
        }
        Ok(res.rows_affected)
    }

    pub async fn update(&self, dto: UserUpdateDTO) -> Result<u64> {
        let user_id = dto.user_id.clone();
        self.check_phonenumber_unique(&user_id, &dto.phonenumber.clone().unwrap_or_default())
            .await?;
        let user_id = user_id.unwrap_or_default();
        self.check_user_allowed(&user_id).await?;
        self.check_user_data_scope(&user_id).await?;

        let role_ids = dto.role_ids.clone();
        let post_ids = dto.post_ids.clone();
        let mut user = SysUser::from(dto);
        user.update_by = Some(get_user_name());
        if role_ids.is_some() {
            CONTEXT
                .sys_user_role_service
                .reset_through_user_id(&user_id, &role_ids.unwrap_or_default())
                .await?;
        }
        if post_ids.is_some() {
            CONTEXT
                .sys_user_post_service
                .reset_through_user_id(&user_id, &post_ids.unwrap_or_default())
                .await?;
        }
        Ok(SysUser::update_by_column(pool!(), &user, "user_id")
            .await?
            .rows_affected)
    }

    pub async fn remove(&self, user_id: &str) -> Result<u64> {

        let user_cache=CONTEXT.sys_user_service.get_user_cache_by_token(&get_token()).await?;
        if user_cache.id.eq(user_id){
            return Err(Error::from("不能删除自己！"));
        }

        if user_id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }
        self.check_user_allowed(user_id).await?;
        self.check_user_data_scope(user_id).await?;
        let r = pool!()
            .exec(
                "update sys_user set del_flag = '2' where user_id = ?",
                vec![to_value!(user_id)],
            )
            .await?;
        if r.rows_affected > 0 {
            CONTEXT
                .sys_user_role_service
                .remove_by_user_id(user_id)
                .await?;
            CONTEXT
                .sys_user_post_service
                .remove_by_user_id(user_id)
                .await?;
        }
        Ok(r.rows_affected)
    }

    pub async fn get_user_cache_by_token(&self, token: &str) -> Result<UserCache> {
        let token = JWTToken::verify(&CONTEXT.config.jwt_secret, &token);
        if token.is_err() {
            return Err(Error::from("Token失效，请重新登录！"));
        }
        CONTEXT
            .cache_service
            .get_json::<UserCache>(&format!("{}{}", LOGIN_TOKEN_KEY, &token?.login_user_key))
            .await
    }

    pub async fn update_password(&self, dto: UserUpdateDTO) -> Result<u64> {
        let user_id = dto.user_id.clone().unwrap_or_default();

        self.check_user_allowed(&user_id).await?;
        self.check_user_data_scope(&user_id).await?;

        let new_password = Some(PasswordEncoder::encode(&dto.password.clone().unwrap()));
        self.update_password_plain(&new_password.unwrap_or_default(), &user_id)
            .await
    }
    pub async fn update_password_plain(&self, new_password: &str, user_id: &str) -> Result<u64> {
        let new_password = Some(PasswordEncoder::encode(&new_password));
        let res = pool!()
            .exec(
                "update sys_user set password = ? where user_id = ?",
                vec![to_value!(new_password), to_value!(user_id)],
            )
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn update_status(&self, dto: &UserUpdateDTO) -> Result<u64> {
        let user_id = dto.user_id.clone().unwrap_or_default();
        self.check_user_allowed(&user_id).await?;
        let status = dto.status.unwrap_or_default();
        let res = pool!()
            .exec(
                "update sys_user set status = ? where user_id = ?",
                vec![to_value!(status), to_value!(user_id)],
            )
            .await?;
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
    check_unique!(
        check_email_unique,
        "sys_user",
        user_id,
        email,
        "邮箱账号已存在！"
    );
    /**
     * 校验用户是否有数据权限
     *
     * @param userId 用户id
     */
    pub async fn check_user_data_scope(&self, user_id: &str) -> Result<()> {
        if !get_user_name().eq(ADMIN_NAME) {
            let mut dto = UserPageDTO::default();
            dto.user_id = Some(user_id.to_string());
            let res = self.page(&dto).await?;
            if res.records.is_empty() {
                return Err(Error::from("没有权限访问用户数据！"));
            }
        }
        Ok(())
    }
    pub async fn check_user_exist(&self, user_id: &str) -> Result<SysUser> {
        SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), user_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from("找不到此用户"))
    }
    remove_batch!(user_ids);
}
