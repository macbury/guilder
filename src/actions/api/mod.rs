use rocket::fairing::AdHoc;

mod wallets;
mod accounts;
mod assets;
mod comments;
mod currencies;
mod errors;
mod logout;
mod health;
mod me;
mod bonds;
mod integrations;
mod categories;
mod live;
pub mod sign_in;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("Api", |rocket| async {
    rocket
      .attach(bonds::stage())
      .attach(categories::stage())
      .attach(assets::stage())
      .attach(comments::stage())
      .attach(accounts::stage())
      .attach(integrations::stage())
      .attach(wallets::stage())
      .mount("/api", routes![
        live::action,
        currencies::action,
        health::action,
        sign_in::action,
        me::action,
        logout::action
      ])
      .register("/api/", catchers![
        errors::not_found,
        errors::forbidden,
        errors::unprocessable
      ])
  })
}
