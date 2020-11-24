const fs = require("fs");
const { Account } = require("chainx.js");
const path = require("path");

const migrationHeight = 23170000;

exports.writeFile = (filename, obj) => {
  dir = path.dirname(filename);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  fs.writeFile(filename, JSON.stringify(obj, null, 2), (err) => {
    if (err) {
      console.error(err);
      throw err;
    }

    console.log(`Saved data to file: ${filename}`);
  });
};

exports.writeAux = (json_filename, obj) => {
  this.writeFile("./res/aux/" + json_filename, obj);
};

// Sum the values of a JS Object
exports.sumValues = (obj) =>
  Object.values(obj).reduce((acc, value) => acc + value, 0);

exports.asAddress = (pubkey) => Account.encodeAddress(pubkey);
exports.asPubkey = (address) => Account.decodeAddress(address);

function readJSON(filepath) {
  return JSON.parse(fs.readFileSync(filepath));
}

function basePath(filename) {
  return path.join("..", "state_1.0", migrationHeight.toString(), filename);
}

exports.readJSON = readJSON;
exports.basePath = basePath;

exports.getAccounts = () => {
  return readJSON(basePath("assets.json"));
};

exports.getLegacyNominators = () => {
  return readJSON(basePath("vote-weight-accounts.json"));
};

exports.getLegacyValidatorWeights = () => {
  return readJSON(basePath("vote-weight-nodes.json"));
};

exports.getLegacyValidators = () => {
  return readJSON(basePath("intentions.json"));
};

exports.getLegacyAssetMiners = () => {
  return readJSON(basePath("deposit-weight-accounts.json"));
};

exports.getLegacyMiningAssets = () => {
  return readJSON(basePath("deposit-weight-nodes.json"));
};

exports.getRewardPotAccounts = () => {
  let rawIntentions = readJSON(basePath("intentions.json"));
  let pots = [];
  rawIntentions.forEach((ele) => {
    pots.push([
      this.asAddress(ele.jackpotAccount),
      this.asAddress(ele.account),
    ]);
  });
  return pots;
};
