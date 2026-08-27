export type VideoItem = {
  id: string;
  originalName: string;
  relativePath: string;
  sizeBytes: number;
  lastModified: number;
  extension: string;
  targetName: string;
  status: 'OK' | 'ALERTA' | 'CONFLITO' | 'ERRO';
};

export type NamingConfig = {
  rov: string;
  camera: string;
  prefix: string;
  separator: string;
  includeDate: boolean;
  includeTime: boolean;
  sequenceStart: number;
  sequenceDigits: number;
  timezoneOffsetMinutes: number;
};

export const VIDEO_EXTENSIONS = ['mp4','mov','avi','mkv','mts','m2ts','mxf','mpg','mpeg','wmv'];
