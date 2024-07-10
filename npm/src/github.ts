import { Base64AssetId, Chain, Registry, Rpc } from './registry';
import { JsonGlobals, JsonMetadata } from './json';
import { globalsFromJson, RegistryGlobals } from './globals';

export interface GithubRegistryResponse {
  chainId: string;
  ibcConnections: Chain[];
  rpcs: Rpc[];
  assetById: Record<Base64AssetId, JsonMetadata>;
  stakingAssetId: Base64AssetId;
  numeraires: Base64AssetId[];
}

const REGISTRY_BASE_URL = 'https://raw.githubusercontent.com/prax-wallet/registry/main/registry';

type ChainId = string;

export class GithubFetcher {
  private registryCache: Record<ChainId, Registry> = {};
  private globalsCache: RegistryGlobals | undefined = undefined;

  async fetchRegistry(chainId: ChainId): Promise<Registry> {
    if (this.registryCache[chainId]) {
      return this.registryCache[chainId];
    }
    const response = await this.typedFetcher<GithubRegistryResponse>(
      `${REGISTRY_BASE_URL}/chains/${chainId}.json`,
    );
    this.registryCache[chainId] = new Registry(response);
    return this.registryCache[chainId];
  }

  async fetchGlobals(): Promise<RegistryGlobals> {
    if (this.globalsCache) {
      return this.globalsCache;
    }
    const response = await this.typedFetcher<JsonGlobals>(`${REGISTRY_BASE_URL}/globals.json`);

    this.globalsCache = globalsFromJson(response);
    return this.globalsCache;
  }

  clearCache(): void {
    this.registryCache = {};
    this.globalsCache = undefined;
  }

  private async typedFetcher<T>(url: string): Promise<T> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch from: ${url}`);
    }
    return (await response.json()) as T;
  }
}
