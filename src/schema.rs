// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        completed -> Bool,
        due_date -> Nullable<Date>,
    }
}
