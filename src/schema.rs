table! {
    companies (company_name) {
        company_name -> Text,
        id_siret -> Text,
        company_address -> Text,
        company_phone -> Text,
        domain -> Text,
        email -> Text,
        creation_date -> Timestamp,
    }
}

table! {
    employees (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        gender -> Text,
        birthdate -> Timestamp,
        age -> Int4,
        address -> Text,
        start_date -> Timestamp,
        company_name -> Text,
    }
}

joinable!(employees -> companies (company_name));

allow_tables_to_appear_in_same_query!(
    companies,
    employees,
);
