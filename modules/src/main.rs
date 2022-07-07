mod config;
mod routes;

fn main() {
    config::print_config();
    routes::health_route::print_health_route();
    println!("main");
}
