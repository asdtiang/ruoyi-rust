use crate::context::CONTEXT;
use crate::error::Result;

use crate::system::domain::vo::SysDictDataSimpleVO;

/**
 * 字典工具类
 *
 * @author ruoyi
 */

/**
 * 分隔符
 */

const SEPARATOR: &'static str = ",";
const DICT_KEY: &'static str = "dict_data";

/**
 * 获取字典缓存
 *
 * @param key 参数键
 * @return dictDatas 字典数据列表
 */
pub async fn get_dict_cache(dict_type: &str) -> Result<Vec<SysDictDataSimpleVO>> {
    let array_cache: Vec<SysDictDataSimpleVO> = CONTEXT
        .cache_service
        .get_json(&get_dict_redis_key(dict_type))
        .await?;
    Ok(array_cache)
}

/**
 * 根据字典类型和字典值获取字典标签
 *
 * @param dict_type 字典类型
 * @param dict_value 字典值
 * @return 字典标签
 */
pub async fn get_dict_label_default(dict_type: &str, dict_value: &str) -> Result<String> {
    get_dict_label(dict_type, dict_value, SEPARATOR).await
}

/**
 * 根据字典类型和字典标签获取字典值
 *
 * @param dict_type 字典类型
 * @param dict_label 字典标签
 * @return 字典值
 */
pub async fn get_dic_value_default(dict_type: &str, dict_label: &str) -> Result<String> {
    get_dict_value(dict_type, dict_label, SEPARATOR).await
}

/**
 * 根据字典类型和字典值获取字典标签
 *
 * @param dict_type 字典类型
 * @param dict_value 字典值
 * @param separator 分隔符
 * @return 字典标签
 */
pub async fn get_dict_label(dict_type: &str, dict_value: &str, separator: &str) -> Result<String> {
    let datas = get_dict_cache(dict_type).await?;

    if datas.len() > 0 {
        if dict_value.contains(separator) {
            let ss = dict_value
                .split(separator)
                .map(|value| {
                    for dict in datas.iter() {
                        if value.eq(&dict.dict_value) {
                            return dict.dict_label.clone();
                        }
                    }
                    return "".to_string();
                })
                .collect::<Vec<String>>();

            return Ok(ss.join(separator));
        } else {
            for dict in datas {
                if dict_value.eq(&dict.dict_value) {
                    return Ok(dict.dict_label);
                }
            }
        }
    }
    Ok("".to_string())
}

/**
 * 根据字典类型和字典标签获取字典值
 *
 * @param dict_type 字典类型
 * @param dict_label 字典标签
 * @param separator 分隔符
 * @return 字典值
 */
/**
 * 根据字典类型和字典值获取字典标签
 *
 * @param dict_type 字典类型
 * @param dict_value 字典值
 * @param separator 分隔符
 * @return 字典标签
 */
pub async fn get_dict_value(dict_type: &str, dict_label: &str, separator: &str) -> Result<String> {
    let datas = get_dict_cache(dict_type).await?;

    if datas.len() > 0 {
        if dict_label.contains(separator) {
            let ss = dict_label
                .split(separator)
                .map(|label| {
                    for dict in datas.iter() {
                        if label.eq(&dict.dict_label) {
                            return dict.dict_value.clone();
                        }
                    }
                    return "".to_string();
                })
                .collect::<Vec<String>>();

            return Ok(ss.join(separator));
        } else {
            for dict in datas {
                if dict_label.eq(&dict.dict_label) {
                    return Ok(dict.dict_value);
                }
            }
        }
    }
    Ok("".to_string())
}

/**
 *得到缓存Key
 * @param dict_type 字典类型
 * @return 缓存键key
 */
pub fn get_dict_redis_key(dict_type: &str) -> String {
    format!("{}:{}", DICT_KEY, dict_type)
}
