import { createHash } from 'node:crypto';
import { createReadStream, createWriteStream } from 'node:fs';
import { mkdir, rename, stat, unlink } from 'node:fs/promises';
import { dirname, join, resolve, sep } from 'node:path';

export type CopyResult = {
  source: string;
  destination: string;
  bytes: number;
  sha256: string;
};

function isInside(parent: string, child: string): boolean {
  const p = resolve(parent) + sep;
  return resolve(child).startsWith(p);
}

function destinationExists(path: string): Promise<boolean> {
  return stat(path).then(() => true).catch(() => false);
}

async function sha256File(path: string): Promise<string> {
  const hash = createHash('sha256');
  await new Promise<void>((resolvePromise, reject) => {
    const input = createReadStream(path);
    input.on('data', chunk => hash.update(chunk));
    input.on('error', reject);
    input.on('end', () => resolvePromise());
  });
  return hash.digest('hex');
}

/**
 * Fail-safe copy: the source is read-only, the destination is created as a
 * unique temporary file, the temporary file is size/hash verified, and only
 * then atomically renamed to the requested final name. Existing destinations
 * are never overwritten.
 */
export async function copyVerified(
  source: string,
  destinationDir: string,
  destinationName: string,
  verifyHash = true,
): Promise<CopyResult> {
  const src = resolve(source);
  const dir = resolve(destinationDir);
  const dest = join(dir, destinationName);

  if (src === dest) throw new Error('Operação inválida: origem e destino não podem ser iguais.');
  if (isInside(src, dir) || isInside(dir, src)) {
    throw new Error('Origem e destino não podem estar um dentro do outro.');
  }

  const sourceStat = await stat(src);
  if (!sourceStat.isFile()) throw new Error('A origem precisa ser um arquivo.');
  await mkdir(dir, { recursive: true });

  if (await destinationExists(dest)) {
    throw new Error(`Destino já existe e não será sobrescrito: ${dest}`);
  }

  const temp = join(
    dir,
    `.${destinationName}.subsearenamer-${process.pid}-${Date.now()}-${Math.random().toString(36).slice(2)}.tmp`,
  );

  const sourceHash = createHash('sha256');
  let bytes = 0;

  try {
    await new Promise<void>((resolvePromise, reject) => {
      const input = createReadStream(src);
      const output = createWriteStream(temp, { flags: 'wx' });
      let settled = false;
      const fail = (error: Error) => {
        if (settled) return;
        settled = true;
        input.destroy();
        output.destroy();
        reject(error);
      };

      input.on('data', chunk => {
        sourceHash.update(chunk);
        bytes += chunk.length;
      });
      input.on('error', fail);
      output.on('error', fail);
      output.on('finish', () => {
        if (!settled) {
          settled = true;
          resolvePromise();
        }
      });
      input.pipe(output);
    });

    if (bytes !== sourceStat.size) {
      throw new Error(`Falha de integridade: tamanho esperado ${sourceStat.size}, obtido ${bytes}.`);
    }

    const sourceDigest = sourceHash.digest('hex');
    const tempStat = await stat(temp);
    if (tempStat.size !== sourceStat.size) {
      throw new Error('Falha de integridade no arquivo temporário.');
    }

    if (verifyHash) {
      const tempDigest = await sha256File(temp);
      if (tempDigest !== sourceDigest) {
        throw new Error('Falha de integridade: SHA-256 da cópia não corresponde ao original.');
      }
    }

    // Re-check immediately before finalization. Never overwrite a file that
    // appeared after the first collision check.
    if (await destinationExists(dest)) {
      throw new Error(`Destino passou a existir durante a operação e não será sobrescrito: ${dest}`);
    }

    await rename(temp, dest);
    return { source: src, destination: dest, bytes, sha256: sourceDigest };
  } catch (error) {
    await unlink(temp).catch(() => undefined);
    throw error;
  }
}
