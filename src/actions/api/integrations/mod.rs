use rocket::fairing::AdHoc;
pub mod types;
mod all;
mod create;
mod show;
mod sync;
mod destroy;
mod update;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("REST Integrations", |rocket| async {
    rocket
      .mount("/api/integrations", routes![
        show::action,
        sync::action,
        all::action,
        create::action,
        destroy::action,
        update::action
      ])
  })
}
