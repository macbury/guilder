use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, relative};
use rocket::response::{content};

#[get("/<_..>", rank=11)]
async fn index() -> content::RawHtml<&'static str> {
  content::RawHtml(include_str!("index.html"))
}

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("Single Page Application", |rocket| async {
    rocket
      .mount("/static", FileServer::from(relative!("./public/")).rank(10))
      .mount("/", routes![index])
  })
}
