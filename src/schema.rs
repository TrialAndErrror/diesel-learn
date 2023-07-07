// @generated automatically by Diesel CLI.

diesel::table! {
    grocery (id) {
        id -> Int4,
        amount -> Text,
        name -> Text,
        done -> Bool,
        timestamp -> Timestamptz,
    }
}
