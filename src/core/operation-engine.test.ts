import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { copyVerified } from './operation-engine';

describe('copyVerified', () => {
  let root: string;
  let sourceDir: string;
  let destinationDir: string;

  beforeEach(async () => {
    root = await mkdtemp(join(tmpdir(), 'subsearenamer-test-'));
    sourceDir = join(root, 'source');
    destinationDir = join(root, 'destination');
  });

  afterEach(async () => {
    await rm(root, { recursive: true, force: true });
  });

  it('copies, hashes and preserves the original', async () => {
    await writeFile(join(root, 'setup'), 'x');
    await rm(join(root, 'setup'));
    await import('node:fs/promises').then(fs => fs.mkdir(sourceDir, { recursive: true }));
    await writeFile(join(sourceDir, 'video.mp4'), Buffer.from('critical-video-data'));

    const result = await copyVerified(join(sourceDir, 'video.mp4'), destinationDir, 'renamed.mp4');
    expect(await readFile(join(sourceDir, 'video.mp4'))).toEqual(Buffer.from('critical-video-data'));
    expect(await readFile(result.destination)).toEqual(Buffer.from('critical-video-data'));
    expect(result.bytes).toBe(Buffer.byteLength('critical-video-data'));
    expect(result.sha256).toMatch(/^[a-f0-9]{64}$/);
  });

  it('never overwrites an existing destination', async () => {
    await import('node:fs/promises').then(fs => fs.mkdir(sourceDir, { recursive: true }));
    await import('node:fs/promises').then(fs => fs.mkdir(destinationDir, { recursive: true }));
    await writeFile(join(sourceDir, 'video.mp4'), 'original');
    await writeFile(join(destinationDir, 'renamed.mp4'), 'do-not-touch');

    await expect(copyVerified(join(sourceDir, 'video.mp4'), destinationDir, 'renamed.mp4')).rejects.toThrow('não será sobrescrito');
    expect(await readFile(join(destinationDir, 'renamed.mp4'), 'utf8')).toBe('do-not-touch');
  });

  it('does not finalize when verification is disabled until copy is complete', async () => {
    await import('node:fs/promises').then(fs => fs.mkdir(sourceDir, { recursive: true }));
    await writeFile(join(sourceDir, 'video.mp4'), 'data');
    const result = await copyVerified(join(sourceDir, 'video.mp4'), destinationDir, 'renamed.mp4', false);
    expect(await readFile(result.destination, 'utf8')).toBe('data');
  });
});
