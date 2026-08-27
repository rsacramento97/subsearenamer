import type { NamingConfig } from './types';

function pad(value: number, digits: number) {
  return String(value).padStart(digits, '0');
}

/** Builds a deterministic target filename without touching file contents. */
export function buildTargetName(dateMs: number, extension: string, index: number, config: NamingConfig) {
  const shifted = new Date(dateMs + config.timezoneOffsetMinutes * 60_000);
  const yyyy = shifted.getUTCFullYear();
  const mm = pad(shifted.getUTCMonth() + 1, 2);
  const dd = pad(shifted.getUTCDate(), 2);
  const hh = pad(shifted.getUTCHours(), 2);
  const mi = pad(shifted.getUTCMinutes(), 2);
  const ss = pad(shifted.getUTCSeconds(), 2);
  const seq = pad(config.sequenceStart + index, config.sequenceDigits);
  const parts: string[] = [];
  if (config.prefix.trim()) parts.push(config.prefix.trim());
  if (config.includeDate) parts.push(`${yyyy}${mm}${dd}`);
  if (config.includeTime) parts.push(`${hh}${mi}${ss}`);
  if (config.rov.trim()) parts.push(config.rov.trim());
  if (config.camera.trim()) parts.push(config.camera.trim());
  if (config.sequenceDigits > 0) parts.push(seq);
  const base = parts.join(config.separator);
  return `${base || 'VIDEO'}${extension.startsWith('.') ? extension : `.${extension}`}`;
}
