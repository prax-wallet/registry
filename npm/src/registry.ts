import {
  AssetId,
  Metadata,
} from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { base64ToUint8Array, mapObjectValues, Stringified, uint8ArrayToBase64 } from './utils';
import { GithubRegistryResponse } from './github';

export type Base64AssetId = Stringified<AssetId['inner']>;

export interface Chain {
  addressPrefix: string;
  chainId: string;
  ibcChannel: string;
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

  constructor(response: GithubRegistryResponse) {
    this.chainId = response.chainId;
    this.ibcConnections = response.ibcConnections;
    this.rpcs = response.rpcs;
    this.assetById = mapObjectValues(response.assetById, id => Metadata.fromJson(id));
    this.stakingAssetId = new AssetId({ inner: base64ToUint8Array(response.stakingAssetId) });
    this.numeraires = response.numeraires.map(a => new AssetId({ inner: base64ToUint8Array(a) }));
  }

  getMetadata(id: AssetId): Metadata {
    const key = uint8ArrayToBase64(id.inner);
    const metadata = this.assetById[key];
    if (!metadata) {
      throw new Error(`No metadata in registry for asset id: ${key}`);
    }
    return metadata;
  }
}
