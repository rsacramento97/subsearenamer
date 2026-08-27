import { describe, expect, it } from 'vitest';
import { sha256Hex, sameSize } from './sha256';

describe('integrity helpers', () => {
  it('calculates SHA-256 deterministically', async () => {
    const data = new TextEncoder().encode('SubSeaRenamer').buffer;
    await expect(sha256Hex(data)).resolves.toBe('26c72ac5b16497efaeff7bf3f97ce4df44c069575984f20f97ba15033254751b');
  });
  it('compares exact file sizes', () => {
    expect(sameSize(100, 100)).toBe(true);
    expect(sameSize(100, 101)).toBe(false);
  });
});
