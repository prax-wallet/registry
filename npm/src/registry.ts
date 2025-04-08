import { AssetId, Denom, Metadata } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { JsonRegistry } from './json';
import { JsonValue } from '@bufbuild/protobuf';
import { base64ToUint8Array, uint8ArrayToBase64 } from './utils/base64';
import { sha256Hash } from './utils/sha256';

// @ts-expect-error alias for dev only
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Stringified<T> = string;

export type Base64AssetId = Stringified<AssetId['inner']>;

export interface Chain {
  addressPrefix: string;
  chainId: string;
  channelId: string;
  counterpartyChannelId: string;
  images: Image[];
  displayName: string;
}

export interface EntityMetadata {
  name: string;
  url: string;
  images: Image[];
}

export interface Image {
  png?: string;
  svg?: string;
}

export const isDenom = (value?: Denom | AssetId): value is Denom =>
  value?.getType().typeName === Denom.typeName;

export class Registry {
  public readonly chainId: string;
  public readonly ibcConnections: Chain[];
  public readonly numeraires: AssetId[];

  private readonly assetById: Record<Base64AssetId, Metadata> = {};
  private readonly assetByDenom: Record<string, Metadata> = {};

  constructor(r: JsonRegistry) {
    this.chainId = r.chainId;
    this.ibcConnections = r.ibcConnections;
    this.numeraires = r.numeraires.map(a => new AssetId({ inner: base64ToUint8Array(a) }));

    // Create two mapping objects to query Metadata efficiently by assetBy and denom
    Object.entries(r.assetById).forEach(([key, value]) => {
      const metadata = Metadata.fromJson(value as unknown as JsonValue, {
        ignoreUnknownFields: true,
      });

      this.assetById[key] = metadata;
      this.assetByDenom[metadata.base] = metadata;
    });
  }

  private _resolveMetadata(id: AssetId | Denom, throwIfMissing: boolean): Metadata | undefined {
    const metadata = isDenom(id)
      ? this.assetByDenom[id.denom]
      : this.assetById[uint8ArrayToBase64(id.inner)];

    if (!metadata && throwIfMissing) {
      const missingKey = isDenom(id) ? id.denom : uint8ArrayToBase64(id.inner);
      const label = isDenom(id) ? 'denom' : 'asset id';
      throw new Error(`No metadata in registry for ${label}: ${missingKey}`);
    }

    return metadata;
  }

  // Throws an error if not in registry
  getMetadata(id: AssetId | Denom): Metadata {
    return this._resolveMetadata(id, true)!;
  }

  // Returns undefined if not in registry
  tryGetMetadata(id: AssetId | Denom): Metadata | undefined {
    return this._resolveMetadata(id, false);
  }

  getAllAssets(): Metadata[] {
    return Object.values(this.assetById);
  }

  async version(): Promise<string> {
    return sha256Hash(JSON.stringify(this));
  }
}
