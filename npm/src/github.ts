import { Base64AssetId, Chain, Registry, EntityMetadata } from './registry';
import { JsonGlobals, JsonMetadata } from './json';
import { RegistryGlobals } from './globals';

export interface GithubRegistryResponse {
  chainId: string;
  ibcConnections: Chain[];
  rpcs: EntityMetadata[];
  assetById: Record<Base64AssetId, JsonMetadata>;
  stakingAssetId: Base64AssetId;
  numeraires: Base64AssetId[];
}

export const REGISTRY_BASE_URL =
  'https://raw.githubusercontent.com/prax-wallet/registry/main/registry';

type ChainId = string;

export class GithubFetcher {
  async fetchRegistry(chainId: ChainId): Promise<Registry> {
    const response = await this.typedFetcher<GithubRegistryResponse>(
      `${REGISTRY_BASE_URL}/chains/${chainId}.json`,
      {
        cache: 'no-cache',
      },
    );
    return new Registry(response);
  }

  async fetchGlobals(): Promise<RegistryGlobals> {
    const response = await this.typedFetcher<JsonGlobals>(`${REGISTRY_BASE_URL}/globals.json`, {
      cache: 'no-cache',
    });
    return new RegistryGlobals(response);
  }

  private async typedFetcher<T>(url: string, init?: RequestInit): Promise<T> {
    const response = await fetch(url, init);
    if (!response.ok) {
      throw new Error(`Failed to fetch from: ${url}`);
    }
    return (await response.json()) as T;
  }
}
