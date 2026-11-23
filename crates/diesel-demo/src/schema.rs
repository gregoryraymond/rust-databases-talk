diesel::table! {
    school (id) {
        id -> Binary,
        name -> Text,
        lon -> Double,
        lat -> Double,
        created_at -> Timestamp,
    }
}
