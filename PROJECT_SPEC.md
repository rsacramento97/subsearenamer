# SubSeaRenamer

## Objetivo
Aplicativo profissional para organização e renomeação segura de vídeos de inspeção submarina com ROV. O preview web serve para validar a experiência; o produto final deve possuir camada desktop nativa para manipulação local de arquivos e funcionar offline no Windows.

## Regra absoluta de segurança
Os vídeos originais **nunca** devem ser alterados, movidos, sobrescritos ou excluídos.

Fluxo obrigatório:

**ORIGINAL → CÓPIA TEMPORÁRIA → FLUSH/SYNC → VALIDAÇÃO → FINALIZAÇÃO DO NOVO NOME → VERIFICAÇÃO**

O aplicativo não reencoda, converte, edita ou regrava o conteúdo do vídeo. A operação atua exclusivamente sobre cópias.

## Pasta de destino

- Padrão: `RENAMED_VIDEOS`.
- Usuário pode selecionar outro destino.
- Origem e destino devem estar em árvores diferentes.
- Nunca sobrescrever arquivos existentes.
- Conflito detectado durante a operação deve abortar aquele item sem destruir o original.

## Segurança de cópia

- Validar existência e tipo do arquivo de origem.
- Verificar espaço livre antes da operação.
- Criar arquivo temporário com nome exclusivo e criação sem overwrite.
- Copiar em streaming, sem carregar o vídeo inteiro na memória.
- Executar flush/sync antes da validação.
- Comparar tamanho da cópia com o tamanho da origem.
- SHA-256 opcional, recomendado para operação crítica.
- Finalizar o destino somente depois das validações.
- Fazer nova checagem de conflito imediatamente antes da finalização.
- Em qualquer erro, remover o parcial e manter o original intocado.

## Pré-visualização e simulação

A pré-visualização é obrigatória antes da execução. O modo **SIMULAÇÃO** calcula os nomes e detecta problemas sem modificar arquivos.

A tabela deve mostrar pelo menos: origem, nome atual, novo nome, tamanho, data/hora usada, fuso, status e motivo do bloqueio quando houver.

## Nomenclatura

Campos configuráveis: data, hora, ROV, câmera, projeto, operação, local, job, dive, sequence e número sequencial. Separador configurável e preservação da extensão original.

Caracteres inválidos do Windows devem ser sanitizados. Nomes vazios, reservados ou incompatíveis com o Windows devem bloquear o item. Colisões são tratadas sem overwrite.

## Data/hora e fuso

Permitir:

- seleção do fuso por lista;
- UTC−03:00;
- offset manual;
- fonte do timestamp claramente identificada;
- visualização da data/hora final na prévia;
- alteração de fuso sem qualquer alteração no conteúdo do vídeo.

## Interface operacional

Fluxo:

**ORIGEM → DESTINO → NOMENCLATURA → DATA/HORA/FUSO → PRÉ-VISUALIZAÇÃO → SIMULAÇÃO → CONFIRMAÇÃO EXPLÍCITA → EXECUÇÃO → VALIDAÇÃO → RELATÓRIO**

A execução deve apresentar progresso por arquivo e progresso geral, incluindo bytes copiados, velocidade quando disponível, erros e itens concluídos.

## Recuperação e cancelamento

- Journal registra o estado de cada item.
- Interrupção não transforma parcial em arquivo final.
- Desconexão de origem/destino aborta com segurança.
- Cancelamento impede novas cópias e permite finalizar/limpar o item em andamento com segurança.
- Retomada deve reconhecer itens já validados sem tocar nos originais.
- Desfazer remove somente cópias criadas pelo aplicativo e somente após confirmação explícita.

## Logs e relatórios

Registrar data/hora da operação, versão do aplicativo, origem, destino, nome antigo, nome novo, tamanho, resultado, erro, hash quando habilitado e fuso utilizado.

Relatórios: CSV, XLSX e PDF. Logs não devem ser gravados dentro da pasta de origem dos vídeos.

## Recursos

- Drag and drop.
- Processamento em lote.
- Ordenação por nome, data e ordem manual.
- Detecção de horários inconsistentes.
- Perfis de operação salvos.
- Funcionamento offline.
- Interface profissional para operação offshore.
- Portable e instalador Windows.

## Arquitetura

Separar frontend da camada nativa de arquivos. O motor de integridade deve ser independente da interface para que uma falha visual não comprometa a operação de arquivos.

O produto final deve usar uma camada desktop nativa, preferencialmente Tauri, mantendo o core Rust responsável pelas operações críticas.

## Testes críticos

Arquivos únicos e grandes lotes, nomes duplicados, espaço insuficiente, desconexão, interrupção, cancelamento, falha de cópia, hash divergente, caracteres especiais, nomes reservados do Windows, caminhos longos e alterações de fuso.

## Princípio fail-safe

**Em caso de dúvida ou erro, não realizar a operação.**

Nunca sobrescrever, apagar, mover ou alterar o original. Nunca considerar uma cópia concluída sem validação.

## Critério de release

Nenhuma versão é considerada pronta para uso operacional enquanto os testes críticos de integridade e o checklist em `docs/RELEASE_CHECKLIST.md` não forem atendidos.