import { GithubFetcher } from './github';
import {
  AssetId,
  Metadata,
} from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';

// @ts-expect-error alias for dev only
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Jsonified<T> = string;

export interface Registry {
  chainId: string;
  ibcConfig: IbcConfig[];
  rpcs: Rpc[];
  assetById: Record<Jsonified<AssetId>, Metadata>;
}

export interface IbcConfig {
  addressPrefix: string;
  chainId: string;
  ibcChannel: string;
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

export class ChainRegistryClient {
  private readonly github: GithubFetcher;

  constructor() {
    this.github = new GithubFetcher();
  }

  async get(chainId: string): Promise<Registry> {
    return this.github.fetchRegistryData(chainId);
  }

  clearCache() {
    this.github.clearCache();
  }
}
