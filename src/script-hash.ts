import type { Hex } from './hex';

export class ScriptHash {
  private constructor(readonly value: Hex) {}

  public static create(value: Hex) {
    return new ScriptHash(value);
  }
}
