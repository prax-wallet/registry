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

export class GithubFetcher {
  private cache: Registry[] = [];

  constructor(private readonly chainIds: string[]) {
  }

  async fetchRegistryData(): Promise<Registry[]> {
    if (this.cache.length) return this.cache;

    const chains = await this.typedFetcher<GithubContentsRes[]>(
      `${REGISTRY_URL}/contents/registry`,
    );
    // Filter and fetch only the JSON files that match the chainNames
    const chainDataPromises = chains
      .filter(res => this.matchesChains(res))
      .map(file => this.typedFetcher<Registry>(file.download_url));

    return await Promise.all(chainDataPromises);
  }

  clearCache(): void {
    this.cache = [];
  }

  private async typedFetcher<T>(url: string): Promise<T> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch from: ${url}`);
    }
    return await response.json() as T;
  }

  private matchesChains({ name }: GithubContentsRes): boolean {
    if (!name.endsWith('.json')) return false;
    const withoutExt = name.replace('.json', '');
    return this.chainIds.includes(withoutExt);
  }
}
