use rocket::serde::json::Json;

use crate::models::user_model::NewUser;

#[post("/", data = "<new_user>")]
pub async fn create_user(new_user: Json<NewUser>) -> std::io::Result<()> {
    println!("{} -> {}", new_user.username, new_user.name);
    Ok(())
}
