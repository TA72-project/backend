// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `addresses` table.
    ///
    /// (Automatically generated by Diesel.)
    addresses (id) {
        /// The `id` column of the `addresses` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `number` column of the `addresses` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        number -> Nullable<Int4>,
        /// The `street_name` column of the `addresses` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        street_name -> Text,
        /// The `postcode` column of the `addresses` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        postcode -> Text,
        /// The `city_name` column of the `addresses` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        city_name -> Text,
        /// The `complement` column of the `addresses` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        complement -> Nullable<Text>,
    }
}

diesel::table! {
    /// Representation of the `centers` table.
    ///
    /// (Automatically generated by Diesel.)
    centers (id) {
        /// The `id` column of the `centers` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `name` column of the `centers` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `desc` column of the `centers` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        desc -> Nullable<Text>,
        /// The `workday_start` column of the `centers` table.
        ///
        /// Its SQL type is `Time`.
        ///
        /// (Automatically generated by Diesel.)
        workday_start -> Time,
        /// The `workday_end` column of the `centers` table.
        ///
        /// Its SQL type is `Time`.
        ///
        /// (Automatically generated by Diesel.)
        workday_end -> Time,
        /// The `range_km` column of the `centers` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        range_km -> Int2,
        /// The `id_address` column of the `centers` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_address -> Int8,
    }
}

diesel::table! {
    /// Representation of the `mission_types` table.
    ///
    /// (Automatically generated by Diesel.)
    mission_types (id) {
        /// The `id` column of the `mission_types` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `name` column of the `mission_types` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `people_required` column of the `mission_types` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        people_required -> Int2,
    }
}

diesel::table! {
    /// Representation of the `missions` table.
    ///
    /// (Automatically generated by Diesel.)
    missions (id) {
        /// The `id` column of the `missions` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `desc` column of the `missions` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        desc -> Nullable<Text>,
        /// The `start` column of the `missions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        start -> Timestamp,
        /// The `end` column of the `missions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        end -> Timestamp,
        /// The `recurrence_days` column of the `missions` table.
        ///
        /// Its SQL type is `Nullable<Int2>`.
        ///
        /// (Automatically generated by Diesel.)
        recurrence_days -> Nullable<Int2>,
        /// The `people_required` column of the `missions` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        people_required -> Int2,
        /// The `id_mission_type` column of the `missions` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_mission_type -> Int8,
        /// The `id_patient` column of the `missions` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_patient -> Int8,
    }
}

diesel::table! {
    /// Representation of the `nurses` table.
    ///
    /// (Automatically generated by Diesel.)
    nurses (id) {
        /// The `id` column of the `nurses` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `minutes_per_week` column of the `nurses` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        minutes_per_week -> Int4,
        /// The `id_user` column of the `nurses` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_user -> Int8,
        /// The `id_address` column of the `nurses` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_address -> Int8,
    }
}

diesel::table! {
    /// Representation of the `patients` table.
    ///
    /// (Automatically generated by Diesel.)
    patients (id) {
        /// The `id` column of the `patients` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `id_user` column of the `patients` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_user -> Int8,
        /// The `id_address` column of the `patients` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_address -> Int8,
    }
}

diesel::table! {
    /// Representation of the `skills` table.
    ///
    /// (Automatically generated by Diesel.)
    skills (id) {
        /// The `id` column of the `skills` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `name` column of the `skills` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
    }
}

diesel::table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `fname` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        fname -> Text,
        /// The `lname` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        lname -> Text,
        /// The `mail` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        mail -> Text,
        /// The `phone` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        phone -> Nullable<Text>,
        /// The `password` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        password -> Nullable<Text>,
        /// The `token` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Nullable<Text>,
        /// The `token_gentime` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        token_gentime -> Nullable<Timestamp>,
        /// The `id_center` column of the `users` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id_center -> Int8,
    }
}

diesel::joinable!(centers -> addresses (id_address));
diesel::joinable!(missions -> mission_types (id_mission_type));
diesel::joinable!(missions -> patients (id_patient));
diesel::joinable!(nurses -> addresses (id_address));
diesel::joinable!(nurses -> users (id_user));
diesel::joinable!(patients -> addresses (id_address));
diesel::joinable!(patients -> users (id_user));
diesel::joinable!(users -> centers (id_center));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    centers,
    mission_types,
    missions,
    nurses,
    patients,
    skills,
    users,
);
