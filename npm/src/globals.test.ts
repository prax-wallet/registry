import { describe, expect, it } from 'vitest';
import { RegistryGlobals } from './globals';
import { JsonGlobals } from './json';

const testGlobals: JsonGlobals = {
  rpcs: [{ name: 'rpc1', images: [], url: 'http://rpc1.com' }],
  websites: [{ name: 'frontend1', images: [], url: 'http://example.com' }],
  frontends: ['frontend1', 'frontend2', 'frontend3'],
  stakingAssetId: {
    inner: 'KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA=',
  },
};

describe('Globals', () => {
  it('versions correctly', async () => {
    const registry = new RegistryGlobals(testGlobals);
    const version = await registry.version();
    expect(version).toEqual('7e4059f5641482cfc817222cb5cbd6f62174d578d8473ff4b0a13ef643363b7e');
  });
});
