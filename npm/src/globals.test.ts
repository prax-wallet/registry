import { describe, expect, it } from 'vitest';
import { RegistryGlobals } from './globals';
import { JsonGlobals } from './json';

const testGlobals: JsonGlobals = {
  rpcs: [{ name: 'rpc1', images: [], url: 'http://rpc1.com' }],
  frontendsV2: [{ name: 'frontend1', images: [], url: 'http://example.com' }],
  frontends: ['frontend1', 'frontend2', 'frontend3'],
  stakingAssetId: {
    inner: 'KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA=',
  },
};

describe('Globals', () => {
  it('versions correctly', async () => {
    const registry = new RegistryGlobals(testGlobals);
    const version = await registry.version();
    expect(version).toEqual('e5a0ca843f96a84314751a140677b448c510b6b782d505e093a2503bdcb6b13d');
  });
});
