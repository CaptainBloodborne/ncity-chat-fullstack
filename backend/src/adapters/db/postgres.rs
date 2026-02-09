pub mod user;
pub mod errors;
pub mod chat;

use sqlx::PgPool;

pub struct PostgresUserRepo {
    pool: PgPool
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self{ pool }
    }
}

pub struct PostgresChatRepo {
    pool: PgPool
}

impl PostgresChatRepo {
    pub fn new(pool: PgPool) -> Self {
        Self{ pool }
    }
}