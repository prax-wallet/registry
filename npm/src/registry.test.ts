import { describe, expect, it } from 'vitest';
import { Registry } from './registry';
import { AssetId } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { base64ToUint8Array } from './utils/base64';
import * as testRegistry from '../../registry/chains/penumbra-testnet-deimos-8.json';

describe('Registry', () => {
  it('gets metadata successfully', () => {
    const registry = new Registry(testRegistry);
    const usdcId = base64ToUint8Array('reum7wQmk/owgvGMWMZn/6RFPV24zIKq3W6In/WwZgg=');
    const res = registry.getMetadata(new AssetId({ inner: usdcId }));
    expect(res.base).toEqual('wtest_usd');
  });

  it('throws when searching for metadata that does not exist', () => {
    const registry = new Registry(testRegistry);
    const cubeId = base64ToUint8Array('aGVsbG8gd29ybGQ=');
    const getCubeMetadata = () => registry.getMetadata(new AssetId({ inner: cubeId }));
    expect(getCubeMetadata).toThrow();
  });

  it.skip('gets all assets successfully', () => {
    const registry = new Registry(testRegistry);
    const res = registry.getAllAssets();
    expect(res.length).toEqual(20);
  });

  it.skip('versions correctly', async () => {
    const registry = new Registry(testRegistry);
    const version = await registry.version();
    expect(version).toEqual('9eaf48c7cbf3248e6979830cfc982f2208eeec0fcc4c0e2802f0bd43c8bffad3');
  });
});
