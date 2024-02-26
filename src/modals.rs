use sqlx::FromRow;
use sqlx_struct_enhanced::{EnhancedCrud, Scheme};
use sqlx::query::{Query, QueryAs};
use sqlx::Postgres;
use sqlx::database::HasArguments;

#[derive(Debug, Clone, FromRow, EnhancedCrud)]
pub struct TestTb {
    pub id: String,
    pub name: String,
    pub ts: i32
}