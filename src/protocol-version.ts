import type { ProtocolVersionJSON } from '@emurgo/cardano-serialization-lib-nodejs';

export class ProtocolVersion {
  private constructor(readonly major: number, readonly minor?: number) {}

  public static create(major: number, minor?: number) {
    return new ProtocolVersion(major, minor);
  }

  public static fromJSON(data: ProtocolVersionJSON) {
    return new ProtocolVersion(data.major, data.minor);
  }
}
