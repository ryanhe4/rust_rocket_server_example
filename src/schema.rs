table! {
    comments (id) {
        id -> Integer,
        content -> Varchar,
        reply_to -> Nullable<Integer>,
        likes -> Integer,
        level -> Integer,
        has_replies -> Tinyint,
        deleted -> Tinyint,
        fk_user_id -> Integer,
        fk_post_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Datetime,
    }
}

table! {
    likes (post_id, user_id) {
        post_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        author -> Varchar,
        #[sql_name = "type"]
        type_ -> Integer,
        content -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        fk_user_id -> Integer,
    }
}

table! {
    social_accounts (id) {
        id -> Integer,
        provider -> Varchar,
        social_id -> Varchar,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        username -> Nullable<Varchar>,
        display_name -> Varchar,
        photo_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        is_certified -> Tinyint,
    }
}

joinable!(comments -> posts (fk_post_id));
joinable!(comments -> users (fk_user_id));
joinable!(likes -> posts (post_id));
joinable!(likes -> users (user_id));
joinable!(posts -> users (fk_user_id));
joinable!(social_accounts -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, likes, posts, social_accounts, users,);
