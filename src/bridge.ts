export type RenameJobRequest = {
  sourceDir: string;
  destinationDir: string;
  timezone: string;
  manualOffsetMinutes?: number;
  verifyHash: boolean;
};

export type RenamePreview = {
  source: string;
  destination: string;
  status: 'ready' | 'conflict' | 'invalid';
  reason?: string;
};

export type RenameJobResult = {
  completed: number;
  failed: number;
  cancelled: boolean;
  errors?: string[];
};

export interface NativeRenamerBridge {
  preview(request: RenameJobRequest): Promise<RenamePreview[]>;
  execute(request: RenameJobRequest): Promise<RenameJobResult>;
  cancel(): Promise<void>;
}

export function getNativeRenamerBridge(): NativeRenamerBridge | null {
  const candidate = (globalThis as { __SUBSEA_RENAMER__?: NativeRenamerBridge }).__SUBSEA_RENAMER__;
  return candidate ?? null;
}

export function requireNativeRenamerBridge(): NativeRenamerBridge {
  const bridge = getNativeRenamerBridge();
  if (!bridge) throw new Error('Operação disponível somente no aplicativo desktop.');
  return bridge;
}
