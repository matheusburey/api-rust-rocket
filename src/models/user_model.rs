use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub password: String,
}
