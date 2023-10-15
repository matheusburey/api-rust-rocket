#[macro_use]
extern crate rocket;

mod controllers;
mod models;

use controllers::user_controllers::create_user;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/users", routes![create_user])
        .launch()
        .await?;
    Ok(())
}
