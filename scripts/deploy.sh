#!/bin/bash
set -e

CONTRACT=nft.dieselattack.testnet
#MASTER_ACCOUNT=dieselattack.testnet

# delete and create subaccount
#near delete $CONTRACT $MASTER_ACCOUNT  # uncomment to delete old account
# near create-account $CONTRACT --masterAccount $MASTER_ACCOUNT --initial-balance 100  #uncomment if account is just created

# deploy contract
near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/diesel_attack_nft_near.wasm --accountId $CONTRACT
near call $CONTRACT new --accountId $CONTRACT

# copy credentials for later deploy
sudo cp ~/.near-credentials/testnet/$CONTRACT.json /creds/$CONTRACT.json
sudo chmod 664 creds/$CONTRACT.json