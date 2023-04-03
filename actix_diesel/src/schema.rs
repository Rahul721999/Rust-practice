// @generated automatically by Diesel CLI.

diesel::table! {
    todo_item (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
    }
}

diesel::table! {
    todo_list (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
);
