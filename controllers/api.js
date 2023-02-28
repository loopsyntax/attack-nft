const fastify = require('fastify')({
    logger: {
        level: 'info',
        file: './api.log'
    }
});

const nearAPI = require("near-api-js");
const getConfig = require("../config/near");

const nearConfig = getConfig(process.env.APP_ENV || "development");
const { nodeUrl, networkId, contractName } = nearConfig;
const contractMethods = {
    changeMethods: ["nft_mint"],
//    viewMethods: ["get_tokens"],
};

const creds = `../creds/${contractName}`;
let msg = `Loading Credentials: ${creds}`;
console.log(msg);
fastify.log.info(msg);

const credentials = require(creds);

const {
    keyStores: { InMemoryKeyStore },
    Near,
    Account,
    Contract,
    KeyPair,
    utils: {
        format: { parseNearAmount },
    },
} = nearAPI;

const keyStore = new InMemoryKeyStore();
keyStore.setKey(
    networkId,
    contractName,
    KeyPair.fromString(credentials.private_key)
);

const near = new Near({
    networkId,
    nodeUrl,
    deps: { keyStore },
});

const { connection } = near;
const contractAccount = new Account(connection, contractName);

contractAccount.addAccessKey = (publicKey) =>
    contractAccount.addKey(
        publicKey,
        contractName,
        contractMethods.changeMethods,
        parseNearAmount("0.1")
    );

const contract = new Contract(contractAccount, contractName, contractMethods);

const Api = async (request, reply) => {
    return 'Diesel Attack NFT Game Backend API Server!';
}

const ApiMintNft = async (request, reply) => {
    let result;
    const username = request.query.nearid;
    const gas_cost = 300000000000000;
    const minting_cost = "100000000000000000000000";
    msg = `Minting new NFT for ${username}`;
    console.log(msg);
    // fastify.log.info(msg);

    try {
        result = await contract.nft_mint({
            args: { username },
            gas: gas_cost,
            amount: minting_cost,
        })
        // fastify.log.info(`Success! NFT has been minted for ${username}! Token code = ${result}`);
    } catch (err) {
        console.log(err);
        fastify.log.error(err);
        return 'Error. For more info see log!';
    }

    return result;
}

// const ApiGetInfo = async (request, reply) => {
//     let result;
//     const username = request.query.nearid;
//     msg = `Searching all NFT tokens for ${username}`;
//     console.log(msg);
//     fastify.log.info(msg);
//
//     try {
//         result = await contract.get_tokens({
//             args: { username }
//         })
//         fastify.log.info(`All minted tokens for ${username}: ${result}`);
//     } catch (err) {
//         console.log(err);
//         fastify.log.error(err);
//         return 'Error. For more info see log!';
//     }
//
//     return result;
// }

module.exports = {
    Api,
    ApiMintNft,
//    ApiGetInfo
}