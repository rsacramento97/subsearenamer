# SubSeaRenamer — Status de desenvolvimento

## Concluído

- Core Rust de cópia segura.
- Verificação de espaço disponível.
- Bloqueio de origem/destino sobrepostos.
- Arquivos temporários com criação exclusiva.
- Proteção contra overwrite.
- Limpeza de parciais em falha.
- Validação de tamanho.
- SHA-256 opcional.
- Testes de segurança do core.
- Core e frontend separados conceitualmente.
- Planejamento de nomenclatura, operação, timestamp e fuso.
- Especificação fail-safe reforçada.
- Checklist de release criado.

## Em desenvolvimento

- Integração da interface com a camada nativa de arquivos.
- Empacotamento Tauri/Windows.
- Portable e instalador.
- Journal de retomada integrado ao fluxo real.
- Relatórios finais.
- Testes de carga no Windows.

## Critério de conclusão

O projeto somente será marcado como **PRONTO PARA PRODUÇÃO** depois que a aplicação Windows for testada com cópias reais, incluindo arquivos grandes e interrupções, e todos os itens críticos do `docs/RELEASE_CHECKLIST.md` forem aprovados.

Até esse ponto, o software deve ser tratado como versão de desenvolvimento e nunca como substituto de backup.
