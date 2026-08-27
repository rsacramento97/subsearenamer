export type CopyPlan = {
  source: string;
  destination: string;
  expectedSize: number;
};

export type CopyResult = {
  source: string;
  destination: string;
  bytesCopied: number;
  verified: boolean;
};

/**
 * Browser-safe contract for the desktop/native implementation.
 * The web preview must never claim that a local file was copied.
 * Tauri/Rust will implement the actual filesystem operation.
 */
export function validateCopyPlan(plan: CopyPlan): void {
  if (!plan.source.trim()) throw new Error('Origem inválida.');
  if (!plan.destination.trim()) throw new Error('Destino inválido.');
  if (plan.source === plan.destination) throw new Error('Origem e destino não podem ser iguais.');
  if (!Number.isSafeInteger(plan.expectedSize) || plan.expectedSize < 0) {
    throw new Error('Tamanho esperado inválido.');
  }
}

export function verifyCopy(expectedSize: number, actualSize: number): CopyResult['verified'] {
  return Number.isSafeInteger(expectedSize) && Number.isSafeInteger(actualSize) && expectedSize === actualSize;
}
