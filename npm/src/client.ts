import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils/testnet-parser';
import { Registry } from './registry';
import { allJsonRegistries } from './json';

export class ChainRegistryClient {
  get(chainId: string): Registry {
    const chainIdToIndex = this.swapIfPreviewChain(chainId);
    const jsonRegistry = allJsonRegistries[chainIdToIndex];
    if (!jsonRegistry) {
      throw new Error(`Registry not found for ${chainIdToIndex}`);
    }

    return new Registry(jsonRegistry);
  }

  version() {
    return {
      commit: __COMMIT_HASH__,
      date: new Date(__COMMIT_DATE__),
    };
  }

  private swapIfPreviewChain(chainId: string): string {
    if (!isTestnetPreviewChainId(chainId)) return chainId;

    const derivedChainId = deriveTestnetChainIdFromPreview(chainId);
    if (!derivedChainId) {
      throw new Error(`Chain id could not be derived from testnet preview chain: ${chainId}`);
    }
    return derivedChainId;
  }
}
