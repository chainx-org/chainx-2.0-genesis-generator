const {
  writeAux,
  asAddress,
  getLegacyAssetMiners,
  getLegacyMiningAssets,
} = require("./utils");

exports.processAssetMining = () => {
  const legacyMiners = getLegacyAssetMiners();

  let newMiners = [];

  let xbtcWeight = BigInt(0);

  // Filter out all the X-BTC miners with positive mining weight.
  legacyMiners.forEach((entry) => {
    const who = asAddress(entry.account);
    const weight = entry.xbtc.weight;
    if (weight !== "0") {
      xbtcWeight += BigInt(weight);
      newMiners.push({
        who,
        weight,
      });
    }
  });

  writeAux("genesis_xbtc_miners.json", newMiners);

  const miningAssets = getLegacyMiningAssets();
  writeAux("genesis_xbtc_info.json", miningAssets.xbtc);

  // NOTE: Now fix the inequality of xbtc total weight forcibly.
  //
  // ChainX v1.0.3 had fixed the deposit weight issue, but did not fix the history xbtc weight data.
  // Since we are doing the 2.0 migration, fix it now.
  miningAssets.xbtc.weight = xbtcWeight.toString();

  return {
    xbtc_miners: newMiners,
    xbtc_info: miningAssets.xbtc,
  };
};
