use sqlx::{PgPool, Postgres};
use sqlx_struct_enhanced::EnhancedCrud;
use crate::modals::TestTb;
use redis::{AsyncCommands};
use redis::{Client};

pub trait Service {
    fn service_by_state(state: &AppContext) -> Self;
}

#[derive(Clone)]
pub struct AppContext {
    pub(crate) db: PgPool,
    pub(crate) redis: Client
}

impl AppContext {
    pub fn get_service<T:Service>(&self) -> T {
        T::service_by_state(&self)
    }
    pub async fn get_redis(&self) -> redis::aio::Connection {
        if let Ok(conn) = self.redis.get_async_connection().await {
            conn
        }else{
            panic!("Redis Gone!")
        }
    }

    pub async fn redis_get(&self, key: &str, default_val: &str) -> String {
        let mut conn = self.get_redis().await;
        if let Ok(val) = conn.get::<&str, String>(key).await {
            val
        }else{
            default_val.to_string()
        }
    }

    pub async fn redis_set(&self, key: &str, val: &str) {
        let mut conn = self.get_redis().await;
        if let Err(err) = conn.set::<&str, &str, ()>(key, val).await {
            println!("set redis val with err:{err:?}");
        }
    }

}

pub struct TestService {
    ctx: AppContext
}
 impl Service for TestService {
     fn service_by_state(context: &AppContext) -> Self {
         Self{ctx: context.clone()}
     }
 }

impl TestService {
    pub async fn get_test_tb(&self) -> Vec<TestTb> {
        sqlx::query_as::<Postgres, TestTb>("select * from test_tb").fetch_all(&(self.ctx.db)).await.unwrap().to_vec()
    }

    pub async fn new_test_tb(&self, name: &str, ts: i32) {
        self.ctx.redis_set("test", "hahhaha").await;
        TestTb{
            id: "123".to_string(),
            name: name.to_string(),
            ts,
        }.insert_bind().execute(&self.ctx.db).await.expect("insert error");
    }
}
