#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    std::fs::read_to_string("README.md").unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

