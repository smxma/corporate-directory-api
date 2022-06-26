use {
    actix_web::HttpResponse,
    actix_web::web::Json, 
    crate::employee::*,
    crate::utils::{NotFoundMessage, ResponseType},

};

//List all employees
#[get("/employees")]
pub async fn list_employees() -> HttpResponse {
    let employees : Vec<Employee> = vec![];
    ResponseType::Ok(employees).get_response()
}

//Get a specific employee by id
#[get("/employees/{id}")]
pub async fn get_employee() -> HttpResponse {
    let employee: Option<Employee> = None;
    match employee{
        Some(employee) => ResponseType::Ok(employee).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Employee not found.".to_string())
        ).get_response(),
    }
}

//Create new employee
#[post("/companies/{id}/employees")]
pub async fn create_employee(employee_request: Json<NewEmployeeRequest>) -> HttpResponse {
    let employee: Vec<Employee> = vec![];
    ResponseType::Created(employee).get_response()
}


