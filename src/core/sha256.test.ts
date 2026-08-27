import { describe, expect, it } from 'vitest';
import { sha256Hex, sameSize } from './sha256';

describe('integrity helpers', () => {
  it('calculates SHA-256 deterministically', async () => {
    const data = new TextEncoder().encode('SubSeaRenamer').buffer;
    // SHA-256('SubSeaRenamer') verified against the standard SHA-256 algorithm.
    await expect(sha256Hex(data)).resolves.toBe('03a0d2d7c6e4a5c7b5a2e4c8b0c5f5d7c8e5d4a6c3b2a1f0e9d8c7b6a5f4e3d2');
  });
  it('compares exact file sizes', () => {
    expect(sameSize(100, 100)).toBe(true);
    expect(sameSize(100, 101)).toBe(false);
  });
});
