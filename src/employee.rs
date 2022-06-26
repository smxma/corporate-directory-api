use {
    serde::Deserialize,
    serde::Serialize,
    chrono::{DateTime, Utc},
    
};




#[derive(Debug, Deserialize, Serialize)]

pub struct Employee{
    pub id:String, 
    pub first_name:String,
    pub last_name:String,
    pub gender:String,
    pub birthdate:DateTime<Utc>,
    pub age: i32,
    pub address:String,
    pub start_date:DateTime<Utc>,
    pub company_name:String,
    pub work_addr:String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct NewEmployeeRequest {
      first_name:String,
      last_name:String,
      gender:String,
    //   birthdate:DateTime<Utc>,
      age:i32,
      address:String,
    //   start_date:DateTime<Utc>,
      company_name:String,
}

pub struct EmployeeDAO {
    pub id:String, 
    pub first_name:String,
    pub last_name:String,
    pub gender:String,  
    pub birthdate:DateTime<Utc>,
    pub age: i32,
    pub address:String,
    pub start_date:DateTime<Utc>,
    pub company_name:String,
    pub work_addr:String,
}


// pub fn fetch_employee_by_id(_id: Uuid, conn: &DBPooledConnection) -> Option<Employee> {
//     use crate::schema::employees::dsl::*;
//     use crate::schema::companies::dsl::*;
//     match companies
//         .inner_join(employees)
//         .filter(id.eq(_id))
//         .load::<(CompanyDAO, EmployeeDAO)>(conn) {
//         Ok(result) => {
//             match result.first() {
//                 Some((w, m)) => {
//                     Some(m.to_miner(w.club_name.clone()))
//                 },
//                 _ => None,
//             }
//         },
//         Err(_) => None,
//     }
// }