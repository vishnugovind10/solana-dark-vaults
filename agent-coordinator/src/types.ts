export const BPS_DENOMINATOR = 10_000;
export const POOL_COUNT = 3;

export interface PoolParams {
  readonly yieldBps: number;
  readonly riskScore: number;
  readonly capBps: number;
}

export interface AllocationInput {
  readonly totalCapital: number;
  readonly pools: readonly [PoolParams, PoolParams, PoolParams];
}

export interface AllocationOutput {
  readonly weightsBps: readonly [number, number, number];
}

export interface EpochResult {
  readonly epoch: number;
  readonly computationId: string;
  readonly allocation: AllocationOutput;
  readonly positions: readonly [number, number, number];
  readonly paymentMicrousd: number;
  readonly idempotentReplay: boolean;
}

