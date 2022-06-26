use {
    actix_web::HttpResponse,
    actix_web::web::Json, 
    crate::company::*,
    crate::utils::*,
};



//List all companies
#[get("/companies")]
pub async fn list_companies() -> HttpResponse {
    let companies : Vec<Company> = vec![];
    ResponseType::Ok(companies).get_response()
}

//Get a specific company by id
#[get("/companies/{id}")]
pub async fn get_company() -> HttpResponse {
    let company: Option<Company> = None;
    match company{
        Some(company) => ResponseType::Ok(company).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Company not found.".to_string())
        ).get_response(),
    }
}

//Create new company
#[post("/companies")]
pub async fn create_company(company_request: Json<NewCompanyRequest>) -> HttpResponse {
    let company: Vec<Company> = vec![];
    ResponseType::Created(company).get_response()
}


