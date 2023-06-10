use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DATABASE_URI: String = env::var("DATABASE_URI").unwrap();
}
