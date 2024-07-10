import { describe, it, expect } from 'vitest';
import { sha256Hash } from './sha256';

describe('sha256Hash', () => {
  it('correctly hashes an empty string', async () => {
    const result = await sha256Hash('');
    expect(result).toBe('e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855');
  });

  it('correctly hashes "Hello, world!"', async () => {
    const result = await sha256Hash('Hello, world!');
    expect(result).toBe('315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3');
  });

  it('correctly hashes unicode characters', async () => {
    const result = await sha256Hash('こんにちは');
    expect(result).toBe('125aeadf27b0459b8760c13a3d80912dfa8a81a68261906f60d87f4a0268646c');
  });

  it('correctly hashes hashes', async () => {
    const resultA = await sha256Hash('A');
    const resultB = await sha256Hash('B');
    const resultC = await sha256Hash(resultA + resultB);
    expect(resultC).toBe('b30ab174f7459cdd40a3acdf15d0c9444fec2adcfb9d579aa154c084885edd0a');
  });
});
