use {
    serde::Deserialize,
    serde::Serialize,    
    chrono::{DateTime, Utc},
    uuid::Uuid,
};



#[derive(Debug, Deserialize, Serialize)]

pub struct Company {
    pub id_siret:String,
    pub company_name:String,
    pub company_address:String,
    pub company_phone:String,
    pub domain:String,
    pub email:String,
    pub creation_date:DateTime<Utc>,
    pub nb_employees:i32,
}


// impl Company {
//     pub fn to_company_dao(&self) -> CompanyDAO{
//         CompanyDAO{
//             company_name: Uuid::parse_str(self.company_name.as_str()).unwrap(),
//             company_address: self.company_address.to_string(),
//             company_phone: self.company_phone.to_string(),
//             domain: self.domain.to_string(),
//             email: self.email.to_string(),
//             creation_date: self.creation_date,
//             nb_employees: self.nb_employees
//         }
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCompanyRequest {
      company_name:String,
      company_address:String,
      company_phone:String,
      domain:String,
      email:String,
    //   creation_date:DateTime<Utc>,
}

pub struct CompanyDAO {
    pub id_siret:String,
    pub company_name:Uuid,
    pub company_address:String,
    pub company_phone:String,
    pub domain:String,
    pub email:String,
    pub creation_date:DateTime<Utc>,
}

// impl CompanyDAO {
//     pub fn to_company(&self) -> Company{
//         Company {
//             company_name: self.company_name.to_string(),
//             company_address: self.company_address.to_string(),
//             company_phone: self.company_phone.to_string(),
//             domain: self.domain.to_string(),
//             email: self.email.to_string(),
//             creation_date: self.creation_date,
//             nb_employees: self.nb_employees
//         }
//     }
// }



// pub fn fetch_company_by_id(_company_name: Uuid, conn: &DBPooledConnection) -> Option<Wallet> {
//     use crate::schema::employees::dsl::*;
//     use crate::schema::companies::dsl::*;
//     match companies
//         .filter(crate::schema::companies::address.eq(_company_name))
//         .load::<WalletDAO>(conn) {
//         Ok(result) => match result.first() {
//             Some(matched_wallet_dao) => {
//                 match miners
//                     .filter(crate::schema::miners::address.eq(_company_name))
//                     .load::<MinerDAO>(conn) {
//                         Ok(miner_result) => Some(matched_wallet_dao.to_wallet(
//                             miner_result.into_iter().map(|m| {
//                                 m.to_miner(matched_wallet_dao.club_name.clone())
//                         }).collect::<Vec<Miner>>())),
//                         Err(_) => Some(matched_wallet_dao.to_wallet(vec![])),
//                     }
//             },
//             _ => return None,
//         },
//         Err(_) => None,
//     }
// }