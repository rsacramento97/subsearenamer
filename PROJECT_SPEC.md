# SubSeaRenamer

## Especificação inicial
Aplicativo profissional para organização e renomeação segura de vídeos de inspeção submarina com ROV. O Lovable será usado como preview/interface; o produto final será preparado para Windows instalado e Portable.

## Regra absoluta de segurança
Os vídeos originais nunca devem ser alterados, movidos, sobrescritos ou excluídos. Fluxo obrigatório: ORIGINAL → CÓPIA → VALIDAÇÃO → NOVO NOME → VERIFICAÇÃO.

O aplicativo não deve reencodar, converter, editar ou regravar o conteúdo dos vídeos. A operação final atua sobre cópias.

## Pasta de destino
Criar uma nova pasta para os vídeos renomeados, por padrão `RENAMED_VIDEOS`. Também permitir escolher outra pasta de destino, desde que seja diferente da origem. Nunca sobrescrever arquivos existentes.

## Segurança
- Pré-visualização obrigatória.
- Modo SIMULAÇÃO sem alteração de arquivos.
- Verificação de tamanho após cópia.
- SHA-256 opcional e recomendado.
- Conflitos de nomes exigem decisão explícita.
- Verificação de espaço antes da operação.
- Logs completos e exportáveis.
- Recuperação após interrupção/desconexão.
- Cancelamento seguro.
- Desfazer atua somente sobre as cópias.

## Nomenclatura
Campos configuráveis: data, hora, ROV, câmera, projeto, operação, local, job, dive, sequence e número sequencial. Separadores configuráveis e preservação da extensão original.

## Data/hora e fuso
Permitir seleção do fuso por lista e ajuste manual do offset, incluindo UTC−03:00. Permitir definir a fonte do timestamp e exibir claramente o fuso utilizado.

## Interface
Fluxo: origem → destino → nomenclatura/data/hora/fuso → pré-visualização → simulação → confirmação → cópia/validação/renomeação → relatório.

## Recursos
- Drag and drop.
- Processamento em lote.
- Ordenação por nome/data/ordem manual.
- Detecção de horários inconsistentes.
- Perfis de operação salvos.
- Relatórios CSV/XLSX/PDF.
- Produto desktop offline.
- Interface profissional para operação offshore.

## Arquitetura futura
Separar frontend/preview do motor local de arquivos. A versão desktop deverá usar camada nativa (Tauri ou Electron) para acesso seguro ao sistema de arquivos. O Lovable é preview/interface, não o mecanismo final de manipulação de arquivos locais.

## Testes críticos
Arquivos únicos e grandes lotes, nomes duplicados, espaço insuficiente, desconexão, interrupção, cancelamento, falha de cópia, hash divergente, caracteres especiais, nomes longos e alterações de fuso.

## Princípio fail-safe
Em caso de dúvida ou erro, não realizar a operação. Nunca sobrescrever, apagar, mover ou alterar o original; nunca considerar uma cópia concluída sem validação.