const { writeAux, asAddress, getLegacyNominators } = require("./utils");

exports.processNominatorsVoteInfo = () => {
  const legacyNominators = getLegacyNominators();

  var newNominators = [];
  var totalRevocationsOf = {};

  legacyNominators.forEach((entry) => {
    // Save the state for analyzing the 1.0 state only.
    entry.nodes.forEach((vote) => {
      const totalRevocation = vote.revocations
        .map((r) => r.value)
        .reduce((a, b) => a + b, 0);
      if (totalRevocation > 0) {
        totalRevocationsOf[vote.account] = totalRevocation;
      }
    });

    var nominations = [];
    entry.nodes.forEach((vote) => {
      // Only the record with zero nomination and zero weight can be safely ignored.
      if (vote.nomination !== 0 || vote.weight !== "0") {
        nominations.push({
          nominee: asAddress(vote.account),
          nomination: vote.nomination,
          weight: vote.weight,
        });
      }
    });

    if (nominations.length > 0) {
      newNominators.push({
        nominator: asAddress(entry.account),
        nominations,
      });
    }
  });

  writeAux("genesis_nominators.json", newNominators);
  writeAux("genesis_revocations.json", totalRevocationsOf);

  return newNominators;
};
