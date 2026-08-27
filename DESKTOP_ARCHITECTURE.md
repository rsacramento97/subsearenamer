# Arquitetura Desktop — SubSeaRenamer

## Princípio
O preview web nunca executa operações destrutivas nos arquivos do usuário. A versão Windows terá um backend nativo responsável pelo filesystem.

## Stack
- Frontend: React + TypeScript + Vite.
- Desktop: Tauri 2.
- Backend: Rust.
- Hash: SHA-256 incremental em stream.
- Persistência: journal local por operação.

## Pipeline obrigatório
1. Validar origem e destino.
2. Enumerar arquivos e construir plano completo.
3. Exibir prévia e conflitos.
4. Criar diretório `RENAMED_VIDEOS` fora da origem.
5. Para cada arquivo, copiar para nome temporário exclusivo.
6. Fazer flush/sync do arquivo.
7. Comparar tamanho.
8. Calcular/verificar SHA-256 da cópia.
9. Renomear atomicamente o temporário para o nome final, somente se o destino não existir.
10. Registrar sucesso no journal.
11. Em qualquer falha, remover apenas temporários criados e manter o original intacto.

## Regras de segurança
- Nunca usar `rename` no arquivo original como parte da operação normal.
- Nunca sobrescrever um destino existente.
- Nunca apagar o original.
- Não permitir destino dentro da árvore de origem nem origem dentro da árvore de destino.
- Não iniciar execução se houver conflitos não resolvidos.
- Não considerar cópia concluída sem validação.
- Cancelamento deve preservar arquivos originais e permitir retomada limpa.

## Portable e instalador
A pipeline de release deverá gerar dois artefatos Windows: instalador e versão portable. Antes de publicar, os binários devem passar pelos testes automatizados e por uma matriz de testes com arquivos pequenos, grandes, caracteres especiais, nomes duplicados, espaço insuficiente, interrupção e destino desconectado.
