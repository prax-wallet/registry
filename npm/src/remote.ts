import { Registry } from './registry';
import { GithubFetcher } from './github';
import { swapIfPreviewChain } from './preview';
import { RegistryGlobals } from './globals';
import { sha256Hash } from './utils/sha256';

export class RemoteClient {
  private readonly github = new GithubFetcher();

  async get(chainId: string): Promise<Registry> {
    const chainIdToQuery = swapIfPreviewChain(chainId);
    return this.github.fetchRegistry(chainIdToQuery);
  }

  clearCache() {
    this.github.clearCache();
  }

  async globals(): Promise<RegistryGlobals> {
    return this.github.fetchGlobals();
  }

  async version(chainId: string) {
    const registry = await this.get(chainId);
    const globals = await this.globals();
    const registryHash = await sha256Hash(JSON.stringify(registry));
    const globalsHash = await sha256Hash(JSON.stringify(globals));

    return await sha256Hash(registryHash + globalsHash);
  }
}
