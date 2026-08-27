# SubSeaRenamer — Architecture Decisions

## ADR-001: conteúdo de vídeo nunca é regravado

A renomeação opera sobre o arquivo como bytes. Não há transcodificação, remux, edição ou alteração de metadados internos.

## ADR-002: core de integridade separado da interface

A lógica crítica de cópia/validação permanece no core Rust. A interface deve apenas montar o plano, solicitar a operação e apresentar o estado retornado pelo core.

## ADR-003: destino sempre novo

A estratégia padrão é criar cópias em `RENAMED_VIDEOS` ou em um destino escolhido. O original continua sendo a fonte imutável.

## ADR-004: falha conservadora

Qualquer condição inesperada interrompe a operação do item. O aplicativo prefere deixar um arquivo sem renomear a correr o risco de alterar ou perder o original.

## ADR-005: offline-first

A operação de arquivos e a nomenclatura devem funcionar sem internet. Serviços online não podem ser dependência para processar vídeos.
