export type OperationStatus = 'planned' | 'copying' | 'verifying' | 'completed' | 'failed' | 'rolled_back';

export type OperationRecord = {
  id: string;
  source: string;
  destination: string;
  originalName: string;
  newName: string;
  status: OperationStatus;
  sourceSize?: number;
  copiedSize?: number;
  sha256?: string;
  error?: string;
  createdAt: string;
  updatedAt: string;
};

/** Persistent journal contract for the desktop implementation.
 * A record is written before touching a destination and updated after every
 * irreversible step, allowing interrupted batches to be inspected/recovered.
 */
export interface OperationJournal {
  append(record: OperationRecord): Promise<void>;
  update(id: string, patch: Partial<OperationRecord>): Promise<void>;
  listPending(): Promise<OperationRecord[]>;
  get(id: string): Promise<OperationRecord | null>;
}

export function createOperationId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
}
