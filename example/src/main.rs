use rocket::{catchers, http::Method, launch};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions, Guard};

use rocket_db_pools::Database;

use backend;

#[launch]
fn rocket() -> _{
    dotenv::dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
    .attach(cors.to_cors().unwrap())
    // .attach(database::DB::init())
    // .attach(database::DBSetup{})
    // .register("/", catchers![
    //     views::errors::not_found,
    //     views::errors::internal_server_error,
    // ])
    .mount("/", backend::routes())
}