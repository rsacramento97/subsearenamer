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

export async function copyVerified(source: string, destinationDir: string, destinationName: string, verifyHash = true): Promise<CopyResult> {
  const src = resolve(source);
  const dir = resolve(destinationDir);
  const dest = join(dir, destinationName);
  if (!src || !dir || src === dest) throw new Error('Operação inválida: origem e destino não podem ser iguais.');
  if (isInside(src, dir) || isInside(dir, src)) throw new Error('Origem e destino não podem estar um dentro do outro.');

  const sourceStat = await stat(src);
  if (!sourceStat.isFile()) throw new Error('A origem precisa ser um arquivo.');
  await mkdir(dir, { recursive: true });
  const temp = join(dir, `.${destinationName}.subsearenamer-${Date.now()}.tmp`);

  const hash = createHash('sha256');
  let bytes = 0;
  try {
    await new Promise<void>((resolvePromise, reject) => {
      const input = createReadStream(src);
      const output = createWriteStream(temp, { flags: 'wx' });
      input.on('data', chunk => { hash.update(chunk); bytes += chunk.length; });
      input.on('error', reject);
      output.on('error', reject);
      output.on('finish', resolvePromise);
      input.pipe(output);
    });
    if (bytes !== sourceStat.size) throw new Error(`Falha de integridade: tamanho esperado ${sourceStat.size}, obtido ${bytes}.`);
    const digest = hash.digest('hex');
    if (!verifyHash) return { source: src, destination: dest, bytes, sha256: digest };
    const destinationStat = await stat(temp);
    if (destinationStat.size !== sourceStat.size) throw new Error('Falha de integridade no arquivo temporário.');
    await rename(temp, dest);
    return { source: src, destination: dest, bytes, sha256: digest };
  } catch (error) {
    await unlink(temp).catch(() => undefined);
    throw error;
  }
}
