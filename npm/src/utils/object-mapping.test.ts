import { describe, expect, it } from 'vitest';
import { mapObjectValues } from './object-mapping';

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
