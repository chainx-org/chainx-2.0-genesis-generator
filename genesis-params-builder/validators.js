const {
  writeAux,
  writeFile,
  asAddress,
  getLegacyValidators,
  getLegacyValidatorWeights,
} = require("./utils");

// 1 PCX
const minimumActiveRewardPotBalance = 100000000;

exports.filterActiveIntentions = () => {
  let legacyValidators = getLegacyValidators();

  console.log(`Legacy intentions count: ${legacyValidators.length}`);

  var dyingValidators = [];
  var deadValidators = [];
  var activeValidators = [];

  var autoClaimed = {};

  var autoClaimedPots = [];

  var totalDyingValidatorsPotBalance = 0;

  legacyValidators.forEach((ele) => {
    let { account, jackpot, jackpotAccount, selfVote, totalNomination } = ele;

    ele.account = asAddress(account);

    if (jackpot === 0) {
      // No nominators, can be swept safely.
      if (totalNomination === 0) {
        deadValidators.push(ele);
        // Only the validator self-bond, can be distribute to the validator directly.
      } else if (selfVote === totalNomination) {
        deadValidators.push(ele);
      } else {
        deadValidators.push(ele);
      }
    } else if (jackpot < minimumActiveRewardPotBalance) {
      // Someone might forget the reward
      if (totalNomination === 0) {
        dyingValidators.push(ele);
        // Only the validator self-bond, can be distribute to the validator directly.
      } else if (selfVote === totalNomination) {
        autoClaimed[asAddress(account)] = jackpot;
        autoClaimedPots.push(asAddress(jackpotAccount));
        dyingValidators.push(ele);
      } else {
        dyingValidators.push(ele);
      }
      totalDyingValidatorsPotBalance += jackpot;
    } else {
      // The reward pot has 1+ PCX balance
      activeValidators.push(ele);
    }
  });

  console.log(`Dead intentions count: ${deadValidators.length}`);
  writeAux("intentions_dead.json", deadValidators);

  console.log(
    `Dying intentions count: ${dyingValidators.length}, total dying intention reward pot balance: ${totalDyingValidatorsPotBalance}`
  );
  writeAux("intentions_dying.json", dyingValidators);

  console.log(`active intentions count: ${activeValidators.length}`);
  writeAux("intentions_active.json", activeValidators);

  const legacyValidatorWeights = getLegacyValidatorWeights();
  let calculatedValidatorWeightMap = {};
  legacyValidatorWeights.forEach((ele) => {
    calculatedValidatorWeightMap[asAddress(ele.account)] = ele.weight;
  });

  var genesisValidators = [];
  activeValidators.forEach((ele) => {
    genesisValidators.push({
      who: ele.account,
      referral_id: ele.name,
      self_bonded: ele.selfVote,
      total_nomination: ele.totalNomination,
      total_weight: calculatedValidatorWeightMap[ele.account],
    });
  });

  writeAux("genesis_validators.json", genesisValidators);
  writeAux("genesis_balances_auto_claimed.json", autoClaimed);

  return [genesisValidators, autoClaimed, autoClaimedPots];
};
