import { Registry } from './registry';
import { allJsonRegistries } from './json';
import { swapIfPreviewChain } from './preview';
import * as GlobalsJson from '../../registry/globals.json';
import { RegistryGlobals } from './globals';

export class BundledClient {
  get(chainId: string): Registry {
    const chainIdToIndex = swapIfPreviewChain(chainId);
    const jsonRegistry = allJsonRegistries[chainIdToIndex];
    if (!jsonRegistry) {
      throw new Error(`Registry not found for ${chainIdToIndex}`);
    }
    return new Registry(jsonRegistry);
  }

  globals(): RegistryGlobals {
    return new RegistryGlobals(GlobalsJson);
  }
}
