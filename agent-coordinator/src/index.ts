import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname } from "node:path";
import type { CoordinatorConfig } from "./config.js";
import { LocalMxeClient } from "./mxe_client.js";
import type { OracleSource } from "./oracle_feed.js";
import { MockFacilitator } from "./payments/mock_facilitator.js";
import { X402Client } from "./payments/x402_client.js";
import { MockSettlementAdapter } from "./settlement.js";
import type { EpochResult } from "./types.js";

interface PersistedState {
  readonly epoch: number;
  readonly status: "pending" | "completed";
  readonly computationId: string;
  readonly result?: EpochResult;
}

async function loadState(path: string): Promise<PersistedState | undefined> {
  try {
    return JSON.parse(await readFile(path, "utf8")) as PersistedState;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
}

async function saveState(path: string, state: PersistedState): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(state, null, 2)}\n`, { encoding: "utf8", mode: 0o600 });
}

export class EpochCoordinator {
  readonly #mxe = new LocalMxeClient();
  readonly #settlement = new MockSettlementAdapter();
  readonly #payments: X402Client;

  constructor(
    private readonly config: CoordinatorConfig,
    private readonly oracle: OracleSource,
  ) {
    const facilitator = new MockFacilitator(config.paymentSecret);
    this.#payments = new X402Client(facilitator, config.paymentSecret);
  }

  async runEpoch(epoch: number): Promise<EpochResult> {
    if (!Number.isSafeInteger(epoch) || epoch <= 0) throw new Error("epoch must be positive");
    const previous = await loadState(this.config.stateFile);
    if (previous?.epoch === epoch && previous.status === "completed" && previous.result !== undefined) {
      return { ...previous.result, idempotentReplay: true };
    }
    if (previous?.status === "pending") {
      throw new Error(`manual recovery required for pending computation ${previous.computationId}`);
    }

    await this.#payments.payFor(`oracle:${epoch}`, this.config.rpcCostMicrousd);
    const input = await this.oracle.read(epoch);
    await this.#payments.payFor(`compute:${epoch}`, this.config.computeCostMicrousd);
    const receipt = await this.#mxe.submit(input);
    await saveState(this.config.stateFile, {
      epoch,
      status: "pending",
      computationId: receipt.id,
    });

    const positions = this.#settlement.settle(input.totalCapital, receipt.output);
    const result: EpochResult = {
      epoch,
      computationId: receipt.id,
      allocation: receipt.output,
      positions,
      paymentMicrousd: this.#payments.totalMicrousd(),
      idempotentReplay: false,
    };
    await saveState(this.config.stateFile, {
      epoch,
      status: "completed",
      computationId: receipt.id,
      result,
    });
    return result;
  }
}

