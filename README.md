#Nom-Nom

This is a simple feeder code binary written in rust to index starknet data to a db.

!!! Currently in very early development stage.
- Only stores blocks details from Starknet in DBs.

### Requirements 
- Rust v1.8+ [link](https://www.rust-lang.org/)
- SurrealDB [link](https://www.rust-lang.org/)
- Starknet RPC Endpoint. You can get one from NM's RPC Service. (link)[https://data.voyager.online/]



### To run locally
- Copy `example.env` to a file `.env` and add your RPC Endpoint there.
- `cargo build` build project
- `cargo run --bin api` start api's on localhost:3000
- `cargo run --bin feeder` start feeder to populate db with rpc endpoint
