use validator::ValidationError;

pub fn string_required(str: &&String) -> Result<(), ValidationError> {
    if str.len() > 0 {
        Ok(())
    }else{
        Err(ValidationError::new("500"))
    }
   
}