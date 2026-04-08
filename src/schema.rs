// @generated automatically by Diesel CLI.

pub mod indexer {
    diesel::table! {
        indexer.characters (id) {
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 12]
            item_id -> Varchar,
            #[max_length = 12]
            tenant -> Varchar,
            #[max_length = 66]
            owner_cap_id -> Varchar,
            #[max_length = 66]
            owner_address -> Varchar,
            tribe_id -> Int8,
            name -> Text,
            description -> Nullable<Text>,
            url -> Nullable<Text>,
            checkpoint_updated -> Int8,
        }
    }

    diesel::table! {
        indexer.events_character_created (character_id, occurred_at) {
            occurred_at -> Timestamptz,
            #[max_length = 12]
            item_id -> Varchar,
            #[max_length = 12]
            tenant -> Varchar,
            #[max_length = 66]
            character_id -> Varchar,
            #[max_length = 66]
            owner_address -> Varchar,
            tribe_id -> Int8,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(characters, events_character_created);
}
