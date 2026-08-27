export type VideoInput = { path: string; name: string; size: number; extension: string; timestampMs?: number };
export type PlannedOperation = { source: string; destination: string; sourceName: string; destinationName: string; size: number; status: 'ready' | 'conflict' | 'invalid' };

const INVALID_CHARS = /[<>:"/\\|?*\x00-\x1F]/g;

export function sanitizeFileName(name: string): string {
  return name.replace(INVALID_CHARS, '_').replace(/[. ]+$/g, '').trim();
}

export function isSafeDestination(sourceDir: string, destinationDir: string): boolean {
  return Boolean(sourceDir && destinationDir) && sourceDir !== destinationDir;
}

export function buildPlan(inputs: VideoInput[], destinationDir: string, nameFor: (input: VideoInput, index: number) => string, existingNames: string[] = []): PlannedOperation[] {
  const occupied = new Set(existingNames.map(n => n.toLowerCase()));
  return inputs.map((input, index) => {
    const destinationName = sanitizeFileName(nameFor(input, index));
    const conflict = !destinationName || occupied.has(destinationName.toLowerCase());
    if (!conflict) occupied.add(destinationName.toLowerCase());
    return {
      source: input.path,
      destination: `${destinationDir.replace(/[\\/]$/, '')}/${destinationName}`,
      sourceName: input.name,
      destinationName,
      size: input.size,
      status: conflict ? 'conflict' : 'ready',
    };
  });
}
