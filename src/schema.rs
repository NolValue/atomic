table! {
    auths (id) {
        id -> Varchar,
        uid -> Nullable<Varchar>,
        refresh_token -> Nullable<Varchar>,
        access_token -> Nullable<Varchar>,
        auth_expiry -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        url -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(auths -> users (uid));

allow_tables_to_appear_in_same_query!(
    auths,
    users,
);
