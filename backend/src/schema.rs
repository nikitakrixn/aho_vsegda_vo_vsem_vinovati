// @generated automatically by Diesel CLI.

diesel::table! {
    departments (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        middle_name -> Nullable<Varchar>,
        department_id -> Nullable<Int4>,
        position_id -> Nullable<Int4>,
        hire_date -> Date,
        termination_date -> Nullable<Date>,
        #[max_length = 255]
        ad_login -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
    }
}

diesel::table! {
    positions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(employees -> departments (department_id));
diesel::joinable!(employees -> positions (position_id));

diesel::allow_tables_to_appear_in_same_query!(
    departments,
    employees,
    positions,
);
