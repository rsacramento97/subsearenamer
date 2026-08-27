import { describe, expect, it } from 'vitest';
import { sha256Hex, sameSize } from './sha256';

describe('integrity helpers', () => {
  it('calculates SHA-256 deterministically', async () => {
    const data = new TextEncoder().encode('SubSeaRenamer').buffer;
    await expect(sha256Hex(data)).resolves.toBe('a0a0c4f7e6c5a5d4d8f7a4f7c5e5c3a5d1e9f0e6c5a9c8a6f7a4e3c2b1d0f9e8');
  });
  it('compares exact file sizes', () => {
    expect(sameSize(100, 100)).toBe(true);
    expect(sameSize(100, 101)).toBe(false);
  });
});
