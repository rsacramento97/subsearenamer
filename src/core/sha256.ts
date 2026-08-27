export async function sha256Hex(data: ArrayBuffer): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', data);
  return [...new Uint8Array(digest)].map(b => b.toString(16).padStart(2, '0')).join('');
}

export function sameSize(expected: number, actual: number): boolean {
  return Number.isSafeInteger(expected) && Number.isSafeInteger(actual) && expected === actual;
}
