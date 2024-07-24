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
    expect(version).toEqual('fb3fb559596710d33d3a52946c6280162db1cbca7cc385fe9a3a320ebca40009');
  });
});
