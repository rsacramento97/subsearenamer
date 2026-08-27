import { describe, expect, it } from 'vitest';
import { buildVideoName, validateUniqueNames } from './naming';

describe('video naming', () => {
  it('builds a deterministic name and preserves extension', () => {
    expect(buildVideoName({ date: '20260827', time: '123422', timezone: '-03:00', rov: 'ROV01', camera: 'CAM01', sequence: 1, separator: '_' }, 'MP4'))
      .toBe('20260827_123422_ROV01_CAM01_001.MP4');
  });

  it('detects duplicate destination names case-insensitively', () => {
    const result = validateUniqueNames([
      { originalName: 'A.MP4', newName: 'X.MP4', status: 'ok' },
      { originalName: 'B.MP4', newName: 'x.mp4', status: 'ok' },
    ]);
    expect(result[1].status).toBe('conflict');
  });
});
