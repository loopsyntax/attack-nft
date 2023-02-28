# Diesel Attack NFT Game Backend API Server

Welcome to the cyberpunk post-apocalyptic world of Diesel Attack

![Diesel Attack NFT Game Gameplay](/docs/img/gameplay.jpg)


>To earn and collect NFTs during the game you need a [NEAR Wallet Account](https://wallet.testnet.near.org) (on the testnet). We never ask you for keys.


<details open="open">
<summary>Table of Contents</summary>

- [About](#about)
- [API Usage](#api-usage)
    - [API methods](#api-methods)
    - [Security](#security)
- [How To Build](#how-to-build)
    - [Prerequisites](#prerequisites)
    - [Build the contract](#build-the-contract)
    - [Deploy to the Dev account](#deploy-to-the-dev-account)
    - [Deploy to the Testnet account](#deploy-to-the-testnet-account)
    - [Tests](#tests)



</details>

## About

This is the source of the backend API server of the RPG 2D-sidescroller game Diesel Attack. It implements smart contract on the NEAR blockchain to mint NFTs for players during the game on the fly. When a game client app sends a request to the server using API, a player gets a new weapon or ship as NFT based on the weighted random algorithm. Created on NEAR RUST SDK. NFT arts are hosted on IPFS. The server itself is running on Fastify (Node.js framework).

## API Usage



### API methods

#### Status

`GET /api/` returns greetings message (server status check)

Production server endpoint:

https://v1.dieselattack.com/api/

#### Mint NFT

`GET /api/mint-nft?nearid=<username.testnet>` mints NFT for player and returns a code of the collectible to a game client app. Where `<username.testnet>` is the player's NEAR account on the testnet.

Production server endpoint:

https://v1.dieselattack.com/api/mint-nft?nearid=username.testnet

>You can check it by yourself and mint NFT to your NEAR testnet account. No authentication needed. Just put your Testnet account address instead of `username`, browse it and check your NEAR wallet Collectibles section. 

Server's NFT contract address: `nfts.dieselattack.testnet` ([See in Explorer](https://explorer.testnet.near.org/accounts/nfts.dieselattack.testnet))

### Security

:warning: For the duration of the NEAR MetaBUILD III Hackathon we left the possibility to check and mint NFT via API link for everyone intentionally.
But minting NFT is a non-free operation and there is a vulnerability. Users can abuse the contract, and it may run out of Gas.
To prevent this in production on the mainnet you SHOULD use the secret `appkey` token, where `secret_token` is something like `QmFzZTY0IGVuY29kaW5nIG9`:

`GET /api/mint-nft?nearid=<username.testnet>&appkey=<secret_token>`

## How To Build

### Prerequisites

References to the original documentation:

- [NEAR Wallet Account](https://wiki.near.org/getting-started/creating-a-near-wallet)
- [Rust](https://www.rust-lang.org/tools/install) + [Wasm32](https://rustwasm.github.io/docs.html)
- [Node.js](https://nodejs.org/en/download/package-manager/)
- [Near-cli](https://docs.near.org/tools/near-cli#setup)
- [Yarn](https://yarnpkg.com/getting-started/install)

<details>
<summary>Installing the prerequisites</summary>

1. Sign up to the NEAR Testnet Wallet and follow the instructions:
    [https://wallet.testnet.near.org](https://wallet.testnet.near.org)


2. Install Rust with [Rustup](https://rust-lang.github.io/rustup/) tool (recommended):
    ```sh
    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
    ```

3. Install Wasm32:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

4. On Ubuntu you may need to install Build-essential:
    ```sh
    sudo apt install build-essential
    ```

5. Install [NVM](https://github.com/nvm-sh/nvm) (Node.js version manager):
    ```sh
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
    ```

6. Add these lines to your `~/.bashrc`, `~/.profile`, or `~/.zshrc` file to have it automatically sourced (you may need to restart the session):
    ```
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
    ```

7. Install the latest LTS version of Node.js:
    ```sh
    nvm install --lts
    ```

8. Install NPM package manager:
    ```sh
    npm install -g npm@latest
    ```

9. Install Near-cli globally:
    ```sh
    npm install -g near-cli
    ```

10. Install [Corepack](https://nodejs.org/dist/latest/docs/api/corepack.html) (Yarn version manager):

    *Node.js >=16.10*:
    ```sh
    corepack enable
    ```

    *Node.js <16.10*:
    ```sh
    npm i -g corepack
    ```

11. Activate the latest global Yarn version:

    *Node.js ^16.17 or >=18.6*:
    ```sh
    corepack prepare yarn@stable --activate
    ```

    *Node.js <16.17 or <18.6*:

    Take a look at the [latest Yarn release](https://github.com/yarnpkg/berry/releases/latest), note the version number, and run:
    ```sh
    corepack prepare yarn@<version> --activate
    ```

</details>


### Build the contract

1. Install all the dependencies:
    ```sh
    yarn
    ```

2. Build the smart-contract into WebAssembly:
    ```sh
    yarn build
    ```


### Deploy to the Dev account

1. Deploy the contract to the dev account:
    ```sh
    yarn deploy:dev
    ```

2. Run the local server:
    ```sh
    yarn start:dev
    ```


### Deploy to the Testnet account 

1. Login to the testnet account:
    ```sh
    near login
    ```

2. Set your values for `CONTRACT` and `MASTER_ACCOUNT` in the deploy script `./scripts/deploy.sh`.


3. Make sure you have the executable permissions on the deploy script:
    ```sh
    sudo chmod +x ./scripts/deploy.sh
    ```

4. Deploy the contract to the testnet account:
    ```sh
    yarn deploy
    ```

5. Set your value for `CONTRACT_NAME` in the account config file `./config/testnet-account.env`


6. Run the local server:
    ```sh
    yarn start
    ```

### Tests

- Run the testnet account test:
    ```sh
    yarn test
    ```

- Run the dev account test:
    ```sh
    yarn test:dev
    ```

- Run the local server test:
    ```sh
    yarn test:server
    ```
