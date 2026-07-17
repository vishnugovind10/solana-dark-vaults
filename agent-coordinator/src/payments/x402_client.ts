import { createHmac } from "node:crypto";
import { MockFacilitator, type PaymentChallenge } from "./mock_facilitator.js";

export interface LedgerEntry {
  readonly requestId: string;
  readonly amountMicrousd: number;
  readonly protocol: "x402-mock";
}

export class X402Client {
  readonly #ledger: LedgerEntry[] = [];

  constructor(
    private readonly facilitator: MockFacilitator,
    private readonly signingSecret: string,
  ) {}

  async payFor(requestId: string, amountMicrousd: number): Promise<void> {
    const challenge: PaymentChallenge = this.facilitator.challenge(requestId, amountMicrousd);
    const signature = createHmac("sha256", this.signingSecret).update(challenge.nonce).digest("hex");
    if (!this.facilitator.verify(challenge, signature)) throw new Error("mock x402 payment rejected");
    this.#ledger.push({ requestId, amountMicrousd, protocol: "x402-mock" });
  }

  entries(): readonly LedgerEntry[] {
    return this.#ledger.map((entry) => ({ ...entry }));
  }

  totalMicrousd(): number {
    return this.#ledger.reduce((total, entry) => total + entry.amountMicrousd, 0);
  }
}

