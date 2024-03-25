use rocket::{routes, Route};

mod exchange_code;


pub fn routes() -> Vec<Route>{
    let site_routes = routes![
        exchange_code::exchange_code,
    ];

    site_routes
}