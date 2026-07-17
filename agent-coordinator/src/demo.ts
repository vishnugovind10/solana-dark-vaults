import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { EpochCoordinator } from "./index.js";
import { MockOracleSource } from "./oracle_feed.js";

const directory = await mkdtemp(join(tmpdir(), "dark-vault-demo-"));
try {
  const coordinator = new EpochCoordinator(
    {
      stateFile: join(directory, "pending.json"),
      paymentSecret: "local-demo-secret",
      rpcCostMicrousd: 25,
      computeCostMicrousd: 75,
    },
    new MockOracleSource(),
  );
  const result = await coordinator.runEpoch(1);
  console.log("solana-dark-vaults local reference epoch");
  console.log("1. Mock deposit capital: 1,000,000 units");
  console.log(`2. X25519-encrypted local submission: ${result.computationId}`);
  console.log(`3. Allocation output (bps): ${result.allocation.weightsBps.join(" / ")}`);
  console.log(`4. Mock settled positions: ${result.positions.join(" / ")}`);
  console.log(`5. x402 mock cost ledger: ${result.paymentMicrousd} micro-USD`);
  console.log("Scope: local simulator; no Solana transaction or Arcium MXE execution occurred.");
} finally {
  await rm(directory, { recursive: true, force: true });
}

