pub fn create_user_session_key(
    user_id:&String
)->String{
    format!("auth:session:{}:profile",user_id)
}

pub fn create_user_verification_key(
    user_id:&String
)->String{
    format!("auth:verification:{}",user_id)
}

pub fn create_cart_key(user_id:&str){

}