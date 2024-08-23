#[macro_use] extern crate rocket;
use std::path::{PathBuf, Path};

use rocket::fs::{NamedFile, relative};

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("static")).join("index.html");
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
