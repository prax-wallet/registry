import { Registry } from './registry';
import { allJsonRegistries } from './json';
import * as GlobalsJson from '../../registry/globals.json';
import { RegistryGlobals } from './globals';
import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils/testnet-parser';

export class BundledClient {
  get(chainId: string): Registry {
    const jsonRegistry = allJsonRegistries[chainId];
    if (jsonRegistry) {
      return new Registry(jsonRegistry);
    }

    if (isTestnetPreviewChainId(chainId)) {
      const fallbackChainId = deriveTestnetChainIdFromPreview(chainId);
      console.warn(`Attempting to get fallback chain registry: ${fallbackChainId}`);
      const fallbackRegistry = allJsonRegistries[fallbackChainId];
      if (fallbackRegistry) {
        return new Registry(fallbackRegistry);
      }
    }

    throw new Error(`Registry not found for ${chainId}`);
  }

  globals(): RegistryGlobals {
    return new RegistryGlobals(GlobalsJson);
  }
}
