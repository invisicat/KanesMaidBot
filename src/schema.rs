table! {
    servers (id) {
        id -> Int4,
        guild -> Int8,
        joined -> Timestamptz,
        blacklisted -> Bool,
    }
}
