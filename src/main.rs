#[macro_use]

extern crate actix_web;
use{
    actix_web::{middleware,App,HttpServer},
    std::{env,io},
};
#[macro_use]
extern crate diesel;



mod employee;
mod employee_controller;
mod schema;
mod utils;
mod company;
mod company_controller;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default())
        .service(employee_controller::list_employees)
        .service(employee_controller::get_employee)   
        .service(employee_controller::create_employee)   
        .service(company_controller::list_companies)   
        .service(company_controller::get_company)   
        .service(company_controller::create_company)   
   
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}