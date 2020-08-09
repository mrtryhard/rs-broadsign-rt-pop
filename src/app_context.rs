extern crate log;
extern crate r2d2;
extern crate r2d2_sqlite;

use crate::broadsign::real_time_pop_request::RealTimePopRequest;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use std::sync::Arc;

pub struct Database {
    pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
}

// Quick and simple AppContext that will contain the database context
// so we can store the pops.
pub struct AppContext {
    pub database: Database,
}

impl Database {
    pub fn from_sqlite(file_name: &'static str) -> Self {
        let manager = SqliteConnectionManager::file(file_name);
        let pool = Arc::new(r2d2::Pool::builder().build(manager).unwrap());

        info!("Initializing pops database 'pops.db'.");

        let conn = pool.get().unwrap();
        let result = conn.execute_batch(
            r#"
        BEGIN;
        create table if not exists api_users (
            user_id integer primary key,
            api_key text not null unique
        );
        create table if not exists pops (
            pop_id integer primary key,
            user_id integer not null,
            player_id integer not null,
            display_unit_id integer not null,
            frame_id integer not null,
            active_screens_count integer not null,
            ad_copy_id integer not null,
            schedule_id integer not null,
            impressions integer not null,
            interactions integer not null,
            end_time integer not null,
            duration_ms integer not null,
            service_name text not null,
            service_value text not null,
            extra_data text not null,
            FOREIGN KEY(user_id) REFERENCES api_users(user_id)
        );
        COMMIT;"#,
        );

        if let Err(e) = result {
            error!("Could not create tables: {}", e);
            panic!("Could not create tables.");
        }

        Database {
            pool: Arc::clone(&pool),
        }
    }

    pub fn user_exists(&self, api_key: &String) -> bool {
        let pool_result = self.pool.get();

        match pool_result {
            Err(e) => {
                error!("Could not get database connection: {}", e);
                false
            }
            Ok(conn) => {
                let result: Result<bool, _> = conn.query_row(
                    "select 1 from api_users where api_key = ?1;",
                    params![api_key],
                    |row| row.get(0),
                );

                match result {
                    Ok(exists) => exists,
                    Err(e) => {
                        error!("{}", e);
                        false
                    }
                }
            }
        }
    }

    // Only present for the unit tests.
    pub fn create_user(&self, api_key: &'static str) {
        let pool_result = self.pool.get();

        match pool_result {
            Err(e) => {
                error!("Could not get database connection: {}", e);
            }
            Ok(conn) => {
                let result = conn.execute(
                    "insert or ignore into api_users (api_key) values (?1);",
                    params![api_key],
                );

                if let Err(e) = result {
                    error!("{}", e);
                }
            }
        }
    }

    pub fn store_pop(&self, pops: &RealTimePopRequest) -> bool {
        let pool_result = self.pool.get();

        match pool_result {
            Err(e) => {
                error!("Could not get database connection: {}", e);
                return false;
            }
            Ok(mut conn) => {
                if let Ok(tx) = conn.transaction() {
                    for pop in &pops.pops {
                        let result = tx.execute(
                            r#"insert into pops (
                        user_id,
                        player_id,
                        display_unit_id,
                        frame_id,
                        active_screens_count,
                        ad_copy_id,
                        schedule_id,
                        impressions,
                        interactions,
                        end_time,
                        duration_ms,
                        service_name,
                        service_value,
                        extra_data)
                        select
                            user_id,
                            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13
                        from
                            api_users
                        where
                            api_key = ?14;"#,
                            params![
                                pops.player_id as i64,
                                pop.display_unit_id as i64,
                                pop.frame_id as i64,
                                pop.active_screens_count as i32,
                                pop.ad_copy_id as i64,
                                pop.schedule_id as i64,
                                pop.impressions as i32,
                                pop.interactions as i32,
                                pop.end_time.timestamp_millis(),
                                pop.duration_ms as i32,
                                pop.service_name,
                                pop.service_value,
                                pop.extra_data.as_str().unwrap(),
                                pops.api_key
                            ],
                        );

                        if let Err(err) = result {
                            error!("{}", err);
                            let _ = tx.rollback();
                            return false;
                        }
                    }

                    match tx.commit() {
                        Ok(_) => return true,
                        Err(e) => {
                            error!("{}", e);
                            return false;
                        }
                    }
                }

                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests_database {
    use super::*;
    use crate::broadsign::real_time_pop_request::{RealTimePopEntry, RealTimePopRequest};
    use rusqlite::NO_PARAMS;
    use serde_json::json;

    fn ensure_user(db: &Database) {
        let conn = db.pool.get().unwrap();
        let r = conn.execute(
            "insert or ignore into api_users (api_key) values ('some_secure_api_key')",
            NO_PARAMS,
        );

        if let Err(e) = r {
            panic!("{}", e);
        }
    }

    #[actix_rt::test]
    async fn given_initialization_step_do_not_fail() {
        let _ = Database::from_sqlite("test.db");
        assert!(true);
    }

    #[actix_rt::test]
    async fn given_an_existing_api_key_user_exists_should_return_true() {
        let db = Database::from_sqlite("test.db");
        ensure_user(&db);
        let exists = db.user_exists(&"some_secure_api_key".to_owned());

        assert_eq!(exists, true);
    }

    #[actix_rt::test]
    async fn given_a_valid_pop_request_should_succeed_to_insert() {
        let db = Database::from_sqlite("test.db");
        ensure_user(&db);
        let result = db.store_pop(&RealTimePopRequest {
            api_key: "some_secure_api_key".to_owned(),
            player_id: 123456,
            pops: vec![RealTimePopEntry {
                display_unit_id: 123,
                frame_id: 124,
                active_screens_count: 2,
                ad_copy_id: 56467,
                campaign_id: 61000,
                schedule_id: 61001,
                impressions: 675,
                interactions: 0,
                end_time: chrono::NaiveDate::from_ymd(2017, 11, 23).and_hms_milli(13, 27, 12, 500),
                duration_ms: 12996,
                service_name: "bmb".to_owned(),
                service_value: "701".to_owned(),
                extra_data: json!(""),
            }],
        });

        assert_eq!(result, true);
    }
}
