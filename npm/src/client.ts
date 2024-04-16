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

interface IbcConfig {
  addressPrefix: string;
  chainId: string;
  ibcChannel: string;
}

interface Rpc {
  name: string;
  url: string;
  images: Image[];
}

interface Image {
  png?: string;
  svg?: string;
}

export interface RegistryOptions {
  chainIds: string[];
}

export class ChainRegistryClient {
  private readonly github: GithubFetcher;

  constructor(options: RegistryOptions) {
    this.github = new GithubFetcher(options.chainIds);
  }

  async allRegistries(): Promise<Registry[]> {
    return this.github.fetchRegistryData();
  }

  async registry(chainId: string): Promise<Registry> {
    const all = await this.github.fetchRegistryData();
    const match = all.find(r => r.chainId === chainId);
    if (!match) {
      throw new Error(
        `Unable to fetch registry for ${chainId}. Did you pass in the right chain names?`,
      );
    }
    return match;
  }

  clearCache() {
    this.github.clearCache();
  }
}

// // Example usage:
// const client = new ChainRegistryClient({ chainNames: ['osmosis', 'cosmos'] });
// client.fetchRegistryData().then(data => {
//   console.log(data); // Process or display the fetched data
// });
