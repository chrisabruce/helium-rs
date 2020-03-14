use std::error::Error;

use helium;

pub struct Blockchain {
    db_url: String,
    pool_size: u32,
}

impl Blockchain {
    pub async fn new(db_url: &str, pool_size: u32) -> Result<Self, Box<dyn Error>> {
        Ok(Blockchain{
            db_url: db_url.to_string(), 
            pool_size,
        })
    }
}