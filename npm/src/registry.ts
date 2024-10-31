import { AssetId, Metadata } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { JsonRegistry } from './json';
import { JsonValue } from '@bufbuild/protobuf';
import { base64ToUint8Array, uint8ArrayToBase64 } from './utils/base64';
import { mapObjectValues } from './utils/object-mapping';
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

export class Registry {
  public readonly chainId: string;
  public readonly ibcConnections: Chain[];
  public readonly numeraires: AssetId[];

  private readonly assetById: Record<Base64AssetId, Metadata>;

  constructor(r: JsonRegistry) {
    this.chainId = r.chainId;
    this.ibcConnections = r.ibcConnections;
    this.assetById = mapObjectValues(r.assetById, jsonMetadata =>
      Metadata.fromJson(jsonMetadata as unknown as JsonValue, { ignoreUnknownFields: true }),
    );
    this.numeraires = r.numeraires.map(a => new AssetId({ inner: base64ToUint8Array(a) }));
  }

  // Throws an error if not in registry
  getMetadata(id: AssetId): Metadata {
    const key = uint8ArrayToBase64(id.inner);
    const metadata = this.assetById[key];
    if (!metadata) {
      throw new Error(`No metadata in registry for asset id: ${key}`);
    }
    return metadata;
  }

  // Returns undefined if not in registry
  tryGetMetadata(id: AssetId): Metadata | undefined {
    const key = uint8ArrayToBase64(id.inner);
    const metadata = this.assetById[key];
    if (!metadata) {
      return undefined;
    }
    return metadata;
  }

  getAllAssets(): Metadata[] {
    return Object.values(this.assetById);
  }

  async version(): Promise<string> {
    return sha256Hash(JSON.stringify(this));
  }
}
