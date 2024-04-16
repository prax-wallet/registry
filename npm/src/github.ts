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

const REGISTRY_BASE_URL = 'https://raw.githubusercontent.com/prax-wallet/registry/main/registry';

type ChainId = string;

export class GithubFetcher {
  private cache: Record<ChainId, Registry> = {};

  async fetchRegistryData(chainId: ChainId): Promise<Registry> {
    if (this.cache[chainId]) return this.cache[chainId]!;
    return this.typedFetcher<Registry>(`${REGISTRY_BASE_URL}/${chainId}.json`);
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
