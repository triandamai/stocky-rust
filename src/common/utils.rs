use validator::ValidationErrors;

pub fn get_validation_message(
    error:&ValidationErrors
)->String{
     error.field_errors().into_iter()
    .map(|v|{
        let message:String = v.1.into_iter().map(|er|{
            let message = match er.clone().message{
                Some(val)=>val.to_string(),
                None=>String::from("<no message>")
            };
            return format!("{}",message)
        }).collect();
        return format!("Field {} {} ",v.0,message);
    }).collect()
}

pub fn create_uuid()->String{
    uuid::Uuid::new_v4().as_simple().to_string()
}