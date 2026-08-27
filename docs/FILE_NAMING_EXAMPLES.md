# SubSeaRenamer — Exemplos de nomenclatura

A extensão original deve sempre ser preservada.

## Exemplo operacional

Entrada:

`D:\\ROV\\RAW\\VID_004821.MP4`

Configuração:

- Data: ligada
- Hora: ligada
- ROV: `ROV01`
- Câmera: `CAM01`
- Sequência: ligada
- Separador: `_`
- Fuso: `UTC-03:00`

Saída planejada:

`RENAMED_VIDEOS\\20260827_123422_ROV01_CAM01_001.MP4`

## Regras

- Não alterar `.MP4` para outra extensão.
- Não alterar o conteúdo do vídeo.
- Não usar caracteres reservados do Windows.
- Não permitir nome vazio.
- Não sobrescrever outro arquivo.
- Se dois itens produzirem o mesmo nome, o lote deve ser marcado como conflito antes da execução.

## Importante

O nome é apenas metadado do arquivo. O timestamp usado na nomenclatura não deve modificar o timestamp ou qualquer outro metadado interno do vídeo.
