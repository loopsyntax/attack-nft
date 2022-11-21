mod constants;

// use std::collections::HashMap;
use near_contract_standards::non_fungible_token::{Token, TokenId, NonFungibleToken};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC
  };

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseOrValue
};
// use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::collections::{LookupMap, UnorderedSet, LazyOption};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::Base64VecU8;
// use near_sdk::serde_json::{json, Result};
use near_sdk::serde_json::json;

pub use constants::{BASE_URI, DATA_IMAGE_SVG_NEAR_ICON, ONE_NEAR, ONE_YOCTO, SINGLE_CALL_GAS};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval
}


// NFT data
pub struct NftData {
    code: String,
    name: String,
    description: String,
    media_url: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with provided metadata
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let owner_id = env::current_account_id();

        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Diesel Attack NFT Collection".to_string(),
            symbol: "DIESEL".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();        

        Self {
            owner_id: owner_id.clone().into(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }                
    }

    // Mint NFT and send it to `username` account
    #[payable]
    pub fn nft_mint(&mut self, username: String) -> TokenId {
        let rand: u8 = *env::random_seed().get(0).unwrap();

        let nft_data = match rand {
                    0..=36 => NftData {
                        code: String::from("gun-1"),
                        name: String::from("Machine Hunt"),
                        description: String::from("Long-barreled firearm designed for accurate shooting, with a barrel that has a helical pattern of grooves (rifling) cut into the bore wall."),
                        media_url: String::from("https://bafkreibbwmh6t4s46hi5vw7qxcglkreadbqnf5ydqqxyuaqqwewbnwdckm.ipfs.nftstorage.link"),
                    },
                    37..=73 => NftData {
                        code: String::from("gun-2"),
                        name: String::from("Gauss Rifle"),
                        description: String::from("Is a type of projectile accelerator consisting of one or more coils used as electromagnets in the configuration of a linear motor that accelerates a ferromagnetic or conducting projectile to high velocity."),
                        media_url: String::from("https://bafkreicqk2tfkao6ckwsqsfc6az3buwqkzhyoha46fyur74ygiu2yklv4u.ipfs.nftstorage.link"),
                    },
                    74..=110 => NftData {
                        code: String::from("gun-3"),
                        name: String::from("Rocket Launcher"),
                        description: String::from("A launcher consisting of a tube or cluster of tubes (as a three-tube unit placed on the underside of diesel ship) for firing rocket shells. Trust me, you dont wanna feel its impact."),
                        media_url: String::from("https://bafkreidjqk52hl6o6upzas4cd6kd7exx4wgghdc4aqjwdev73vjbbbfe7m.ipfs.nftstorage.link"),
                    },
                    111..=147 => NftData {
                        code: String::from("gun-4"),
                        name: String::from("Destroyer of The Worlds"),
                        description: String::from("Ever heard of a Death Star? Meet its younger brother. When unstoppable force meets immovable object... force wins, if its Destroyer of The Worlds."),
                        media_url: String::from("https://bafkreiekbwphahf25zwlgl3bbgxlwcqlq5lmqyu5p2nopalm5la4km4gl4.ipfs.nftstorage.link"),
                    },
                    148..=184 => NftData {
                        code: String::from("gun-5"),
                        name: String::from("ShotGun"),
                        description: String::from("A short-barreled firearm designed to shoot a straight-walled cartridges, which usually discharges numerous small pellet-like spherical sub-projectiles."),
                        media_url: String::from("https://bafkreibtzmq624yrdc5s3ur67rtqt5p5vcmvbt5lllekyuy2kgsnnfqoom.ipfs.nftstorage.link"),
                    },
                    185..=214 => NftData {
                        code: String::from("ship-1"),
                        name: String::from("Nautilus"),
                        description: String::from("This Ship utilizes marine engine that uses the heat of compression to ignite and power the ignition of diesel fuel. Legend told, it was used by Nemo itself."),
                        media_url: String::from("https://bafybeid6uipm6x2flcu4yqup7bg2kc5bfz2twsz5jzm5owdbnw6hui3lza.ipfs.nftstorage.link"),
                    },
                    _ => NftData {
                        code: String::from("ship-2"),
                        name: String::from("Ragnarok"),
                        description: String::from("When its time for a final battle, you'd better use Ragnarok. Maneuverable, fast and deadly hitting. Your foes have no chances."),
                        media_url: String::from("https://bafybeibn6by4wxdqujenwqedq72y2suiypic7z2pgufidazpd34ny5mjsy.ipfs.nftstorage.link"),
                    }
        };

        // Generate token_id
        let timestamp: u64 = env::block_timestamp();
//        let rand: u8 = *env::random_seed().get(1).unwrap();
        let token_id: String = format!("{}:{}", timestamp, rand);
        log!("code: {}", &nft_data.code.clone());
        log!("name: {}", &nft_data.name.clone());
        let contract_id = env::current_account_id();
        let root_id = AccountId::try_from(contract_id).unwrap();
        let media_hash = Base64VecU8(env::sha256(&nft_data.media_url.as_bytes()));
        log!("media url: {}", &nft_data.media_url.clone());

        // Default to common token
        let token_metadata = TokenMetadata {
            title: Some(nft_data.name),
            description: Some(nft_data.description),
            media: Some(nft_data.media_url),
            media_hash: Some(media_hash),
            copies: Some(1u64),
            issued_at: Some(timestamp.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        // Mint NFT   
        self.nft(token_id.clone(), root_id.clone(), token_metadata.clone());

        // Transfer NFT to new owner
        let receiver_id = AccountId::try_from(username).unwrap();
        log!("sender id: {}", root_id.clone());
        log!("receiver id: {}", receiver_id.clone());
        log!("token id: {}", token_id.clone());
        env::promise_create(
            root_id,
            "nft_transfer",
            json!({
                "token_id": token_id.clone(),
                "receiver_id": receiver_id,
            })
            .to_string()
            .as_bytes(),
            ONE_YOCTO,
            SINGLE_CALL_GAS,
        );
        log!("Success! NFT has been minted for {}! Token code = {}", receiver_id.clone(), &nft_data.code.clone());


        nft_data.code
    }

    // Mint a new token with ID=token_id belonging to receiver_id.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. self.tokens.mint will also require it to be Some, since
    /// StorageKey::TokenMetadata was provided at initialization.
    ///
    /// self.tokens.mint will enforce predecessor_account_id to equal the owner_id given in
    /// initialization call to new.
    #[payable]
    fn nft(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }
}

// Implement NFT standard
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
      self.metadata.get().unwrap()
  }
}