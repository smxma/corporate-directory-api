use {
  diesel::{
      ExpressionMethods, 
      Insertable, 
      Queryable, 
      RunQueryDsl
  },
  diesel::query_dsl::methods::FilterDsl,
  diesel::result::Error,
  serde::{
      Deserialize, 
      Serialize
  },
  chrono::{NaiveDateTime},
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
      birthdate:String,
      age:String,
      address:String,
      company_name:String,
      start_date:String,
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
          Some(matched_employee) => {Some(matched_employee.to_employee(get_work_addr(matched_employee.company_name.to_string(), conn)))
        },
        _ => None,
        }
        Err(_) => None,
      }
}

pub fn create_new_employee(new_employee_request: NewEmployeeRequest, conn : &DBPooledConnection) -> Result<Employee,Error> {
    use crate::schema::employees::dsl::*;
    let work_addr = get_work_addr(new_employee_request.company_name.to_string(), &conn);
    let new_employee = Employee {
        id: Uuid::new_v4().to_string(),
        first_name: new_employee_request.first_name.to_string(),
        last_name: new_employee_request.last_name.to_string(),
        gender: new_employee_request.gender.to_string(),
        birthdate: NaiveDateTime::parse_from_str(&new_employee_request.birthdate, "%Y-%m-%d %H:%M:%S").unwrap() ,
        start_date: NaiveDateTime::parse_from_str(&new_employee_request.start_date, "%Y-%m-%d %H:%M:%S").unwrap(),
        age: new_employee_request.age.parse::<i32>().unwrap(),
        address: new_employee_request.address.to_string(),
        company_name: new_employee_request.company_name.to_string(),
        work_addr:work_addr.to_string(),
  };
  let new_employee_dao = new_employee.to_employee_dao();
  match diesel::insert_into(employees).values(&new_employee_dao).execute(conn){
      Ok(_) => Ok(new_employee),
      Err(e) => Err(e),
  }
}