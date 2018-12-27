#[macro_use]
extern crate serde_derive;
//extern crate reqwest;

use reqwest::header::{ContentType, Headers};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub name: Option<String>,
    pub public_key: String,
    pub balance: u64,
    pub encrypted: bool,
    pub transaction_fee: u64,
    pub has_association: bool,
}

#[derive(Deserialize, Debug)]
pub struct Gateway {
    pub address: String,
    pub h3_index: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub blocks_mined: u64,
    pub score: f64,
    pub last_proc_challenge: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct GatewaysResponse {
    pub total: u64,
    pub per_page: u64,
    pub page: u64,
    pub entries: Vec<Gateway>,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    time: i64,
    round: u64,
    height: u64,
    hash: String,
}

#[derive(Deserialize, Debug)]
pub enum Transaction {
    Payment(PaymentTx),
    AddHotspot(AddHotspotTx),
    AssertLocation(AssertLocationTx),
}

#[derive(Deserialize, Debug)]
pub struct PaymentTx {
    pub tx_type: String,
    pub time: i64,
    pub payer: String,
    pub payee: String,
    pub nonce: u64,
    pub index: u64,
    pub height: u64,
    pub hash: String,
    pub fee: u64,
    pub block_hash: String,
    pub amount: u64,
}

#[derive(Deserialize, Debug)]
pub struct AddHotspotTx {
    pub tx_type: String,
    pub time: i64,
    pub owner: String,
    pub index: u64,
    pub height: u64,
    pub hash: String,
    pub gateway: String,
    pub block_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct AssertLocationTx {
    pub tx_type: String,
    pub time: i64,
    pub owner: String,
    pub nonce: u64,
    pub location: u64,
    pub index: u64,
    pub height: u64,
    pub hash: String,
    pub gateway: String,
    pub fee: u64,
    pub block_hash: String,
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

    pub fn delete_account(&self, address: &str) -> Result<(), reqwest::Error> {
        let request_url = self.url_for(format!("/accounts/{address}", address = address).as_str());
        reqwest::Client::new().delete(&request_url).send()?;
        Ok(())
    }

    pub fn pay(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u64,
    ) -> Result<(), reqwest::Error> {
        let request_url = self
            .url_for(format!("/accounts/{from_address}/pay", from_address = from_address).as_str());

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

    pub fn list_gateways_raw(
        &self,
        _page: Option<u64>,
        _per_page: Option<u64>,
    ) -> Result<GatewaysResponse, reqwest::Error> {
        let request_url = self.url_for("/gateways");

        let mut response = reqwest::Client::new().get(&request_url).send()?;
        let gateway_response: GatewaysResponse = response.json()?;

        Ok(gateway_response)
    }

    pub fn list_blocks(&self, before: Option<u64>) -> Result<Vec<Block>, reqwest::Error> {
        let request_url = self.url_for("/explorer/blocks");

        let mut params = HashMap::new();
        if let Some(val) = before {
            params.insert("before", val);
        }

        let mut response = reqwest::Client::new()
            .get(&request_url)
            .query(&params)
            .send()?;
        let blocks: Vec<Block> = response.json()?;

        Ok(blocks)
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
