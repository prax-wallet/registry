import { Metadata } from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { Jsonified } from './utils';
import { Base64AssetId, Chain, Registry, Rpc } from './registry';

export interface GithubRegistryResponse {
  chainId: string;
  ibcConnections: Chain[];
  rpcs: Rpc[];
  assetById: Record<Base64AssetId, Jsonified<Metadata>>;
  stakingAssetId: Base64AssetId;
  numeraires: Base64AssetId[];
}

const REGISTRY_BASE_URL = 'https://raw.githubusercontent.com/prax-wallet/registry/main/registry';

type ChainId = string;

export class GithubFetcher {
  private cache: Record<ChainId, Registry> = {};

  async fetchRegistryData(chainId: ChainId): Promise<Registry> {
    if (this.cache[chainId]) return this.cache[chainId]!;
    const response = await this.typedFetcher<GithubRegistryResponse>(
      `${REGISTRY_BASE_URL}/${chainId}.json`,
    );
    return new Registry(response);
  }

  clearCache(): void {
    this.cache = {};
  }

  private async typedFetcher<T>(url: string): Promise<T> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch from: ${url}`);
    }
    return (await response.json()) as T;
  }
}
