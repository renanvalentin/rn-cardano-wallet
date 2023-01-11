export class Ed25519KeyHash {
  private constructor(readonly value?: string) {}

  public static create(value: string) {
    return new Ed25519KeyHash(value);
  }
}
export type Ed25519KeyHashes = Ed25519KeyHash[];
