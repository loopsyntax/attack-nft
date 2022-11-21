const sh = require("shelljs");

sh.exec(`curl "http://localhost:4000/api/"`);
sh.exec(`curl "http://localhost:4000/api/mint-nft?nearid=dieselattack.testnet"`);
