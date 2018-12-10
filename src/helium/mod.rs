extern crate reqwest;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub name: String,
    pub public_key: String,
    pub balance: u64,
    pub encrypted: String,
    pub transaction_fee: u64,
    pub has_association: bool,
}

#[derive(Deserialize, Debug)]
pub struct Gateway {
    pub address: String,
    pub h3_index: String,
    pub lat: String,
    pub lng: String,
    pub blocks_mined: u64,
    pub score: i32,
    pub last_proc_challenge: String,
    pub status: String,
}

pub struct Client {
    host: String,
    port: u16,
}

impl Client {
    pub fn new(host: String, port: u16) -> Client {
        Client { host, port }
    }

    pub fn list_accounts(&self) -> Result<Vec<Account>, reqwest::Error> {
        let request_url = format!(
            "http://{host}:{port}/accounts",
            host = self.host,
            port = self.port
        );
        let mut response = reqwest::get(&request_url)?;

        let accounts: Vec<Account> = response.json()?;
        Ok(accounts)
    }

    pub fn get_account(&self, address: String) -> Result<Account, reqwest::Error> {
        let request_url = format!(
            "http://{host}:{port}/accounts/{address}",
            host = self.host,
            port = self.port,
            address = address
        );
        let mut response = reqwest::get(&request_url)?;

        let account: Account = response.json()?;
        Ok(account)
    }
}
