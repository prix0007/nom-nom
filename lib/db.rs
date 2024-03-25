use std::env;

use serde_json::json;
use starknet::core::types::requests::GetTransactionByHashRequest;
use starknet::core::types::{BlockWithTxHashes, FieldElement, Transaction};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub type SurrealDb = Surreal<surrealdb::engine::remote::ws::Client>;

pub async fn create_db_connection() -> Result<SurrealDb, surrealdb::Error> {
    let db_connection_str: String =
        env::var("DB_CONNECTION_URI").unwrap_or("127.0.0.1:8000".into());
    // Connect to the server
    let db = Surreal::new::<Ws>(db_connection_str).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    let env_namespace: String =
        env::var("DB_NAMESPACE").expect("DB_NAMESPACE required to set in env");
    let env_db_name: String = env::var("DB_NAME").expect("DB_NAME required to set in env");

    // Select a specific namespace / database
    db.use_ns(env_namespace).use_db(env_db_name).await?;
    Ok(db)
}

pub async fn read_block_with_txns(
    db: SurrealDb,
    block_id: String,
) -> Result<Option<BlockWithTxHashes>, surrealdb::Error> {
    Ok(db.select(("blocks", block_id)).await?)
}

pub async fn read_block_with_txns_from_block_number(
    db: SurrealDb,
    block_number: u64,
) -> Result<Option<BlockWithTxHashes>, surrealdb::Error> {
    let mut response = db
        .query("SELECT * from type::table($table) WHERE block_number = $block_number;")
        .bind(json!({
                "table": "blocks",
                "block_number": block_number}
        ))
        .await?;
    let value: Option<BlockWithTxHashes> = response.take(0)?;
    Ok(value)
}

pub async fn write_block_with_txns(
    db: SurrealDb,
    data: BlockWithTxHashes,
) -> Result<Option<BlockWithTxHashes>, surrealdb::Error> {
    let record: Option<BlockWithTxHashes> = db
        .create(("blocks", data.block_hash.to_string()))
        .content::<BlockWithTxHashes>(data.into())
        .await?;
    // dbg!(&record);
    Ok(record)
}

pub async fn read_transaction_from_transaction_hash(
    db: SurrealDb,
    transaction_hash: FieldElement,
) -> Result<Option<Transaction>, surrealdb::Error> {
    let mut response = db
        .query("SELECT * from type::table($table) WHERE transaction_hash = $transaction_hash;")
        .bind(json!({
                "table": "transactions",
                "transaction_hash": transaction_hash}
        ))
        .await?;
    let value: Option<Transaction> = response.take(0)?;
    Ok(value)
}

pub async fn write_txn_by_hash(
    db: SurrealDb,
    data: Transaction,
) -> Result<Option<Transaction>, surrealdb::Error> {
    let record: Option<Transaction> = db
        .create(("transactions", data.transaction_hash().to_string()))
        .content::<Transaction>(data.into())
        .await?;
    // dbg!(&record);
    Ok(record)
}
