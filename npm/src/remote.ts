import { Registry } from './registry';
import { GithubFetcher } from './github';
import { swapIfPreviewChain } from './preview';
import { RegistryGlobals } from './globals';
import { BundledClient } from './bundled';

export class RemoteClient {
  private readonly github = new GithubFetcher();

  constructor(private readonly bundled: BundledClient) {}

  async get(chainId: string): Promise<Registry> {
    const chainIdToQuery = swapIfPreviewChain(chainId);
    return this.github.fetchRegistry(chainIdToQuery);
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
