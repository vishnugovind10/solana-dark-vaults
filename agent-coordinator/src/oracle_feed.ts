import type { AllocationInput } from "./types.js";

export interface OracleSource {
  read(epoch: number): Promise<AllocationInput>;
}

/** Deterministic mock. It is not a trusted or live market-data source. */
export class MockOracleSource implements OracleSource {
  async read(epoch: number): Promise<AllocationInput> {
    if (!Number.isSafeInteger(epoch) || epoch <= 0) throw new Error("epoch must be positive");
    return {
      totalCapital: 1_000_000,
      pools: [
        { yieldBps: 780 + epoch, riskScore: 220, capBps: 4_500 },
        { yieldBps: 640 + epoch, riskScore: 280, capBps: 3_500 },
        { yieldBps: 430 + epoch, riskScore: 410, capBps: 4_000 },
      ],
    };
  }
}

