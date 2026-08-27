import { describe, expect, it } from 'vitest';
import { buildPlan, sanitizeFileName, isSafeDestination } from './operation-plan';

describe('operation plan safety', () => {
  it('sanitizes Windows-invalid filename characters', () => {
    expect(sanitizeFileName('ROV:CAM?01.mp4')).toBe('ROV_CAM_01.mp4');
  });
  it('rejects an empty or identical destination', () => {
    expect(isSafeDestination('C:/Videos', 'C:/Videos')).toBe(false);
    expect(isSafeDestination('C:/Videos', '')).toBe(false);
  });
  it('marks duplicate planned names as conflicts', () => {
    const plan = buildPlan([
      { path: 'a.mp4', name: 'a.mp4', size: 10, extension: '.mp4' },
      { path: 'b.mp4', name: 'b.mp4', size: 20, extension: '.mp4' },
    ], 'C:/RENAMED_VIDEOS', () => 'same.mp4');
    expect(plan[0].status).toBe('ready');
    expect(plan[1].status).toBe('conflict');
  });
});
