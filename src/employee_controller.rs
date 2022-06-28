use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    uuid::Uuid,
    crate::DBPool, 
    crate::employee::*,
    crate::utils::{NotFoundMessage, ResponseType},

};

//List all employees
#[get("/employees")]
pub async fn list_employees(pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let employees : Vec<Employee> = feetch_all_employees(&conn);
    ResponseType::Ok(employees).get_response()
}

//Get a specific employee by id
#[get("/employees/{id}")]
pub async fn get_employee(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let employee: Option<Employee> = feetch_employee_by_id(
        Uuid::parse_str(path.0.as_str()).unwrap(), &conn);
    match employee{
        Some(employee) => ResponseType::Ok(employee).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Employee not found.".to_string())
        ).get_response(),
    }
}

//Create new employee
#[post("/companies/{company_name}/employees")]
pub async fn create_employee(path: Path<String>,employee_request: Json<NewEmployeeRequest>, pool: Data<DBPool>) -> HttpResponse {
    println!("employee_request ==> {:?}", employee_request);
    let company_name = path.to_string();
    let conn = crate::get_connection_to_pool(pool);
    match create_new_employee(employee_request.0, &conn){
        Ok(created_employee) => ResponseType::Created(created_employee).get_response(),
        Err(_) => ResponseType::NotFound(NotFoundMessage::new("Error creating employee!! ".to_string())).get_response(),
    }
}


