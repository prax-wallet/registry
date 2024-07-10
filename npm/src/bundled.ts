import { Registry } from './registry';
import { allJsonRegistries, JsonRegistry } from './json';
import { sha256Hash } from './utils/sha256';
import { swapIfPreviewChain } from './preview';
import * as GlobalsJson from '../../registry/globals.json';
import { globalsFromJson, RegistryGlobals } from './globals';

export class BundledClient {
  get(chainId: string): Registry {
    const jsonRegistry = this.getJsonRegistry(chainId);
    return new Registry(jsonRegistry);
  }

  private getJsonRegistry(chainId: string): JsonRegistry {
    const chainIdToIndex = swapIfPreviewChain(chainId);
    const jsonRegistry = allJsonRegistries[chainIdToIndex];
    if (!jsonRegistry) {
      throw new Error(`Registry not found for ${chainIdToIndex}`);
    }
    return jsonRegistry;
  }

  globals(): RegistryGlobals {
    return globalsFromJson(GlobalsJson);
  }

  async version(chainId: string) {
    const registry = this.getJsonRegistry(chainId);
    const globals = this.globals();
    const registryHash = await sha256Hash(JSON.stringify(registry));
    const globalsHash = await sha256Hash(JSON.stringify(globals));

    return await sha256Hash(registryHash + globalsHash);
  }
}
