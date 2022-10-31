use rocket::fairing::AdHoc;
mod sort;
mod search;
mod show;
mod all;
mod destroy;
mod update;

//https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/chat/src/main.rs
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("REST Assets", |rocket| async {
    rocket
      .mount("/api/assets", routes![
        all::action,
        search::action,
        show::action,
        destroy::action,
        update::action
      ])
  })
}
