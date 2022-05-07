table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        sig_scheme -> Varchar,
        keypair -> Varchar,
        pubkey -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(users,);
