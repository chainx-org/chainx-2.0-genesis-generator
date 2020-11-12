const { filterActiveIntentions } = require("./validators");
const { processNominatorsVoteInfo } = require("./nominators");
const { processAccounts } = require("./balances");
const { processAssetMining } = require("./asset_mining");
const { writeAux, writeFile, sumValues, asPubkey } = require("./utils");

const xminingasset = processAssetMining();
const nominators = processNominatorsVoteInfo();

const [validators, autoClaimed, autoClaimedPots] = filterActiveIntentions();
console.log(`Total autoClaimed: ${sumValues(autoClaimed)}`);

const [newAccounts, wellknownAccounts, btcAccounts] = processAccounts(
  autoClaimedPots
);

let finalFreeBalances = [];

newAccounts.forEach((ele) => {
  if (ele.who in autoClaimed) {
    finalFreeBalances.push({
      who: ele.who,
      free: ele.free + autoClaimed[ele.who],
    });
  } else {
    finalFreeBalances.push(ele);
  }
});

// Duplicate the free balances info using public key
let finalFreeBalancesInPubkey = [];
newAccounts.forEach((ele) => {
  finalFreeBalancesInPubkey.push({ who: asPubkey(ele.who), free: ele.free });
});

// Write the aux info of the exported state.
writeAux("genesis_xstaking.json", { validators, nominators });
writeAux("genesis_balances.json", {
  free_balances: finalFreeBalances,
  wellknown_accounts: wellknownAccounts,
});
writeAux("genesis_xminingasset.json", xminingasset);
writeAux("genesis_balances_in_pubkey.json", finalFreeBalancesInPubkey);

////////////////////////////////////////////////////////////
//   Write the final genesis parameters for ChainX 2.0.
////////////////////////////////////////////////////////////
writeFile("./res/2.0/genesis_builder_params.json", {
  balances: {
    free_balances: finalFreeBalances,
    wellknown_accounts: wellknownAccounts,
  },
  xassets: btcAccounts,
  xstaking: {
    validators,
    nominators,
  },
  xmining_asset: xminingasset,
});
