#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
use{
    actix_web::{middleware,App,HttpServer},
    std::{env,io},
    actix_web::web::Data,
    diesel::r2d2::ConnectionManager,
    diesel::PgConnection, 
    r2d2::{Pool, PooledConnection},
};

 

mod employee;
mod employee_controller;
mod schema;
mod utils;
mod company;
mod company_controller;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_to_pool(pool: Data<DBPool>) -> DBPooledConnection {
    pool.get().expect("Error getting connection from pool")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move|| {
        App::new()
        .data(pool.clone())
        .wrap(middleware::Logger::default())
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