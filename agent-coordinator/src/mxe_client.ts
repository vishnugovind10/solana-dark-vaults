import {
  createCipheriv,
  createDecipheriv,
  createHash,
  diffieHellman,
  generateKeyPairSync,
  randomBytes,
  randomUUID,
  type KeyObject,
} from "node:crypto";
import { computeReferenceAllocation } from "./allocation.js";
import type { AllocationInput, AllocationOutput } from "./types.js";

interface Envelope {
  readonly iv: Buffer;
  readonly ciphertext: Buffer;
  readonly authTag: Buffer;
}

export interface ComputationReceipt {
  readonly id: string;
  readonly output: AllocationOutput;
  readonly encryptedBytes: number;
}

function deriveKey(privateKey: KeyObject, publicKey: KeyObject): Buffer {
  return createHash("sha256").update(diffieHellman({ privateKey, publicKey })).digest();
}

function encryptJson(value: unknown, key: Buffer): Envelope {
  const iv = randomBytes(12);
  const cipher = createCipheriv("aes-256-gcm", key, iv);
  const ciphertext = Buffer.concat([cipher.update(JSON.stringify(value), "utf8"), cipher.final()]);
  return { iv, ciphertext, authTag: cipher.getAuthTag() };
}

function decryptJson<T>(envelope: Envelope, key: Buffer): T {
  const decipher = createDecipheriv("aes-256-gcm", key, envelope.iv);
  decipher.setAuthTag(envelope.authTag);
  return JSON.parse(Buffer.concat([decipher.update(envelope.ciphertext), decipher.final()]).toString("utf8")) as T;
}

/**
 * Local cryptographic simulator for the MXE client boundary.
 *
 * It performs real X25519 key agreement and authenticated encryption, but computation happens in
 * this process through the cleartext reference. It must not be described as an Arcium execution.
 */
export class LocalMxeClient {
  readonly #client = generateKeyPairSync("x25519");
  readonly #mxe = generateKeyPairSync("x25519");

  async submit(input: AllocationInput): Promise<ComputationReceipt> {
    const clientKey = deriveKey(this.#client.privateKey, this.#mxe.publicKey);
    const mxeKey = deriveKey(this.#mxe.privateKey, this.#client.publicKey);
    if (!clientKey.equals(mxeKey)) throw new Error("X25519 shared-secret mismatch");
    const envelope = encryptJson(input, clientKey);
    const privateInput = decryptJson<AllocationInput>(envelope, mxeKey);
    const output = computeReferenceAllocation(privateInput);
    return {
      id: randomUUID(),
      output,
      encryptedBytes: envelope.ciphertext.byteLength,
    };
  }
}

