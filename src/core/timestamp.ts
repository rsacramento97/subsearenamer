export type TimestampSource = 'filename' | 'file-modified' | 'metadata';

export type TimezoneConfig = {
  mode: 'select' | 'manual';
  offset: string;
};

const OFFSET = /^[+-](?:0\d|1\d|2[0-3]):[0-5]\d$/;

export function validateTimezone(config: TimezoneConfig): void {
  if (!OFFSET.test(config.offset)) {
    throw new Error('Fuso horário inválido. Use o formato +HH:MM ou -HH:MM.');
  }
}

export function applyOffset(date: Date, offset: string): Date {
  validateTimezone({ mode: 'manual', offset });
  const sign = offset[0] === '-' ? -1 : 1;
  const [hours, minutes] = offset.slice(1).split(':').map(Number);
  const target = sign * (hours * 60 + minutes);
  return new Date(date.getTime() + target * 60_000 + date.getTimezoneOffset() * 60_000);
}

export function formatTimestamp(date: Date, offset: string): string {
  const local = applyOffset(date, offset);
  const yyyy = local.getUTCFullYear();
  const mm = String(local.getUTCMonth() + 1).padStart(2, '0');
  const dd = String(local.getUTCDate()).padStart(2, '0');
  const hh = String(local.getUTCHours()).padStart(2, '0');
  const min = String(local.getUTCMinutes()).padStart(2, '0');
  const ss = String(local.getUTCSeconds()).padStart(2, '0');
  return `${yyyy}${mm}${dd}_${hh}${min}${ss}`;
}
