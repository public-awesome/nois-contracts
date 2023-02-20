import test from "ava";

import { Bot } from "./bot";
import { DrandInstantiateMsg, OsmosisContractPaths, osmosisContracts, uploadContracts } from "./contracts";
import { setupOsmosisClient } from "./utils";

interface TestContext {
  osmosisCodeIds: Record<keyof OsmosisContractPaths, number>;
}

test.before(async (t) => {
  const osmoClient = await setupOsmosisClient();
  t.log("Upload contracts ...");
  const osmosisCodeIds = await uploadContracts(t, osmoClient, osmosisContracts);
  const context: TestContext = { osmosisCodeIds };
  t.context = context;
  t.pass();
});

test.serial("drand: bot can submit", async (t) => {
  // Instantiate Drand on osmosis
  const osmoClient = await setupOsmosisClient();

  const msg: DrandInstantiateMsg = {
    manager: osmoClient.senderAddress,
    min_round: 2183660,
    incentive_point_price: "0",
    incentive_denom: "unois",
  };
  const { contractAddress: drandAddress } = await osmoClient.sign.instantiate(
    osmoClient.senderAddress,
    (t.context as TestContext).osmosisCodeIds.drand,
    msg,
    "Drand instance",
    "auto"
  );
  t.log(`Instantiated drand at ${drandAddress} with msg ${JSON.stringify(msg)}`);
  t.truthy(drandAddress);

  const before = await osmoClient.sign.queryContractSmart(drandAddress, {
    beacon: { round: 2183666 },
  });
  t.deepEqual(before, { beacon: null });

  const bot = await Bot.connect(drandAddress);

  // Register
  await bot.register("joe");

  // Submit
  const addRundRes = await bot.submitRound(2183666);
  t.log(`Gas used: ${addRundRes.gasUsed}/${addRundRes.gasWanted}`);

  const after = await osmoClient.sign.queryContractSmart(drandAddress, {
    beacon: { round: 2183666 },
  });
  t.regex(after.beacon.published, /^1660941000000000000$/);
  t.regex(after.beacon.verified, /^1[0-9]{18}$/);
  t.is(after.beacon.randomness, "768bd188a948f1f2959d15c657f159dd34bdf741b7d4b17a29b877eb36c04dcf");
});