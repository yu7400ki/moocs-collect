use crate::db::Migration;

pub fn get_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        sql: include_str!("../migrations/20250321173837_cache.sql"),
    }]
}
