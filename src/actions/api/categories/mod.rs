use rocket::fairing::AdHoc;
pub mod types;
mod all;
mod create;
mod show;
mod destroy;
mod update;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("REST Categories", |rocket| async {
    rocket
      .mount("/api/categories", routes![
        show::action,
        all::action,
        create::action,
        destroy::action,
        update::action
      ])
  })
}
