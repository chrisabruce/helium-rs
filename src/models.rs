use chrono::{DateTime, Utc};

pub struct Block {
    pub height: i64,
    pub time: i64,
    pub timestamp: DateTime<Utc>,
    pub prev_hash: Option<String>,
    pub block_hash: String,
    pub transaction_count: i32,
    pub hbbft_round: i64,
    pub election_epoch: i64,
    pub rescue_signature: String,
}

pub struct BlockSignature {
    pub block: i64,
    pub signer: String,
    pub signature: String,
}

pub enum TransactionType {
        CoinbaseV1,
        SecurityCoinbaseV1,
        OuiV1,
        GenGatewayV1,
        RoutingV1,
        PaymentV1,
        SecurityExchangeV1,
        ConsensusGroupV1,
        AddGatewayV1,
        AssertLocationV1,
        CreateHtlcV1,
        RedeemHtlcV1,
        PocRequestV1,
        PocReceiptsV1,
        VarsV1,
        RewardsV1,
        TokenBurnV1,
        DcCoinbaseV1,
        TokenBurnExchangeRateV1,
}

pub struct Transaction {
    pub block: i64,
    pub hash: String,
    pub r#type: TransactionType,
    pub fields: String,
}

pub enum TransactionActorRole {
    Payee,
    Payer,
    Owner,
    Gateway,
    RewardGateway,
    Challenger,
    Challengee,
    Witness,
    ConsensusMember,
    Escrow,
}

pub struct TransactionActor {
    pub actor: String,
    pub actor_role: TransactionActorRole,
    pub transaction_hash: String,
}

pub struct Account {
    pub block: i64,
    pub timestamp: DateTime<Utc>,
    pub address: String,
    pub dc_balance: i64,
    pub dc_nonce: i64,
    pub security_balance: i64,
    pub security_nonce: i64,
    pub balance: i64,
    pub nonce: i64,
}

pub struct Gateway {
    pub block: i64,
    pub address: String,
    pub owner: String,
    pub location: Option<String>,
    pub alpha: f64,
    pub beta: f64,
    pub delta: i32,
    pub score: f64,
    pub last_poc_challenge: Option<i64>,
    pub last_poc_onion_challenge: Option<String>,
    pub witnesses: String,
}

pub enum PendingTransactionStatus {
    Received,
    Pending,
    Failed,
}

pub enum PendingTransactionNonceType {
    Balance,
    Security,
    Dc,
}

pub struct PendingTransaction {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub hash: String,
    pub r#type: TransactionType,
    pub address: String,
    pub nonce: i64,
    pub nonce_type: PendingTransactionNonceType,
    pub failed_reason: Option<String>,
    pub data: Vec<u8>,
}



