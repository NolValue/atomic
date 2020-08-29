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
    posts (id) {
        id -> Varchar,
        source_type -> Nullable<Int2>,
        source_id -> Nullable<Varchar>,
        public -> Nullable<Bool>,
        reshares -> Nullable<Bool>,
        comments -> Nullable<Bool>,
        poster -> Varchar,
        content -> Text,
        media -> Nullable<Json>,
        created_on -> Timestamp,
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
joinable!(posts -> users (poster));

allow_tables_to_appear_in_same_query!(auths, posts, users,);
