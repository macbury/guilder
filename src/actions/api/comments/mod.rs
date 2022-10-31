use rocket::fairing::AdHoc;
mod all;
mod create;
mod destroy;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("REST Comments", |rocket| async {
    rocket
      .mount("/api/comments", routes![
        destroy::action,
        all::action,
        create::action
      ])
  })
}
