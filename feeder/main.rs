use dotenv::dotenv;
use tokio::task::JoinSet;

use common::{
    db::{create_db_connection, write_block_with_txns},
    rpc_api::blocks::{get_block_with_txs, get_latest_block},
};
use std::time::{Duration, Instant};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load env vars from .env file
    dotenv().ok();

    feeder_concurrent_latest().await?;
    Ok(())
}

pub const CHUNKS: u64 = 100;

pub async fn feeder_concurrent_latest() -> Result<()> {
    let latest_block = get_latest_block().await?;
    let mut feeder_tasks = JoinSet::new();

    for i in 0..=latest_block {
        if i % CHUNKS == 0 {
            if i + CHUNKS < latest_block {
                let start = latest_block - i - CHUNKS;
                let end = latest_block - i;
                feeder_tasks.spawn(async move { feeder_begin(start, end) });
            }
            if i + CHUNKS > latest_block {
                let start = 0;
                let end = latest_block - i;
                feeder_tasks.spawn(async move { feeder_begin(start, end) });
            }
        }
    }
    // for s in (0..latest_block).step_by(CHUNKS.into()) {
    //     println!("{:?}", s);
    // }

    let start = Instant::now();
    while let Some(res) = feeder_tasks.join_next().await {
        match res.unwrap().await {
            Ok(_) => println!("Indexed"),
            Err(_) => println!("Errored"),
        }
    }
    let duration = start.elapsed();
    println!("All blocks fetched in :{:?}", duration);
    Ok(())
}

pub async fn feeder_begin(starting_block: u64, upto_block: u64) -> Result<()> {
    let db = create_db_connection().await.unwrap();

    // let mut set = JoinSet::new();
    let tasks: Vec<_> = (starting_block..upto_block)
        .collect::<Vec<u64>>()
        .into_iter()
        .map(move |item| {
            tokio::spawn(async move {
                let block_with_txns = get_block_with_txs(item).await;
                block_with_txns
            })
        })
        .collect();

    let mut blocks_with_txns = vec![];
    for task in tasks {
        blocks_with_txns.push(task.await?);
    }

    let responses: Vec<_> = blocks_with_txns
        .into_iter()
        .map(move |block_with_txns| {
            let db = db.clone();
            tokio::spawn(async move {
                let block_with_txns = block_with_txns.unwrap();
                // println!("{}: {:?}", block_with_txns.block_number, block_with_txns);
                write_block_with_txns(db, block_with_txns).await
            })
        })
        .collect();

    let mut db_responses = vec![];
    for response in responses {
        db_responses.push(response.await?);
    }

    // println!("responses: {:?}", db_responses);
    println!("Indexed Block {:?}-{:?}", starting_block, upto_block);
    Ok(())
}
