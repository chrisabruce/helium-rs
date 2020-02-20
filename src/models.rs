use chrono;

pub struct Block {
    pub height: i64,
    pub time: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub prev_hash: Option<String>
    pub block_hash: String,
    pub transaction_count: i32,
    pub hbbft_round: i64,
    pub election_epock: i64,
    


}

pub struct BlockSignature {

}

pub enum TransactionType {

}

pub struct Transaction {

}

pub enum TransactionActorRole {

}

pub struct TransactionActor {

}

pub struct Account {

}

pub struct Gateway {

}

pub enum PendingTransactionStatus {

}

pub enum PendingTransactionNonceType {

}

pub struct PendingTransaction {

}



