import { GithubFetcher } from './github';
import {
  AssetId,
  Metadata,
} from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils';

// @ts-expect-error alias for dev only
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Jsonified<T> = string;

export interface Registry {
  chainId: string;
  ibcConnections: Chain[];
  rpcs: Rpc[];
  assetById: Record<Jsonified<AssetId>, Jsonified<Metadata>>;
  stakingAssetId: Jsonified<AssetId>;
  numeraires: Jsonified<AssetId>[];
}

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

export class ChainRegistryClient {
  private readonly github: GithubFetcher;

  constructor() {
    this.github = new GithubFetcher();
  }

  async get(chainId: string): Promise<Registry> {
    if (isTestnetPreviewChainId(chainId)) {
      const derivedChainId = deriveTestnetChainIdFromPreview(chainId);
      if (derivedChainId) {
        return this.github.fetchRegistryData(derivedChainId);
      }
    }
    return this.github.fetchRegistryData(chainId);
  }

  clearCache() {
    this.github.clearCache();
  }
}
