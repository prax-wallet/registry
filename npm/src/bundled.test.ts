import { describe, it, expect } from 'vitest';
import { ChainRegistryClient } from './client';
import { Registry } from './registry';
import { AssetId } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { base64ToUint8Array } from './utils/base64';

describe('BundledClient', () => {
  const { bundled } = new ChainRegistryClient();

  it('returns a Registry object for valid chain IDs', () => {
    const registry = bundled.get('penumbra-1');
    expect(registry).toBeInstanceOf(Registry);
    expect(registry.chainId).toEqual('penumbra-1');
  });

  it('throws an error when no registry is found', () => {
    expect(() => bundled.get('unknown')).toThrow('Registry not found for unknown');
  });

  it.skip('handles preview chain IDs by swapping them', () => {
    const registry = bundled.get('penumbra-testnet-deimos-8-xf2dbce94');
    expect(registry).toBeInstanceOf(Registry);
    expect(registry.chainId).toEqual('penumbra-testnet-deimos-8');
  });

  it('returns staking asset global as expected', () => {
    const registry = bundled.globals();
    const umStakingAsset = new AssetId({
      inner: base64ToUint8Array('KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA='),
    });
    expect(umStakingAsset.equals(registry.stakingAssetId)).toBeTruthy();
  });
});
