const {
  writeAux,
  sumValues,
  asAddress,
  getAccounts,
  getRewardPotAccounts,
} = require("./utils");

// 5RzDbX1ZiQZuAuxMGBn6WzvZiJnGEoainSWj9VWe27K6EcLz
const legacyCouncilAccount =
  "0x67df26a755e0c31ac81e2ed530d147d7f2b9a3f5a570619048c562b1ed00dfdd";

// 5RqxsaJpkqP8CHyiVUrLWL4HDaHNX3ytte7fo8sAD8Jnh8sy
const legacyTeamAccount =
  "0x6193a00c655f836f9d8a62ed407096381f02f8272ea3ea0df0fd66c08c53af81";

var wellknownAccounts = [];

exports.processAccounts = (autoClaimedPots) => {
  let rawAccounts = getAccounts();

  var newAccounts = [];
  var newBtcAccounts = [];

  var zeroCount = 0;
  var zeroBtcCount = 0;

  const wellknownAccounts = {
    legacy_council: asAddress(legacyCouncilAccount),
    legacy_team: asAddress(legacyTeamAccount),
    legacy_pots: getRewardPotAccounts(),
  };

  var newPubkeys = [];

  rawAccounts.forEach((entry) => {
    const account = entry.account;
    const who = asAddress(account);

    // Collect all the accounts that have non-zero PCX assets.
    const pcxAsset = entry.assets.filter((asset) => asset.name === "PCX");
    if (pcxAsset.length > 0) {
      const sum = sumValues(pcxAsset[0].details);
      if (sum > 0) {
        if (!autoClaimedPots.includes(who)) {
          newAccounts.push({ who, free: sum });
          newPubkeys.push({ account, free: sum });
        }
      } else {
        zeroCount += 1;
      }
    }

    // Collect all the accounts that have non-zero X-BTC assets.
    const btcAsset = entry.assets.filter((asset) => asset.name === "BTC");
    if (btcAsset.length > 0) {
      const sum = sumValues(btcAsset[0].details);
      if (sum > 0) {
        newBtcAccounts.push({ who, free: sum });
      } else {
        zeroBtcCount += 1;
      }
    }
  });

  console.log(`Total accounts: ${rawAccounts.length}`);
  console.log(`Zero PCX accounts: ${zeroCount}`);
  console.log(`Total positive BTC Accounts: ${newBtcAccounts.length}`);
  console.log(`Zero BTC accounts: ${zeroBtcCount}`);

  writeAux("genesis_balances_pure.json", newAccounts);
  writeAux("genesis_xassets.json", newBtcAccounts);
  writeAux("genesis_wellknown_accounts.json", wellknownAccounts);
  writeAux("genesis_balances_pubkey_pure.json", newPubkeys);

  return [newAccounts, wellknownAccounts, newBtcAccounts];
};
