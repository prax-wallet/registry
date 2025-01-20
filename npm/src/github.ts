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

const DEFAULT_REGISTRY_BASE_URL =
  'https://raw.githubusercontent.com/prax-wallet/registry/main/registry';

type ChainId = string;

export class GithubFetcher {
  constructor(private baseUrl: string = DEFAULT_REGISTRY_BASE_URL) {}

  async fetchRegistry(chainId: ChainId): Promise<Registry> {
  const response = await this.typedFetcher<GithubRegistryResponse>(
      `${this.baseUrl}/chains/${chainId}.json`,
    );
    return new Registry(response);
  }

  async fetchGlobals(): Promise<RegistryGlobals> {
    const response = await this.typedFetcher<JsonGlobals>(`${this.baseUrl}/globals.json`);
    return new RegistryGlobals(response);
  }

  private async typedFetcher<T>(url: string): Promise<T> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch from: ${url}`);
    }
    return (await response.json()) as T;
  }
}
