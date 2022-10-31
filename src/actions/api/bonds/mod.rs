use rocket::fairing::AdHoc;
mod sort;
mod all;
mod show;
mod destroy;
mod update;
mod responses;
mod interest_rates;
mod performance;
mod balances;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("REST Bonds", |rocket| async {
    rocket
      .mount("/api/bonds", routes![
        all::action,
        balances::action,
        performance::action,
        interest_rates::action,
        show::action,
        destroy::action,
        update::action
      ])
  })
}
