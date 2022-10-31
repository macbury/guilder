use rocket::response::content::RawJson;

#[get("/currencies")]
pub fn action() -> RawJson<&'static str> {
  let data = include_str!("./data.json");
  RawJson(data)
}
