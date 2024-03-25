use rocket::post;
use rocket::serde::{Deserialize, Serialize, json::Json};


#[derive(Deserialize)]
#[serde(crate="rocket::serde", tag="type")]
pub enum ExchangeRequest{
    Discord{
        code: String,
        state: String,
    },
}


#[derive(Serialize)]
#[serde(crate="rocket::serde")]
pub struct Private<'a>{
    version: &'a str
}

pub const V: Private<'static> = Private{
    version: env!("CARGO_PKG_VERSION")
};

#[post("/exchange_code", data="<exchange_request>")]
pub async fn exchange_code(exchange_request: Json<ExchangeRequest>) -> Json<Private<'static>>{
    match exchange_request.into_inner() {
        ExchangeRequest::Discord{code, state} => {
            println!("{}, {}", code, state);
        }
    };
    Json(V)
}