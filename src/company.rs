use {
    diesel::{
        ExpressionMethods, 
        Insertable, 
        Queryable, 
        RunQueryDsl
    },
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    rand::Rng,
    serde::{
        Deserialize, 
        Serialize
    },
    chrono::{Utc, NaiveDateTime},
    uuid::Uuid,
    super::schema::companies,
    crate::DBPooledConnection,
    crate::employee::*,
  };
  


#[derive(Debug, Deserialize, Serialize)]

pub struct Company {
    pub id_siret:String,
    pub company_name:String,
    pub company_address:String,
    pub company_phone:String,
    pub domain:String,
    pub email:String,
    pub creation_date:NaiveDateTime,
    pub nb_employees:i128,
}


impl Company {
    pub fn to_company_dao(&self) -> CompanyDAO{
        CompanyDAO{
            company_name: self.company_name.to_string(),
            id_siret: self.id_siret.to_string(),
            company_address: self.company_address.to_string(),
            company_phone: self.company_phone.to_string(),
            domain: self.domain.to_string(),
            email: self.email.to_string(),
            creation_date: self.creation_date,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCompanyRequest {
      company_name:String,
      company_address:String,
      company_phone:String,
      domain:String,
      email:String,
}

#[derive(Queryable, Insertable)]
#[table_name = "companies"]
pub struct CompanyDAO {
    pub id_siret:String,
    pub company_name:String,
    pub company_address:String,
    pub company_phone:String,
    pub domain:String,
    pub email:String,
    pub creation_date:NaiveDateTime,
}

impl CompanyDAO {
    pub fn to_company(&self, nb_employees: i128) -> Company{
        Company {
            company_name: self.company_name.to_string(),
            company_address: self.company_address.to_string(),
            company_phone: self.company_phone.to_string(),
            domain: self.domain.to_string(),
            email: self.email.to_string(),
            creation_date: self.creation_date,
            id_siret: self.id_siret.to_string(),
            nb_employees,       
        }
    }
}


pub fn get_nb_employees(_company_dao: &CompanyDAO, conn : &DBPooledConnection) -> i128 {
    use crate::schema::employees::dsl::*;
    match employees.filter(company_name.eq(_company_dao.company_name.to_string())).count().get_result::<i128>(&*conn) {
        Ok(nb_employees) => nb_employees,
        Err(e) => {
            println!("Error: the number of employees not good {}", e);
            0
        }
    }
}


pub fn feetch_all_companies(conn: &DBPooledConnection) -> Vec<Company> {
    use crate::schema::companies::dsl::*;
    match companies.load::<CompanyDAO>(conn){
        Ok(result) => 
        {
          result.into_iter().map(|c| { 
            c.to_company(get_nb_employees(&c, conn))
          }).collect()
        }
        Err(e) => {
            println!("Error: {}", e);
            vec![]
        }
    }
}

pub fn feetch_company_by_company_name(_company_name: String, conn: &DBPooledConnection) -> Option<Company> {
    use crate::schema::employees::dsl::*;
    use crate::schema::companies::dsl::*;
    match companies
        .filter(crate::schema::companies::company_name.eq(_company_name))
        .load::<CompanyDAO>(conn) {
        Ok(result) => match result.first() { 
            Some(matched_company) => {
                let nb_employees = get_nb_employees(&matched_company, conn);
                Some(matched_company.to_company(nb_employees))
            },
            _ => None,
        },
        Err(_) => None,
    }
}


pub fn create_new_company(new_company_request: NewCompanyRequest, conn : &DBPooledConnection) -> Result<Company,Error> {
    use crate::schema::companies::dsl::*;
    let mut new_company = Company{
        company_name: new_company_request.company_name.to_string(),
        company_address: new_company_request.company_address.to_string(),
        company_phone: new_company_request.company_phone.to_string(),
        domain: new_company_request.domain.to_string(),
        email: new_company_request.email.to_string(),
        creation_date: Utc::now().naive_utc(),
        id_siret: Uuid::new_v4().to_string(),
        nb_employees: 0,
    };
    let new_company_dao = new_company.to_company_dao();
    match diesel::insert_into(companies).values(&new_company_dao).execute(conn) {
        Ok(_) => Ok(new_company_dao.to_company(i128::from(0))),
        Err(e) => Err(e),
    }
}