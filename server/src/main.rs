mod db;
mod errors;
mod metadata_extractor;
mod server;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    // Initialize DB creation
    match db::create() {
        Ok(()) => println!("Connection Established with DB!"),
        Err(err) => println!("Connection with DB failed - {}", err),
    };

    rocket::build().mount(
        "/",
        routes![server::index, server::find, server::list, server::upload],
    )
}
