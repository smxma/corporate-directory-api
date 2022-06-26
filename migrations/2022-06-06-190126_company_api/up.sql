-- Your SQL goes here
CREATE TABLE IF NOT EXISTS companies (
    company_name TEXT PRIMARY KEY NOT NULL,
    company_address TEXT NOT NULL,
    company_phone TEXT NOT NULL,
    domain TEXT NOT NULL,
    email TEXT NOT NULL,
    creation_date DATE NOT NULL,
    nb_employees INT NOT NULL
);

CREATE TABLE IF NOT EXISTS employees (
    id UUID PRIMARY KEY NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    gender TEXT NOT NULL,
    birthdate DATE NOT NULL,
    age INT NOT NULL,
    address TEXT NOT NULL,
    start_date DATE NOT NULL,
    company_name TEXT NOT NULL REFERENCES companies(company_name)
);