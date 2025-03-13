import { describe, it, expect } from 'vitest';
import { create, equals } from '@bufbuild/protobuf';
import { ChainRegistryClient } from './client';
import { Registry } from './registry';
import { AssetIdSchema } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
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

  it('returns a random suffix chain ID if available', () => {
    const registry = bundled.get('penumbra-testnet-deimos-8-x6de97e39');
    expect(registry).toBeInstanceOf(Registry);
    expect(registry.chainId).toEqual('penumbra-testnet-deimos-8-x6de97e39');
  });

  it('swaps a random suffix chain ID by swapping them if it is not present', () => {
    const registry = bundled.get('penumbra-testnet-phobos-1-xf2dbce94');
    expect(registry).toBeInstanceOf(Registry);
    expect(registry.chainId).toEqual('penumbra-testnet-phobos-1');
  });

  it('fails if swapped random suffix chain ID is not present', () => {
    expect(() => bundled.get('penumbra-testnet-jupiter-1-xf2dbce94')).toThrow();
  });

  it('returns staking asset global as expected', () => {
    const registry = bundled.globals();
    const umStakingAsset = create(AssetIdSchema, {
      inner: base64ToUint8Array('KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA='),
    });
    expect(equals(AssetIdSchema, umStakingAsset, registry.stakingAssetId)).toBeTruthy();
  });
});
