export interface CoordinatorConfig {
  readonly stateFile: string;
  readonly paymentSecret: string;
  readonly rpcCostMicrousd: number;
  readonly computeCostMicrousd: number;
}

function positiveInteger(value: string | undefined, fallback: number, name: string): number {
  const parsed = value === undefined ? fallback : Number(value);
  if (!Number.isSafeInteger(parsed) || parsed <= 0) throw new Error(`${name} must be positive`);
  return parsed;
}

export function configFromEnv(): CoordinatorConfig {
  return {
    stateFile: process.env.DARK_VAULT_STATE_FILE ?? ".state/pending.json",
    paymentSecret: process.env.DARK_VAULT_PAYMENT_SECRET ?? "local-demo-secret",
    rpcCostMicrousd: positiveInteger(process.env.DARK_VAULT_RPC_COST, 25, "DARK_VAULT_RPC_COST"),
    computeCostMicrousd: positiveInteger(
      process.env.DARK_VAULT_COMPUTE_COST,
      75,
      "DARK_VAULT_COMPUTE_COST",
    ),
  };
}

