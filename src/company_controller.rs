use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    serde::{
        Deserialize, 
        Serialize
    },
    crate::DBPool, 
    crate::company::*,
    crate::utils::*,
};



//List all companies
#[get("/companies")]
pub async fn list_companies(pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let companies : Vec<Company> = feetch_all_companies(&conn);
    println!("Companies from list_companies ==> {:?}", companies);
    ResponseType::Ok(companies).get_response()
}

#[derive(Serialize, Deserialize)]
pub struct getCompanyReqPattern {
    pub company_name: String,
}
//Get a specific company by company name
#[get("/companies/{company_name}")]
pub async fn get_company(path: Path<(getCompanyReqPattern)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let company_name = path.company_name.to_string();
    let company: Option<Company> = feetch_company_by_company_name(company_name, &conn);
    match company{
        Some(company) => ResponseType::Ok(company).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Company not found.".to_string())
        ).get_response(),
    } 
}

//Create new company
#[post("/companies")]
pub async fn create_company(company_request: Json<NewCompanyRequest>, pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    match create_new_company(company_request.0,&conn){
        Ok(created_company) => ResponseType::Created(created_company).get_response(),
        Err(_) => ResponseType::NotFound(NotFoundMessage::new("Error creating company!! ".to_string())).get_response(),
    }
}


