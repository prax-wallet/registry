import { Base64AssetId, Chain, Registry, EntityMetadata } from './registry';
import { JsonGlobals, JsonMetadata } from './json';
import { RegistryGlobals } from './globals';
import { RegistryOptions } from './options';

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
  constructor(private readonly options?: RegistryOptions) {}

  async fetchRegistry(chainId: ChainId): Promise<Registry> {
    const response = await this.typedFetcher<GithubRegistryResponse>(
      `${REGISTRY_BASE_URL}/chains/${chainId}.json`,
    );
    return new Registry(response);
  }

  async fetchGlobals(): Promise<RegistryGlobals> {
    const response = await this.typedFetcher<JsonGlobals>(`${REGISTRY_BASE_URL}/globals.json`);
    return new RegistryGlobals(response);
  }

  private async typedFetcher<T>(url: string): Promise<T> {
    // If we figure that we're running on server side NextJS, we need to set
    // 'force-cache', so that caching is enabled *at all*, because their silly
    // polyfill messes up the default semantics of fetch.
    //
    // If we're in the browser, or not using NextJS, we're fine.
    const cache =
      this.options?.nextjsServerSide && typeof window === 'undefined' ? 'force-cache' : 'default';
    const response = await fetch(url, { cache });
    if (!response.ok) {
      throw new Error(`Failed to fetch from: ${url}`);
    }
    return (await response.json()) as T;
  }
}
