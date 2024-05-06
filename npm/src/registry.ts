import {
  AssetId,
  Metadata,
} from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { base64ToUint8Array, mapObjectValues, Stringified, uint8ArrayToBase64 } from './utils';
import { JsonRegistry } from './json';
import { JsonValue } from '@bufbuild/protobuf';

export type Base64AssetId = Stringified<AssetId['inner']>;

export interface Chain {
  addressPrefix: string;
  chainId: string;
  channelId: string;
  counterpartyChannelId: string;
  images: Image[];
  displayName: string;
}

export interface Rpc {
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
  public readonly rpcs: Rpc[];
  public readonly stakingAssetId: AssetId;
  public readonly numeraires: AssetId[];

  private readonly assetById: Record<Base64AssetId, Metadata>;

  constructor(r: JsonRegistry) {
    this.chainId = r.chainId;
    this.ibcConnections = r.ibcConnections;
    this.rpcs = r.rpcs;
    this.assetById = mapObjectValues(r.assetById, jsonMetadata =>
      Metadata.fromJson(jsonMetadata as unknown as JsonValue),
    );
    this.stakingAssetId = new AssetId({ inner: base64ToUint8Array(r.stakingAssetId) });
    this.numeraires = r.numeraires.map(a => new AssetId({ inner: base64ToUint8Array(a) }));
  }

  getMetadata(id: AssetId): Metadata {
    const key = uint8ArrayToBase64(id.inner);
    const metadata = this.assetById[key];
    if (!metadata) {
      throw new Error(`No metadata in registry for asset id: ${key}`);
    }
    return metadata;
  }

  getAllAssets(): Metadata[] {
    return Object.values(this.assetById);
  }
}
