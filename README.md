# SubSeaRenamer

Aplicativo profissional para organização e renomeação segura de vídeos de inspeção submarina com ROV.

## Objetivo

O SubSeaRenamer preserva integralmente os vídeos originais e cria cópias renomeadas em uma nova pasta. O Lovable será usado como preview/interface; o produto final será preparado para Windows instalado e Portable.

## Regra de segurança

**ORIGINAL → CÓPIA → VALIDAÇÃO → NOVO NOME → VERIFICAÇÃO**

Os arquivos originais nunca devem ser alterados, movidos, sobrescritos ou excluídos pelo aplicativo.

O aplicativo não deve reencodar, converter, editar ou regravar o conteúdo dos vídeos.

## Recursos planejados

- Pasta de destino `RENAMED_VIDEOS` criada automaticamente ou selecionada pelo usuário.
- Pré-visualização e modo simulação.
- Nomenclatura configurável para data, hora, ROV, câmera, projeto, operação, dive, sequência etc.
- Fuso horário selecionável e offset manual.
- Cópia segura com validação de tamanho.
- SHA-256 recomendado.
- Detecção de conflitos e nomes duplicados.
- Verificação de espaço disponível.
- Logs e relatórios.
- Recuperação após interrupção/desconexão.
- Cancelamento seguro.
- Desfazer sem tocar nos originais.
- Perfis de operação.
- Processamento em lote e drag-and-drop.
- Arquitetura preparada para Tauri/Electron e Windows Portable/instalador.
- Funcionamento offline na versão desktop.

## Desenvolvimento

O preview web não é o mecanismo final de manipulação de arquivos locais. A camada desktop deverá fornecer as operações nativas de arquivos e aplicar as proteções de integridade definidas em `PROJECT_SPEC.md`.
