import { describe, expect, it } from 'vitest';
import { Registry } from './registry';
import { AssetId } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { base64ToUint8Array } from './utils/base64';
import * as testRegistry from '../../registry/chains/penumbra-testnet-deimos-8-x6de97e39.json';

describe('Registry', () => {
  it('gets metadata successfully', () => {
    const registry = new Registry(testRegistry);
    const usdcId = base64ToUint8Array('reum7wQmk/owgvGMWMZn/6RFPV24zIKq3W6In/WwZgg=');
    const res = registry.getMetadata(new AssetId({ inner: usdcId }));
    expect(res.base).toEqual('wtest_usd');
    expect(res.badges.length).toEqual(1);
    expect(res.badges[0]?.png).toEqual(
      'https://raw.githubusercontent.com/prax-wallet/registry/main/images/penumbra-favicon.png',
    );
  });

  it('throws when searching for metadata that does not exist', () => {
    const registry = new Registry(testRegistry);
    const cubeId = base64ToUint8Array('aGVsbG8gd29ybGQ=');
    const getCubeMetadata = () => registry.getMetadata(new AssetId({ inner: cubeId }));
    expect(getCubeMetadata).toThrow();
  });

  it('returns undefined when using try method', () => {
    const registry = new Registry(testRegistry);
    const cubeId = base64ToUint8Array('aGVsbG8gd29ybGQ=');
    const result = registry.tryGetMetadata(new AssetId({ inner: cubeId }));
    expect(result).toBeUndefined();
  });

  it('gets all assets successfully', () => {
    const registry = new Registry(testRegistry);
    const res = registry.getAllAssets();
    expect(res.length).toBeGreaterThan(0);
  });

  it('versions without throwing', async () => {
    const registry = new Registry(testRegistry);
    await expect(registry.version()).resolves.not.toThrow();
  });
});
