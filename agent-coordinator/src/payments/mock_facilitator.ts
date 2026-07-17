import { createHmac, timingSafeEqual } from "node:crypto";

export interface PaymentChallenge {
  readonly requestId: string;
  readonly amountMicrousd: number;
  readonly nonce: string;
}

/** In-process x402 facilitator mock. It moves no funds and uses a test-only shared secret. */
export class MockFacilitator {
  constructor(private readonly secret: string) {}

  challenge(requestId: string, amountMicrousd: number): PaymentChallenge {
    if (amountMicrousd <= 0) throw new Error("payment amount must be positive");
    return { requestId, amountMicrousd, nonce: `${requestId}:${amountMicrousd}` };
  }

  expectedSignature(challenge: PaymentChallenge): string {
    return createHmac("sha256", this.secret).update(challenge.nonce).digest("hex");
  }

  verify(challenge: PaymentChallenge, signature: string): boolean {
    const expected = Buffer.from(this.expectedSignature(challenge), "hex");
    const supplied = Buffer.from(signature, "hex");
    return expected.length === supplied.length && timingSafeEqual(expected, supplied);
  }
}

