# SubSeaRenamer — Release Checklist

Este checklist é obrigatório antes de qualquer versão ser considerada pronta para uso operacional.

## 1. Integridade dos arquivos

- [ ] Original permanece byte-for-byte idêntico após a operação.
- [ ] Nenhuma operação usa `rename`, `move`, `delete` ou overwrite sobre o original.
- [ ] Destino nunca sobrescreve arquivo existente.
- [ ] Cópia é feita para arquivo temporário exclusivo.
- [ ] `flush/sync` ocorre antes da validação final.
- [ ] Tamanho da cópia é comparado ao original.
- [ ] SHA-256 é validado quando habilitado.
- [ ] Arquivo parcial é removido em qualquer falha.
- [ ] Destino só é finalizado depois de todas as validações.

## 2. Segurança de origem/destino

- [ ] Origem e destino não podem ser o mesmo caminho.
- [ ] Destino dentro da árvore de origem é rejeitado.
- [ ] Origem dentro da árvore de destino é rejeitada.
- [ ] Espaço livre é verificado antes da cópia.
- [ ] Existe segunda verificação de conflito imediatamente antes da finalização.

## 3. Planejamento

- [ ] Pré-visualização é obrigatória.
- [ ] Modo simulação não altera nenhum arquivo.
- [ ] Conflitos de nomes bloqueiam a execução até decisão explícita.
- [ ] Extensão original é preservada.
- [ ] Caracteres inválidos do Windows são sanitizados.
- [ ] Nomes vazios ou inválidos não podem entrar no lote.
- [ ] Sequência e ordenação são determinísticas.

## 4. Data, hora e fuso

- [ ] Fuso é exibido claramente na pré-visualização.
- [ ] Lista de fusos inclui UTC−03:00.
- [ ] Offset manual é aceito.
- [ ] Timestamp utilizado é identificável (metadado, nome ou entrada manual).
- [ ] Mudança de fuso altera somente o nome planejado, nunca o conteúdo do vídeo.

## 5. Recuperação

- [ ] Interrupção durante cópia não produz destino final falso.
- [ ] Desconexão do disco não altera o original.
- [ ] Cancelamento deixa o original intacto.
- [ ] Arquivos temporários abandonados podem ser identificados e limpos com segurança.
- [ ] Journal/log permite identificar o último estado conhecido.

## 6. Testes de carga

- [ ] Vídeo pequeno.
- [ ] Vídeo de vários GB.
- [ ] Lote com centenas de arquivos.
- [ ] Lote com nomes duplicados.
- [ ] Caracteres acentuados e Unicode.
- [ ] Caminho longo do Windows.
- [ ] Espaço insuficiente.
- [ ] Destino removido durante a operação.
- [ ] Arquivo de origem removido durante o planejamento.
- [ ] Falha de leitura.
- [ ] Falha de escrita.
- [ ] Hash divergente.

## 7. Windows

- [ ] Build Release do core.
- [ ] Build do frontend.
- [ ] Aplicação Portable testada em máquina limpa.
- [ ] Instalador testado em máquina limpa.
- [ ] Desinstalação não remove vídeos do usuário.
- [ ] Aplicação funciona offline.
- [ ] Logs são gravados em local seguro e não dentro da pasta dos vídeos de origem.

## 8. Critério de aprovação

Uma versão **não pode ser liberada** se qualquer item crítico de integridade estiver falhando. Em caso de dúvida, o comportamento correto é abortar a operação e preservar o original.
