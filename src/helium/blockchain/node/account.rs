pub struct Account {
    pub address: String,
    pub name: String, 
    pub public_key: String,
    pub balance: u64,
    pub encrypted: String,
    pub transaction_fee: u64,
}

impl Account {
    pub fn list() -> Vec<Account> {
        Vec::new()
    }
}