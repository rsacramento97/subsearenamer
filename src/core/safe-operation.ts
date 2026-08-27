export type SafeOperationResult = {
  source: string;
  destination: string;
  sourceSize: number;
  copiedSize: number;
  verified: boolean;
  sha256?: string;
};

/**
 * Contract for the desktop/native implementation.
 * The browser preview must never pretend to perform local file writes.
 */
export interface SafeFileOperator {
  copyAndVerify(source: string, destination: string, calculateSha256: boolean): Promise<SafeOperationResult>;
  rollback(operationId: string): Promise<void>;
}

export function assertSafeDestination(source: string, destination: string): void {
  const normalize = (value: string) => value.replace(/[\\/]+$/, '').toLowerCase();
  if (normalize(source) === normalize(destination)) {
    throw new Error('A pasta de origem e a pasta de destino não podem ser a mesma.');
  }
}

export function assertDestinationIsNew(source: string, destination: string): void {
  assertSafeDestination(source, destination);
  if (!destination.trim()) {
    throw new Error('A pasta de destino é obrigatória.');
  }
}
