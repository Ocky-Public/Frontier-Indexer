// @generated automatically by Diesel CLI.

pub mod indexer {
    diesel::table! {
        indexer.assemblies (id) {
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            #[max_length = 66]
            owner_cap_id -> Varchar,
            #[max_length = 66]
            location -> Varchar,
            status -> Text,
            #[max_length = 66]
            energy_source_id -> Nullable<Varchar>,
            name -> Nullable<Text>,
            description -> Nullable<Text>,
            url -> Nullable<Text>,
            checkpoint_updated -> Int8,
        }
    }

    diesel::table! {
        indexer.characters (id) {
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
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
        indexer.energy_config (assembly_id, package_id) {
            #[max_length = 66]
            package_id -> Varchar,
            #[max_length = 20]
            assembly_id -> Varchar,
            energy_cost -> Int8,
            #[max_length = 66]
            entry_object_id -> Varchar,
            checkpoint_updated -> Int8,
        }
    }

    diesel::table! {
        indexer.events_assembly_created (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            #[max_length = 66]
            owner_cap_id -> Varchar,
        }
    }

    diesel::table! {
        indexer.events_character_created (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            #[max_length = 66]
            owner_address -> Varchar,
            tribe_id -> Int8,
        }
    }

    diesel::table! {
        indexer.events_energy_production_started (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            current_energy_production -> Int8,
        }
    }

    diesel::table! {
        indexer.events_energy_production_stopped (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
        }
    }

    diesel::table! {
        indexer.events_energy_released (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            type_id -> Int8,
            released -> Int8,
            reserved_total -> Int8,
        }
    }

    diesel::table! {
        indexer.events_energy_reserved (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            type_id -> Int8,
            reserved -> Int8,
            reserved_total -> Int8,
        }
    }

    diesel::table! {
        indexer.events_fuel_burning_started (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            quantity_old -> Int8,
            quantity_new -> Int8,
            burning -> Bool,
        }
    }

    diesel::table! {
        indexer.events_fuel_burning_stopped (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            quantity_old -> Int8,
            quantity_new -> Int8,
            burning -> Bool,
        }
    }

    diesel::table! {
        indexer.events_fuel_burning_updated (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            quantity_old -> Int8,
            quantity_new -> Int8,
            burning -> Bool,
        }
    }

    diesel::table! {
        indexer.events_fuel_deleted (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            quantity_old -> Int8,
            quantity_new -> Int8,
            burning -> Bool,
        }
    }

    diesel::table! {
        indexer.events_fuel_deposited (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            quantity_old -> Int8,
            quantity_new -> Int8,
            burning -> Bool,
        }
    }

    diesel::table! {
        indexer.events_fuel_efficiency_removed (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            type_id -> Int8,
        }
    }

    diesel::table! {
        indexer.events_fuel_efficiency_set (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            type_id -> Int8,
            efficiency -> Int8,
        }
    }

    diesel::table! {
        indexer.events_fuel_withdrawn (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            quantity_old -> Int8,
            quantity_new -> Int8,
            burning -> Bool,
        }
    }

    diesel::table! {
        indexer.events_location_revealed (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            type_id -> Int8,
            #[max_length = 66]
            owner_cap_id -> Varchar,
            #[max_length = 66]
            location_hash -> Varchar,
            solar_system_id -> Int8,
            x -> Text,
            y -> Text,
            z -> Text,
        }
    }

    diesel::table! {
        indexer.events_owner_cap_created (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 66]
            object_id -> Varchar,
        }
    }

    diesel::table! {
        indexer.events_owner_cap_transferred (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 66]
            object_id -> Varchar,
            #[max_length = 66]
            previous_owner -> Varchar,
            #[max_length = 66]
            owner -> Varchar,
        }
    }

    diesel::table! {
        indexer.events_status_changed (event_id, occurred_at) {
            #[max_length = 100]
            event_id -> Varchar,
            occurred_at -> Timestamptz,
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 20]
            item_id -> Varchar,
            tenant -> Text,
            status -> Text,
            action -> Text,
        }
    }

    diesel::table! {
        indexer.fuel_config (type_id, table_id) {
            #[max_length = 66]
            table_id -> Varchar,
            #[max_length = 20]
            type_id -> Varchar,
            efficiency -> Int8,
            #[max_length = 66]
            entry_object_id -> Varchar,
            checkpoint_updated -> Int8,
        }
    }

    diesel::table! {
        indexer.killmails (id, occurred_at) {
            #[max_length = 66]
            id -> Varchar,
            kill_id -> Int8,
            tenant -> Text,
            occurred_at -> Timestamptz,
            solar_system_id -> Int8,
            loss_type -> Text,
            killer_id -> Int8,
            victim_id -> Int8,
            reporter_id -> Int8,
        }
    }

    diesel::table! {
        indexer.owner_caps (id) {
            #[max_length = 66]
            id -> Varchar,
            #[max_length = 66]
            object_id -> Varchar,
            #[max_length = 66]
            owner_address -> Varchar,
            #[max_length = 66]
            package_id -> Varchar,
            module_name -> Text,
            struct_name -> Text,
            checkpoint_updated -> Int8,
        }
    }

    diesel::table! {
        indexer.system_table_registry (table_id) {
            #[max_length = 66]
            table_id -> Varchar,
            #[max_length = 66]
            parent_id -> Varchar,
            #[max_length = 66]
            package_id -> Varchar,
            module_name -> Text,
            struct_name -> Text,
            key_type -> Text,
            value_type -> Text,
            checkpoint_updated -> Int8,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        assemblies,
        characters,
        energy_config,
        events_assembly_created,
        events_character_created,
        events_energy_production_started,
        events_energy_production_stopped,
        events_energy_released,
        events_energy_reserved,
        events_fuel_burning_started,
        events_fuel_burning_stopped,
        events_fuel_burning_updated,
        events_fuel_deleted,
        events_fuel_deposited,
        events_fuel_efficiency_removed,
        events_fuel_efficiency_set,
        events_fuel_withdrawn,
        events_location_revealed,
        events_owner_cap_created,
        events_owner_cap_transferred,
        events_status_changed,
        fuel_config,
        killmails,
        owner_caps,
        system_table_registry,
    );
}
