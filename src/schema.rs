table! {
    auths (id) {
        id -> Varchar,
        uid -> Varchar,
        refresh_token -> Varchar,
        access_token -> Varchar,
        auth_expiry -> Timestamp,
        nickname -> Nullable<Varchar>,
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

allow_tables_to_appear_in_same_query!(auths, users,);
