use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use crate::conf::Configuration;
use crate::service::{AppContext, TestService};

pub async fn run_command(cmd: &str, cfg: &Configuration) {
    println!("Present command [{cmd}]");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cfg.db_uri)
        .await
        .expect("can't connect to database");
    let client = redis::Client::open(cfg.redis_uri.clone()).unwrap();
    let state = AppContext{ db: pool, redis:client };
    if cmd == "new_test_tb" {
        new_test_tb(state).await;
    }

}

pub async fn new_test_tb(ctx: AppContext) {
    ctx.get_service::<TestService>().new_test_tb("banana", 123).await;
}