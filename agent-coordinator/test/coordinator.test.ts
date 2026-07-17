import assert from "node:assert/strict";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { computeReferenceAllocation } from "../src/allocation.js";
import { EpochCoordinator } from "../src/index.js";
import { MockOracleSource } from "../src/oracle_feed.js";
import { MockFacilitator } from "../src/payments/mock_facilitator.js";
import { X402Client } from "../src/payments/x402_client.js";

test("allocator enforces cap and sum invariants", () => {
  const output = computeReferenceAllocation({
    totalCapital: 1_000,
    pools: [
      { yieldBps: 900, riskScore: 100, capBps: 4_000 },
      { yieldBps: 600, riskScore: 200, capBps: 4_000 },
      { yieldBps: 300, riskScore: 300, capBps: 4_000 },
    ],
  });
  assert.deepEqual(output.weightsBps, [4_000, 4_000, 2_000]);
  assert.throws(() =>
    computeReferenceAllocation({
      totalCapital: 0,
      pools: [
        { yieldBps: 1, riskScore: 1, capBps: 4_000 },
        { yieldBps: 2, riskScore: 1, capBps: 4_000 },
        { yieldBps: 3, riskScore: 1, capBps: 4_000 },
      ],
    }),
  );
});

test("mock x402 flow records a non-zero metered payment", async () => {
  const facilitator = new MockFacilitator("secret");
  const client = new X402Client(facilitator, "secret");
  await client.payFor("rpc:1", 25);
  assert.equal(client.totalMicrousd(), 25);
  assert.equal(client.entries()[0]?.protocol, "x402-mock");
});

test("full epoch settles positions and replays idempotently", async () => {
  const directory = await mkdtemp(join(tmpdir(), "dark-vault-test-"));
  try {
    const coordinator = new EpochCoordinator(
      {
        stateFile: join(directory, "pending.json"),
        paymentSecret: "test-secret",
        rpcCostMicrousd: 20,
        computeCostMicrousd: 80,
      },
      new MockOracleSource(),
    );
    const result = await coordinator.runEpoch(1);
    assert.equal(result.positions.reduce((sum, value) => sum + value, 0), 1_000_000);
    assert.equal(result.paymentMicrousd, 100);
    assert.equal(result.idempotentReplay, false);
    const replay = await coordinator.runEpoch(1);
    assert.equal(replay.computationId, result.computationId);
    assert.equal(replay.idempotentReplay, true);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

