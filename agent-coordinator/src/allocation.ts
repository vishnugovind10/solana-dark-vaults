import { BPS_DENOMINATOR, type AllocationInput, type AllocationOutput } from "./types.js";

export function computeReferenceAllocation(input: AllocationInput): AllocationOutput {
  if (!Number.isSafeInteger(input.totalCapital) || input.totalCapital <= 0) {
    throw new Error("totalCapital must be a positive safe integer");
  }

  let capacity = 0;
  const scored = input.pools.map((pool, index) => {
    if (!Number.isInteger(pool.riskScore) || pool.riskScore <= 0) {
      throw new Error(`pool ${index} riskScore must be positive`);
    }
    if (!Number.isInteger(pool.capBps) || pool.capBps < 0 || pool.capBps > BPS_DENOMINATOR) {
      throw new Error(`pool ${index} capBps is outside [0, 10000]`);
    }
    capacity += pool.capBps;
    return { index, score: Math.floor((pool.yieldBps * BPS_DENOMINATOR) / pool.riskScore) };
  });
  if (capacity < BPS_DENOMINATOR) {
    throw new Error("pool caps cannot allocate 10000 bps");
  }

  scored.sort((left, right) => right.score - left.score || left.index - right.index);
  const weights = [0, 0, 0];
  let remaining = BPS_DENOMINATOR;
  for (const { index } of scored) {
    const pool = input.pools[index];
    if (pool === undefined) throw new Error("invalid pool index");
    const grant = Math.min(pool.capBps, remaining);
    weights[index] = grant;
    remaining -= grant;
  }
  if (remaining !== 0) throw new Error("allocation invariant failed");
  return { weightsBps: weights as [number, number, number] };
}

