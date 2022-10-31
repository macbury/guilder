use rocket::fairing::AdHoc;
pub mod types;
mod all;
mod create;
mod show;
mod destroy;
mod update;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("REST Wallets", |rocket| async {
    rocket
      .mount("/api/wallets", routes![
        show::action,
        all::action,
        create::action,
        destroy::action,
        update::action
      ])
  })
}
