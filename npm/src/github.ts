import { Registry } from './client';

export interface GithubContentsRes {
  name: string;
  path: string;
  sha: string;
  size: number;
  url: string;
  html_url: string;
  git_url: string;
  download_url: string;
  type: string;
  _links: {
    self: string;
    git: string;
    html: string;
  };
}

const REGISTRY_URL = 'https://api.github.com/repos/prax-wallet/registry';

type ChainId = string;

export class GithubFetcher {
  private cache: Record<ChainId, Registry> = {};

  async fetchRegistryData(chainId: ChainId): Promise<Registry> {
    if (this.cache[chainId]) return this.cache[chainId]!;

    const chains = await this.typedFetcher<GithubContentsRes[]>(
      `${REGISTRY_URL}/contents/registry`,
    );

    const match = chains.find(res => this.matchesChain(res, chainId));
    if (!match) throw new Error(`Could not find registry for ${chainId}`);

    return await this.typedFetcher<Registry>(match.download_url);
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

  private matchesChain({ name }: GithubContentsRes, chain: ChainId): boolean {
    if (!name.endsWith('.json')) return false;
    const withoutExt = name.replace('.json', '');
    return chain === withoutExt;
  }
}
