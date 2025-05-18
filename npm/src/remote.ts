import { Registry } from './registry';
import { GithubFetcher } from './github';
import { RegistryGlobals } from './globals';
import { BundledClient } from './bundled';
import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils/testnet-parser';
import { RegistryOptions } from './options';

export class RemoteClient {
  private readonly github;

  constructor(
    private readonly bundled: BundledClient,
    options?: RegistryOptions,
  ) {
    this.github = new GithubFetcher(options);
  }

  async get(chainId: string): Promise<Registry> {
    try {
      return await this.github.fetchRegistry(chainId);
    } catch (e) {
      if (isTestnetPreviewChainId(chainId)) {
        const fallbackChainId = deriveTestnetChainIdFromPreview(chainId);
        console.warn(`Attempting to fetch fallback chain registry: ${fallbackChainId}`);
        return await this.github.fetchRegistry(fallbackChainId);
      }
      throw e;
    }
  }

  // If remote fails, fall back to bundled registry for chain
  async getWithBundledBackup(chainId: string): Promise<Registry> {
    try {
      return await this.get(chainId);
    } catch (e: unknown) {
      console.warn(
        `Unable to fetch remote registry for ${chainId}, attempting to return bundled. Fetch err: ${String(e)}`,
      );
      return this.bundled.get(chainId);
    }
  }

  async globals(): Promise<RegistryGlobals> {
    return this.github.fetchGlobals();
  }
}
