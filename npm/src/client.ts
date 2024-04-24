import { GithubFetcher } from './github';
import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils';
import { Registry } from './registry';

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
