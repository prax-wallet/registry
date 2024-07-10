import { Registry } from './registry';
import { GithubFetcher } from './github';
import { swapIfPreviewChain } from './preview';
import { RegistryGlobals } from './globals';

export class RemoteClient {
  private readonly github = new GithubFetcher();

  async get(chainId: string): Promise<Registry> {
    const chainIdToQuery = swapIfPreviewChain(chainId);
    return this.github.fetchRegistry(chainIdToQuery);
  }

  async globals(): Promise<RegistryGlobals> {
    return this.github.fetchGlobals();
  }
}
