mod config;
mod routes;
mod models;

use crate::config::print_config;
use crate::routes::health_route::print_health_route;
use crate::routes::user_route::print_user_route;

fn main() {
    print_main();
    print_config();
    print_health_route();
    print_user_route();
}

fn print_main(){
    println!("main");
}
