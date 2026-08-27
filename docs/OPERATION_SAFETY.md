# SubSeaRenamer — Safety Contract

Este documento define o comportamento que a implementação deve respeitar independentemente da interface.

## Invariantes

1. O arquivo de origem é somente leitura durante toda a operação.
2. Nenhum caminho de destino existente é sobrescrito.
3. Um arquivo temporário nunca é reutilizado silenciosamente.
4. Um parcial nunca é apresentado como concluído.
5. A finalização ocorre somente após validação.
6. Erro significa parada segura, não tentativa destrutiva.

## Máquina de estados

`PLANNED → COPYING → SYNCED → VALIDATED → FINALIZING → COMPLETED`

Estados de falha:

`COPYING → FAILED_CLEANUP`

`SYNCED → FAILED_CLEANUP`

`VALIDATED → FAILED_CLEANUP`

`FINALIZING → FAILED_CLEANUP`

`FAILED_CLEANUP → FAILED`

Nunca avançar para `COMPLETED` sem passar por `VALIDATED`.

## O que o aplicativo não pode fazer

- apagar a origem;
- mover a origem;
- renomear a origem;
- abrir a origem para escrita;
- substituir destino existente;
- apagar automaticamente um destino que já exista;
- considerar sucesso apenas porque `copy` retornou sem erro;
- alterar o timestamp ou conteúdo do vídeo como parte da renomeação.

## Segurança operacional

A interface deve exigir confirmação explícita imediatamente antes da execução e deixar claro que os originais permanecerão onde estão.

Se a aplicação perder acesso ao disco, detectar conflito, ficar sem espaço ou falhar na validação, o item deve permanecer como não concluído.
