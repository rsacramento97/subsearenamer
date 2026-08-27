# Segurança para o operador

Antes de executar um lote:

1. Confirme a pasta de origem.
2. Confirme a pasta `RENAMED_VIDEOS` ou outro destino.
3. Revise a pré-visualização dos nomes.
4. Resolva todos os conflitos.
5. Confirme o fuso horário.
6. Rode a simulação.
7. Confirme a execução.

Durante a execução, o operador deve poder ver o arquivo atual, estado, progresso e eventual erro.

Depois da execução, o relatório deve informar claramente quais arquivos foram copiados e validados.

O aplicativo não deve prometer que uma cópia é um backup independente; a preservação do original é uma proteção contra a operação de renomeação, não substituto de uma política de backup.
