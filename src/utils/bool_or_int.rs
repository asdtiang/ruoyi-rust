//https://github.com/rbatis/rbatis/issues/324
use serde::{ Deserializer, de};

pub fn bool_or_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolOrIntVisitor;

    impl<'de> de::Visitor<'de> for BoolOrIntVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or an integer")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }
        fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Map 0 to false, any other value to true
            Ok(Some(value != 0))
        }
        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Map 0 to false, any other value to true
            Ok(Some(value != 0))
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Map 0 to false, any other value to true
            Ok(Some(value != 0))
        }
        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Map 0 to false, any other value to true
            Ok(Some(value != 0))
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Map 0 to false, any other value to true
            Ok(Some(value != 0))
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }


    }

    deserializer.deserialize_any(BoolOrIntVisitor)
}
