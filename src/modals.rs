use sqlx::FromRow;
use sqlx_struct_enhanced::{EnhancedCrud, Scheme};
use sqlx::query::{Query, QueryAs};
use sqlx::Postgres;
use sqlx::database::HasArguments;
use uuid::Uuid;
use sqlx::types::BigDecimal;


#[derive(Debug, Clone, FromRow, EnhancedCrud)]
pub struct TestTb {
    pub id: Uuid,
    pub name: String,
    pub ts: i32,
    
}

