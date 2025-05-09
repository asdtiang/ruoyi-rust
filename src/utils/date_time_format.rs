use rbatis::rbdc::datetime::DateTime;
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &'static str = "YYYY-MM-DD hh:mm:ss";

    pub fn serialize<S>(
        date: &Option<DateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            None => {serializer.serialize_str("")}
            Some(d) => {
                let s = format!("{}", d.format(FORMAT));
                serializer.serialize_str(&s)
            }
        }
    }

    // deserialize_with 函数的签名必须遵循以下模式：
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // 尽管也可以对输出类型 T 进行泛型化。
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime>,D::Error>
    where
        D: Deserializer<'de>,
    {
        match  String::deserialize(deserializer){
            Ok(s) => {
               Ok( DateTime::parse(FORMAT, s.as_str()).ok())
            }
            Err(_) => {
                Ok(None)
            }
        }

    }


