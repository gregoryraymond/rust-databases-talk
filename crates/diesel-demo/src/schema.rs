//! The PostGIS tables, as Diesel sees them.

diesel::table! {
    use diesel::sql_types::{Text, Timestamptz, Uuid};
    use postgis_diesel::sql_types::Geometry;

    /// Suburb boundaries.
    suburb (id) {
        /// A time-ordered id.
        id -> Uuid,
        /// The locality name.
        name -> Text,
        /// The state it is in.
        state -> Text,
        /// The boundary polygon.
        outline -> Geometry,
        /// The centroid, generated from the outline.
        center -> Geometry,
        /// When the row was made.
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Text, Timestamptz, Uuid};
    use postgis_diesel::sql_types::Geometry;

    /// School locations.
    school (id) {
        /// A time-ordered id.
        id -> Uuid,
        /// The school's name.
        name -> Text,
        /// Government, Catholic or Independent.
        sector -> Text,
        /// Primary, Secondary and so on.
        school_type -> Text,
        /// Where it is.
        location -> Geometry,
        /// When the row was made.
        created_at -> Timestamptz,
        /// When the row last changed.
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(suburb, school);
