import type { VideoItem } from './types';

export function findDuplicateTargets(items: VideoItem[]) {
  const counts = new Map<string, number>();
  for (const item of items) counts.set(item.targetName.toLowerCase(), (counts.get(item.targetName.toLowerCase()) ?? 0) + 1);
  return new Set([...counts.entries()].filter(([, count]) => count > 1).map(([name]) => name));
}

export function validateTargets(items: VideoItem[]) {
  const duplicates = findDuplicateTargets(items);
  return items.map(item => duplicates.has(item.targetName.toLowerCase()) ? { ...item, status: 'CONFLITO' as const } : { ...item, status: 'OK' as const });
}

export function formatBytes(bytes: number) {
  if (!Number.isFinite(bytes)) return '0 B';
  const units = ['B','KB','MB','GB','TB'];
  let value = bytes, unit = 0;
  while (value >= 1024 && unit < units.length - 1) { value /= 1024; unit++; }
  return `${value.toFixed(unit ? 1 : 0)} ${units[unit]}`;
}
