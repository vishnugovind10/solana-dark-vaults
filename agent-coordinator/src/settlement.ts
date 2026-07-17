import { BPS_DENOMINATOR, type AllocationOutput } from "./types.js";

export class MockSettlementAdapter {
  #positions: [number, number, number] = [0, 0, 0];

  settle(totalCapital: number, output: AllocationOutput): readonly [number, number, number] {
    const sum = output.weightsBps.reduce((total, weight) => total + weight, 0);
    if (sum !== BPS_DENOMINATOR) throw new Error("settlement weights must sum to 10000 bps");
    const first = Math.floor((totalCapital * output.weightsBps[0]) / BPS_DENOMINATOR);
    const second = Math.floor((totalCapital * output.weightsBps[1]) / BPS_DENOMINATOR);
    const third = totalCapital - first - second;
    this.#positions = [first, second, third];
    return this.positions();
  }

  positions(): readonly [number, number, number] {
    return [...this.#positions];
  }
}

