#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use reqwest::header::{ContentType, Headers};

#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub name: String,
    pub public_key: String,
    pub balance: u64,
    pub nonce: u64,
    pub encrypted: bool,
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
    host: &'static str,
    port: u16,
}

impl Client {
    pub fn new(host: &'static str, port: u16) -> Self {
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

    pub fn get_account(&self, address: &str) -> Result<Account, reqwest::Error> {
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

    pub fn create_account(&self, name: &str) -> Result<Account, reqwest::Error> {
        let request_url = format!(
            "http://{host}:{port}/accounts",
            host = self.host,
            port = self.host
        );
        let mut response = reqwest::Client::new().post(&request_url).send()?;
        let account: Account = response.json()?;

        let account = self.rename_account(&account.address, &name).unwrap();

        Ok(account)
    }

    pub fn rename_account(&self, address: &str, name: &str) -> Result<Account, reqwest::Error> {
        let request_url = format!(
            "http://{host}:{port}/accounts/{address}/rename",
            host = self.host,
            port = self.port,
            address = address
        );
        let params = [("name", name)];
        let mut response = reqwest::Client::new()
            .post(&request_url)
            .form(&params)
            .send()?;

        let account: Account = response.json()?;

        Ok(account)
    }

    pub fn pay(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u64,
    ) -> Result<(), reqwest::Error> {
        let request_url = format!(
            "http://{host}:{port}/accounts/{from_address}/pay",
            host = self.host,
            port = self.port,
            from_address = from_address
        );
        let body = format!(
            "{{\"toAddress\":\"{}\", \"amount\":{}}}",
            to_address,
            &amount.to_string()
        );

        let mut headers = Headers::new();
        headers.set(ContentType::json());

        let _response = reqwest::Client::new()
            .post(&request_url)
            .headers(headers)
            .body(body)
            .send()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
