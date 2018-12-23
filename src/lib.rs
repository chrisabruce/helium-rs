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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub time: i64,
    pub node_height: u64,
    pub interval: f64,
    pub chain_height: u64,
}

pub struct Node {
    host: &'static str,
    port: u16,
}

impl Node {
    pub fn new(host: &'static str, port: u16) -> Self {
        Node { host, port }
    }

    pub fn status(&self) -> Result<Status, reqwest::Error> {
        let request_url = self.url_for("/");
        let mut response = reqwest::get(&request_url)?;

        let status: Status = response.json()?;
        Ok(status)
    }

    pub fn list_accounts(&self) -> Result<Vec<Account>, reqwest::Error> {
        let request_url = self.url_for("/accounts");
        let mut response = reqwest::get(&request_url)?;

        let accounts: Vec<Account> = response.json()?;
        Ok(accounts)
    }

    pub fn get_account(&self, address: &str) -> Result<Account, reqwest::Error> {
        let request_url = self.url_for(format!("/accounts/{address}", address = address).as_str());
        let mut response = reqwest::get(&request_url)?;

        let account: Account = response.json()?;
        Ok(account)
    }
    pub fn create_account(&self) -> Result<Account, reqwest::Error> {
        let request_url = self.url_for("/accounts");
        let mut response = reqwest::Client::new().post(&request_url).send()?;
        let account: Account = response.json()?;

        Ok(account)
    }
    pub fn create_account_with_name(&self, name: &str) -> Result<Account, reqwest::Error> {
        let account: Account = self.create_account().unwrap();
        let account = self.rename_account(&account.address, &name).unwrap();

        Ok(account)
    }

    pub fn rename_account(&self, address: &str, name: &str) -> Result<Account, reqwest::Error> {
        let request_url =
            self.url_for(format!("/accounts/{address}/rename", address = address).as_str());
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
        nonce: u64,
    ) -> Result<(), reqwest::Error> {
        let request_url = self
            .url_for(format!("/accounts/{from_address}/pay", from_address = from_address).as_str());

        let body = format!(
            "{{\"toAddress\":\"{}\", \"amount\":{}, \"nonce\":{}}}",
            to_address,
            &amount.to_string(),
            nonce
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

    fn url_for(&self, path: &str) -> String {
        format!(
            "http://{host}:{port}{path}",
            host = self.host,
            port = self.port,
            path = path,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
