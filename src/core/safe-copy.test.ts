import { describe, expect, it } from 'vitest';
import { validateCopyPlan, verifyCopy } from './safe-copy';

describe('safe copy contract', () => {
  it('rejects identical source and destination', () => {
    expect(() => validateCopyPlan({ source: 'A/video.mp4', destination: 'A/video.mp4', expectedSize: 1 })).toThrow();
  });
  it('rejects invalid expected sizes', () => {
    expect(() => validateCopyPlan({ source: 'A/a.mp4', destination: 'B/a.mp4', expectedSize: -1 })).toThrow();
  });
  it('requires exact size equality', () => {
    expect(verifyCopy(100, 100)).toBe(true);
    expect(verifyCopy(100, 99)).toBe(false);
  });
});
