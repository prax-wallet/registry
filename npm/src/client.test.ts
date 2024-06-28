import { describe, it, expect } from 'vitest';
import { ChainRegistryClient } from './client';
import { Registry } from './registry';
import { AssetId } from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { base64ToUint8Array } from './utils/base64';

describe('ChainRegistryClient', () => {
  const client = new ChainRegistryClient();

  it('returns a Registry object for valid chain IDs', () => {
    const registry = client.get('penumbra-testnet-deimos-7');
    expect(registry).toBeInstanceOf(Registry);
    expect(registry.chainId).toEqual('penumbra-testnet-deimos-7');
  });

  it('throws an error when no registry is found', () => {
    expect(() => client.get('unknown')).toThrow('Registry not found for unknown');
  });

  it('handles preview chain IDs by swapping them', () => {
    const registry = client.get('penumbra-testnet-deimos-7-xf2dbce94');
    expect(registry).toBeInstanceOf(Registry);
    expect(registry.chainId).toEqual('penumbra-testnet-deimos-7');
  });

  it('returns staking asset global as expected', () => {
    const registry = client.globals();
    const umStakingAsset = new AssetId({
      inner: base64ToUint8Array('KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA='),
    });
    expect(umStakingAsset.equals(registry.stakingAssetId)).toBeTruthy();
  });
});
