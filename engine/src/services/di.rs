use crate::services::db::Db;

pub struct Context{
    pub db: Box<dyn Db>,
}

impl Context {
    pub fn new(db : Box<dyn Db>) -> Self {
        Self { db }
    }
}