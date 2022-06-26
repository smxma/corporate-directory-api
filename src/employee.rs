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
  chrono::{DateTime, Utc, NaiveDateTime},
  uuid::Uuid,
  super::schema::employees,
  crate::company::*,
  crate::DBPooledConnection,
};




#[derive(Debug, Deserialize, Serialize)]

pub struct Employee{
    pub id:String, 
    pub first_name:String,
    pub last_name:String,
    pub gender:String,
    pub birthdate:NaiveDateTime,
    pub age: i32,
    pub address:String,
    pub start_date:NaiveDateTime,
    pub company_name:String,
    pub work_addr:String,
}

impl Employee {
    pub fn to_employee_dao(&self) -> EmployeeDAO {
        EmployeeDAO {
            id: Uuid::parse_str(self.id.as_str()).unwrap(),
            first_name: self.first_name.to_string(),
            last_name: self.last_name.to_string(),
            gender: self.gender.to_string(),
            birthdate: self.birthdate,
            age: self.age,
            address: self.address.to_string(),
            start_date: self.start_date,
            company_name: self.company_name.to_string(),
        }
    }

}


#[derive(Debug, Deserialize, Serialize)]
pub struct NewEmployeeRequest {
      first_name:String,
      last_name:String,
      gender:String,
    //   birthdate:NaiveDateTime,
      age:i32,
      address:String,
    //   start_date:NaiveDateTime,
      company_name:String,
}

#[derive(Queryable, Insertable)]
#[table_name = "employees"]
pub struct EmployeeDAO {
    pub id:Uuid, 
    pub first_name:String,
    pub last_name:String,
    pub gender:String,  
    pub birthdate:NaiveDateTime,
    pub age: i32,
    pub address:String,
    pub start_date:NaiveDateTime,
    pub company_name:String,
}

impl EmployeeDAO {
    pub fn to_employee(&self, work_addr: String) -> Employee{
        Employee{
            id: self.id.to_string(),
            first_name: self.first_name.to_string(),
            last_name: self.last_name.to_string(),
            gender: self.gender.to_string(),
            birthdate: self.birthdate,
            age: self.age,
            address: self.address.to_string(),
            start_date: self.start_date,
            company_name: self.company_name.to_string(),
            work_addr,
}
    }
}

pub fn get_work_addr(_company_name: String, conn : &DBPooledConnection) -> String {
  match feetch_company_by_company_name(_company_name, &conn) {
      Some(matched_company) => matched_company.company_address.to_string(),
      None => "Company address not found !!".to_string(),
  }
}


pub fn feetch_all_employees(conn: &DBPooledConnection) -> Vec<Employee> {
    use crate::schema::employees::dsl::*;
    match employees.load::<EmployeeDAO>(conn){
        Ok(result) => 
        {
          result.into_iter().map(|e| {
              let work_addr = get_work_addr(e.company_name.to_string(), conn);
              e.to_employee(work_addr)
          }).collect::<Vec<Employee>>()
        },
        Err(_) => vec![],
    }
}



pub fn feetch_employee_by_id(_id: Uuid, conn : &DBPooledConnection) -> Option<Employee> {
    use crate::schema::employees::dsl::*;
    match employees.filter(id.eq(_id)).load::<EmployeeDAO>(conn){
        Ok(result) => 
        match result.first(){
          Some(matched_employee) => {Some(matched_employee.to_employee(get_work_addr(matched_employee.company_name.to_string(), conn))),
        },
        _ => None,
        }
        Err(_) => None,
      }
}

