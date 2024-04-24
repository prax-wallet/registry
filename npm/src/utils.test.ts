import { describe, expect, it } from 'vitest';
import {
  base64ToUint8Array,
  deriveTestnetChainIdFromPreview,
  isTestnetPreviewChainId,
  mapObjectValues,
  uint8ArrayToBase64,
} from './utils';

describe('testnet-preview helper', () => {
  it('should correctly identify testnet-preview chain-id', () => {
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-6-711be12a')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-222-711be12a')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-rhea-8b2dfc5c')).toBeTruthy();
    expect(isTestnetPreviewChainId('penumbra-testnet-tethy12-8777cb20')).toBeTruthy();
  });

  it('should not identify chain-id as testnet-preview', () => {
    expect(isTestnetPreviewChainId('penumbra-mainnet')).toBeFalsy();
    expect(isTestnetPreviewChainId('penumbra-testnet-rhea')).toBeFalsy();
    expect(isTestnetPreviewChainId('penumbra-testnet-deimos-6')).toBeFalsy();
  });

  it('should correctly derive testnet chain-id from testnet-preview chain-id', () => {
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

describe('mapObjectValues', () => {
  it('should apply a function to each value in the object', () => {
    const original = { a: 1, b: 2, c: 3 };
    const expected = { a: 2, b: 3, c: 4 };
    const result = mapObjectValues(original, x => x + 1);
    expect(result).toEqual(expected);
  });

  it('should handle objects with various types of values', () => {
    const original = { a: 'hello', b: true, c: 10 };
    const expected = { a: 'HELLO', b: 'TRUE', c: '10' };
    const result = mapObjectValues(original, x => String(x).toUpperCase());
    expect(result).toEqual(expected);
  });

  it('should not mutate the original object', () => {
    const original = { a: 1, b: 2, c: 3 };
    const originalCopy = { ...original };
    mapObjectValues(original, x => x * 10);
    expect(original).toEqual(originalCopy);
  });
});

describe('uint8ArrayToBase64', () => {
  it('converts an empty Uint8Array to an empty string', () => {
    const byteArray = new Uint8Array();
    const result = uint8ArrayToBase64(byteArray);
    expect(result).toBe('');
  });

  it('correctly converts a Uint8Array to a base64 string', () => {
    const byteArray = new Uint8Array([104, 101, 108, 108, 111]); // ASCII for "hello"
    const result = uint8ArrayToBase64(byteArray);
    expect(result).toBe('aGVsbG8='); // base64 for "hello"
  });

  it('correctly converts a large Uint8Array to a base64 string', () => {
    const largeArray = new Uint8Array(1024).fill(97); // 1024 bytes all set to ASCII for "a"
    const result = uint8ArrayToBase64(largeArray);
    expect(result).toBe(Buffer.from(largeArray).toString('base64')); // compare with Node's built-in conversion
  });
});

describe('base64ToUint8Array', () => {
  it('should correctly convert a Base64 string to Uint8Array', () => {
    const base64String = 'SGVsbG8gd29ybGQ='; // 'Hello world' in Base64
    const expectedUint8Array = new Uint8Array([
      72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
    ]); // 'Hello world' in ASCII values
    expect(base64ToUint8Array(base64String)).toEqual(expectedUint8Array);
  });

  it('should return an empty Uint8Array for an empty Base64 string', () => {
    const base64String = '';
    const expectedUint8Array = new Uint8Array([]);
    expect(base64ToUint8Array(base64String)).toEqual(expectedUint8Array);
  });

  it('should handle non-Base64 strings', () => {
    const nonBase64String = 'This is not a Base64 string';
    expect(() => base64ToUint8Array(nonBase64String)).toThrow();
  });
});
