import { describe, expect, test } from 'vitest';
import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils';

describe('testnet-preview helper', () => {
  test('should correctly identify testnet-preview chain-id', () => {
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-6-711be12a')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-222-711be12a')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-rhea-8b2dfc5c')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-tethy12-8777cb20')).toBeTruthy();
  });

  test('should not identify chain-id as testnet-preview', () => {
    expect(isTestnetPreviewChainId('penumbra-mainnet')).toBeFalsy();
    expect(isTestnetPreviewChainId('penumbra-testnet-rhea')).toBeFalsy();
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-6')).toBeFalsy();
  });

  test('should correctly derive testnet chain-id from testnet-preview chain-id', () => {
    expect(deriveTestnetChainIdFromPreview('penumbra-testnet-deimos-6-711be12a')).toEqual(
      'penumbra-testnet-deimos-6',
    );
    expect(deriveTestnetChainIdFromPreview('penumbra-testnet-rhea-8b2dfc5c')).toEqual(
      'penumbra-testnet-rhea',
    );
    expect(deriveTestnetChainIdFromPreview('penumbra-testnet-tethys12-8777cb20')).toEqual(
      'penumbra-testnet-tethys12',
    );
  });
});
