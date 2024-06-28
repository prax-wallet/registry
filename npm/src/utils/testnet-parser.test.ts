import { describe, expect, it } from 'vitest';
import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './testnet-parser';

describe('testnet-preview helper', () => {
  it('should correctly identify testnet-preview chain-id', () => {
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-8-xf2dbce94')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-x0044bb22')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-rhea-xff99ee10')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-tethy12-xb4d8f9a0')).toBeTruthy();
  });

  it('should not identify chain-id as testnet-preview', () => {
    expect(isTestnetPreviewChainId('penumbra-mainnet')).toBeFalsy();
    expect(isTestnetPreviewChainId('penumbra-testnet-rhea')).toBeFalsy();
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-6')).toBeFalsy();
  });

  it('should correctly derive testnet chain-id from testnet-preview chain-id', () => {
    expect(deriveTestnetChainIdFromPreview('penumbra-testnet-deimos-8-xf2dbce94')).toEqual(
      'penumbra-testnet-deimos-8',
    );
    expect(deriveTestnetChainIdFromPreview('penumbra-testnet-rhea-x0044bb22')).toEqual(
      'penumbra-testnet-rhea',
    );
    expect(deriveTestnetChainIdFromPreview('penumbra-testnet-tethys12-xb4d8f9a0')).toEqual(
      'penumbra-testnet-tethys12',
    );
  });
});
