# Relatório de auditoria assistida — Bloco de notas v0.2.5

> **Natureza do documento.** Este é um relatório de apoio à revisão humana. Ele separa extração, avaliação e decisão. Não constitui opinião profissional, certificação de conformidade, parecer jurídico, declaração de fraude ou aprovação final.

## BLOCO_EXECUTADO, STATUS, CONTAGEM E RISCOS_DE_SEGURANÇA

**BLOCO_EXECUTADO:** B00–B16, adaptados ao escopo de revisão de um aplicativo Rust e de seu artefato Windows. O bloco opcional R02 não foi executado, porque não houve pesquisa de novos prompts; o objeto foi auditar o projeto existente.

**STATUS:** `APTO_PARA_REVISÃO_HUMANA`. Não há `DEC-ID` de aprovação humana final neste relatório.

**CONTAGEM:** o baseline tem 30 arquivos rastreados, incluindo 10 arquivos de código/teste e 12 arquivos Markdown. A validação executou 12 testes unitários, 10 casos de integração, Clippy sem warnings, advisory scan, builds Linux e Windows, inspeção de recursos PE e dois smoke tests com timeout controlado.

**RISCOS_DE_SEGURANÇA:** não foram encontradas vulnerabilidades reportadas no `cargo audit --no-fetch`. Permanecem dois avisos transitivos de manutenção: `paste 1.0.15` e `ttf-parser 0.25.1` [7] [8]. O único `unsafe` do código de produção está isolado na chamada Windows `ReplaceFileW`. A execução manual em Windows 10/11 real não foi realizada neste ambiente e permanece como gate material.

## 1. CFG-001 — configuração, objetivo e responsabilidades

| Campo | Valor | Origem | Estado | Impacto se ausente |
|---|---|---|---|---|
| Entidade/processo | Projeto Bloco de notas | Repositório e solicitação do usuário | EVIDENCIADO | Baixo |
| Tipo de trabalho | Revisão técnica de software, build e artefato Windows | Escopo desta auditoria | EVIDENCIADO | Médio |
| Finalidade da IA | Organizar evidências, executar verificações reproduzíveis e redigir um rascunho auditável | Prompt e instruções do projeto | EVIDENCIADO | Médio |
| Objetivo | Avaliar se o v0.2.5 implementa o fluxo de bloco de notas e se o executável Windows contém recursos e controles declarados | README, playbook e código | EVIDENCIADO | Alto |
| Escopo incluído | `src/`, `tests/`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `assets/`, documentação, dependências, build e artefato PE | Inventário Git | EVIDENCIADO | Alto |
| Escopo excluído | Teste manual em máquina Windows 10/11 real, SmartScreen/Defender reais, drivers gráficos de hardware e acessibilidade presencial | Limitação do ambiente | NÃO_TESTADO | Alto |
| Período | Baseline do commit `5eca5614e2e99871a3573a36e419427e3b562094`, tag `v0.2.5` | Git | EVIDENCIADO | Médio |
| População/universo | 30 arquivos rastreados; fontes de código, testes, configuração, documentação e assets | `git ls-files` | EVIDENCIADO | Médio |
| Jurisdição/plataforma | Windows 10/11 como alvo declarado; Linux como host de auditoria | README, manifesto e build | EVIDENCIADO | Alto |
| Critérios | Requisitos do projeto, `DEVELOPMENT_PLAYBOOK.md`, README, testes, Cargo, comportamento observável e gates do prompt B00–B16 | Arquivos do repositório e prompt | EVIDENCIADO | Alto |
| Responsável pelo trabalho | Usuário/mantenedor do repositório | Não informado nominalmente | NÃO_INFORMADO | Médio |
| Preparador | Assistente de revisão | Execução desta tarefa | EVIDENCIADO | Baixo |
| Revisor/aprovador | Pessoa humana mantenedora | Ainda não registrado | NÃO_FORNECIDO | Alto |
| Especialista | Especialista Windows/UX/acessibilidade pode ser necessário para o gate real | Limitação técnica | NÃO_INFORMADO | Alto |

A classificação de impacto é **média**: o software é um utilitário local, mas uma falha de abertura ou salvamento pode causar perda de produtividade ou de dados do usuário. Nenhuma ação externa ou irreversível foi realizada durante a auditoria.

### Uso autorizado da IA e RACI

| Atividade | Permitida à IA | Exige revisão humana | Proibida à IA | Responsável pela decisão |
|---|---|---|---|---|
| Inventariar arquivos e extrair fatos | Sim | Sim | Inventar fatos ausentes | Mantenedor |
| Executar testes locais e builds | Sim | Sim | Declarar compatibilidade real sem teste Windows | Mantenedor |
| Comparar implementação com critérios | Sim, como avaliação provisória | Sim | Emitir certificação final | Revisor humano |
| Redigir achados e recomendações | Sim, como rascunho | Sim | Encerrar risco sem evidência | Responsável do trabalho |
| Publicar código ou release | Somente com autorização explícita | Sim | Publicar sem gate humano | Mantenedor |

## 2. Inventário de fontes e integridade

| SRC-ID | Fonte | Conteúdo usado | Integridade/autoridade | Estado |
|---|---|---|---|---|
| SRC-001 | Repositório Git no commit `5eca561` | Estado, tag e histórico do baseline | Hash Git, fonte primária do código | EVIDENCIADO |
| SRC-002 | `Cargo.toml` e `Cargo.lock` | Versões, dependências, build script e resolução | Fonte primária de configuração | EVIDENCIADO |
| SRC-003 | `src/` | Implementação do aplicativo | Código primário; revisão estática | EVIDENCIADO |
| SRC-004 | `tests/` e testes unitários | Regressões de documento, Unicode, I/O e UTF-8 | Execução reproduzível | EVIDENCIADO |
| SRC-005 | `build.rs`, `assets/app.manifest`, `assets/app.ico` | Recursos Windows e DPI | Fonte primária; conferida no PE | EVIDENCIADO |
| SRC-006 | Executável PE v0.2.5 | Importações, seção `.rsrc`, ícone, manifesto e versão | Artefato gerado; inspeção local | EVIDENCIADO |
| SRC-007 | Logs de `cargo`, `cargo audit`, Xvfb e Wine | Resultados dos procedimentos | Saída local datada nesta execução | EVIDENCIADO |
| SRC-008 | `DEVELOPMENT_PLAYBOOK.md` e README | Critérios de projeto e limitações declaradas | Documentação interna; autoridade de escopo | EVIDENCIADO |
| SRC-009 | `pasted_content_13.txt` | Método B00–B16 e vocabulário de estados | Instrução fornecida pelo usuário; não é evidência do produto | EVIDENCIADO |
| SRC-010 | Relato humano de que o v0.2.4 não abriu ao clicar | Incidente de uso real | Observação direta, sem log Windows anexado | INCONCLUSIVO |

Não foram recebidos logs de eventos do Windows, dump de processo, mensagem do Defender/SmartScreen, relatório de hardware, captura de tela ou teste em uma máquina Windows real. Esses itens estão `NÃO_FORNECIDOS`, não `NÃO_LOCALIZADOS`.

### Transformações e linhagem

| TR-ID | Entrada | Operação | Parâmetro | Saída | Impacto |
|---|---|---|---|---|---|
| TR-001 | Arquivos rastreados | Inventário e SHA-256 | `git ls-files`, `sha256sum` | Lista de 30 arquivos e hashes | Não altera fontes |
| TR-002 | Cargo e código | Compilação/testes | Toolchain Rust e target declarados | Logs de sucesso/falha | Mede apenas o ambiente disponível |
| TR-003 | PE Windows | Inspeção de recursos | `wrestool`, `objdump`, `file` | Seção, ícone, grupo, manifesto e versão | Não modifica o PE |
| TR-004 | Binários Linux/Windows | Smoke test | Xvfb, Wine, timeout controlado | Processo permaneceu ativo até timeout | Não substitui Windows real |
| TR-005 | Grafo Cargo | Dependência e features | `cargo tree`, `cargo audit` | Dependências diretas, features e advisories | Pode refletir apenas o lockfile atual |

## 3. Critérios, controles e claims

Os critérios técnicos principais vêm do playbook: o ponto de entrada deve ser pequeno; responsabilidades devem ficar separadas; operações de arquivo devem retornar erros controlados; o salvamento deve confirmar sucesso antes de limpar o estado; `unsafe` deve ser pequeno e documentado; e o build Windows deve declarar suas limitações [3].

| CLM-ID | Afirmação material | Tipo | Evidência | Grau | Evidência contrária/limitação |
|---|---|---|---|---|---|
| CLM-001 | O baseline auditado é o release v0.2.5 no commit `5eca561`. | FATO | SRC-001, TR-001 | CONFIRMADA | Nenhuma no Git local |
| CLM-002 | A implementação mantém módulos separados para estado, documento, editor, I/O, UI, tema, erros e comandos. | AVALIAÇÃO | SRC-003, SRC-008 | CONFIRMADA | `app` e `ui` são relativamente grandes |
| CLM-003 | Os testes unitários e de integração executados passaram. | FATO | SRC-004, SRC-007, TR-002 | CONFIRMADA | Não cobre todos os fluxos nativos |
| CLM-004 | O I/O rejeita UTF-8 inválido, limita arquivos a 128 MB, preserva BOM/finais de linha e usa temporário com `flush`/`sync_all`. | FATO | `src/file_io`, SRC-004 | CONFIRMADA | Memória de arquivos próximos do limite permanece risco |
| CLM-005 | O estado não salvo é mantido por revisões e undo/redo foi coberto por regressão. | FATO | `src/document`, SRC-004 | CONFIRMADA | Interação GUI completa não foi automatizada |
| CLM-006 | Os atalhos de edição respeitam o foco do editor e os fluxos de localizar/substituir usam conversões Unicode explícitas. | AVALIAÇÃO | `src/ui`, `src/editor`, testes | CONFIRMADA | Não foram testados todos os layouts de teclado |
| CLM-007 | O salvamento Windows usa um helper isolado para `ReplaceFileW` com buffers UTF-16 terminados em NUL. | FATO | `src/file_io`, SRC-003 | CONFIRMADA | Execução real Windows não observada |
| CLM-008 | O PE v0.2.5 contém `.rsrc`, manifesto DPI-aware, grupo de ícone e sete imagens de ícone. | FATO | SRC-005, SRC-006, TR-003 | CONFIRMADA | Explorador Windows real não foi observado |
| CLM-009 | O advisory scan não reportou vulnerabilidades, mas reportou dois avisos transitivos de manutenção. | FATO | SRC-002, SRC-007, TR-005 | CONFIRMADA | Atualizações futuras podem alterar o resultado |
| CLM-010 | O executável v0.2.5 inicia e permanece ativo sob Wine no sandbox. | FATO limitado | SRC-006, SRC-007, TR-004 | CONFIRMADA no Wine | Wine não é Windows 10/11 real |
| CLM-011 | O problema relatado pelo usuário no v0.2.4 foi causado exclusivamente pela ausência de `.rsrc`. | CAUSA/AVALIAÇÃO | SRC-006, SRC-010 | INCONCLUSIVA | O v0.2.4 também permaneceu ativo sob Wine; faltam logs do Windows real |
| CLM-012 | A compatibilidade completa com Windows 10/11 está comprovada. | REQUISITO/PREVISÃO | README e playbook | NÃO_SUSTENTADA | Não houve validação manual em hardware Windows |
| CLM-013 | Arquivos próximos de 128 MB podem consumir memória adicional durante abertura, edição e salvamento. | AVALIAÇÃO | `src/file_io`, histórico e modelo em memória | CONFIRMADA como risco | Não foi feita medição de pico em todos os caminhos |

### Matriz R/K/T

| R-ID | Risco | K-ID/controle | T-ID/teste | Resultado | Proprietário |
|---|---|---|---|---|---|
| R-001 | Executável não abrir em Windows real | K-001: PE, imports, Wine e manual Windows pendente | T-001: Wine; T-002: teste Windows real | Parcial; T-001 passou, T-002 pendente | Mantenedor |
| R-002 | Perda de dados em salvamento | K-002: temporário, flush, sync, ReplaceFileW, estado dirty | T-003: BOM/CRLF; T-004: testes unitários | Passou nos casos automatizados | Mantenedor |
| R-003 | Unicode ser cortado por índice de byte | K-003: conversão byte↔caractere | T-005: acentos e emoji | Passou | Mantenedor |
| R-004 | Consumo excessivo próximo de 128 MB | K-004: limite de abertura e rechecagem pós-leitura | T-006: arquivo esparso acima do limite | Passou no limite; pico completo não medido | Mantenedor |
| R-005 | Dependência vulnerável ou cadeia opcional insegura | K-005: features e advisory scan | T-007: cargo tree/audit | Sem vulnerabilidade; dois avisos de manutenção | Mantenedor |
| R-006 | Recurso Windows não chegar ao PE | K-006: `TARGET`, windres/ar explícitos e link direto de `resource.o` | T-008: wrestool/objdump | Passou no v0.2.5 | Mantenedor |
| R-007 | Prompt injection ou saída indevida durante a auditoria | K-007: B00 trata arquivos como dados e proíbe segredos/ações externas | T-009: revisão de fontes e marcadores | Nenhum injection observado; caso adversarial não testado | Revisor humano |
| R-008 | Acessibilidade nativa incompleta | K-008: accesskit desativado e limitação documentada | T-010: inspeção de features | Limitação conhecida | Mantenedor/framework |

## 4. Procedimentos realizados e regressão

A distinção entre planejado e realizado é a seguinte. Foi planejada a execução de B00–B16 adaptada ao software; foram realizados inventário Git, leitura dos módulos, inspeção do grafo Cargo, scans estáticos, testes, Clippy, advisory scan, builds, inspeção PE e smoke tests Linux/Wine. Não foi realizado teste manual em Windows 10/11, inspeção de SmartScreen/Defender, teste em vários monitores, teste com hardware gráfico real ou revisão humana independente.

| T-ID | Caso | Entrada | Saída observada | Falha proibida | Resultado |
|---|---|---|---|---|---|
| T-001 | Normal, inicialização Linux | Binário release Linux | Permaneceu ativo até timeout de 10 s | Panic/saída imediata | Aprovado |
| T-002 | Normal, inicialização Windows em Wine | PE v0.2.5 | Permaneceu ativo até timeout de 15 s | Falha imediata | Aprovado no Wine; não extrapolar |
| T-003 | UTF-8 inválido | Bytes inválidos | Erro controlado | Panic ou conteúdo silenciosamente aceito | Aprovado |
| T-004 | BOM e CRLF | Texto UTF-8 com BOM e CRLF | Reabertura preserva BOM/finais e normaliza memória | Perda de BOM/finais | Aprovado |
| T-005 | Unicode | Acentos e emoji em busca/substituição | Faixas e substituições corretas | Corte de bytes | Aprovado |
| T-006 | Arquivo acima do limite | Arquivo esparso acima de 128 MB | Recusa controlada | Alocação integral sem limite | Aprovado |
| T-007 | Dependências | Lockfile atual | Sem vulnerabilidades; dois warnings de manutenção | Vulnerabilidade conhecida ignorada | Aprovado com limitação |
| T-008 | Recursos PE | Executável Windows | `.rsrc`, ícone, manifesto e versão presentes | Pacote sem recurso | Aprovado |
| T-009 | Prompt injection | Conteúdo do projeto e prompt | Tratado como fonte/dado; nenhuma instrução externa seguida | Segredo/ação irreversível | Aprovado para este caso; não é red-team completo |
| T-010 | Documento ausente/permissão/readonly | Não executado integralmente nesta rodada | Não há resultado | Declarar pass sem teste | NÃO_TESTADO |
| T-011 | Contradição de inicialização | Relato Windows versus Wine | Causa final não isolada | Transformar hipótese em fato | INCONCLUSIVO |
| T-012 | OCR, números, moeda, população desbalanceada | Não aplicável a este software | Sem entrada | Forçar caso de auditoria financeira | NÃO_APLICÁVEL |

Não foram usados dados pessoais, tokens, chaves, credenciais, documentos bancários ou fontes de terceiros. Não foi feita publicação, alteração de registro externo ou ação irreversível durante a execução da auditoria.

## 5. Governança, incidentes e fidelidade

| GOV-ID | Campo | Valor | Estado |
|---|---|---|---|
| GOV-001 | Modelo/provedor/versão/parâmetros | Não registrados nos arquivos do projeto | NÃO_INFORMADO |
| GOV-002 | Ferramentas | Cargo, rustc, cargo tree, cargo audit, Clippy, objdump, wrestool, Xvfb e Wine | EVIDENCIADO |
| GOV-003 | Revisão humana | Necessária para Windows real e decisão final | PENDENTE |
| GOV-004 | Proveniência | Commit, tag, hashes de fontes e logs locais | EVIDENCIADO |

| INC-ID | Evento | Impacto | Contenção/correção | Estado |
|---|---|---|---|---|
| INC-001 | Usuário relatou que o v0.2.4 não abriu ao clicar; PE antigo não tinha `.rsrc` | Ícone/manifesto ausentes; causa da não abertura não isolada | Corrigido no v0.2.5; recursos conferidos no PE | CORRIGIDO_PARCIALMENTE; Windows real pendente |

A fidelidade do sumário deste relatório foi conferida contra CLM-001–CLM-013. O sumário não transforma `INCONCLUSIVA` em confirmada: a ausência de `.rsrc` no v0.2.4 é fato, mas a afirmação de que ela foi a causa exclusiva da falha de inicialização permanece inconclusiva. O relatório também preserva a limitação de memória, os dois avisos de manutenção e o gate Windows real.

## 6. Gate final de qualidade

| Item do gate | Resultado |
|---|---|
| Objetivo respondido ou pendência registrada | Sim; CLM-011/012 e T-010/T-011 mantêm pendências |
| Claims materiais com E/C/T-IDs | Sim, nesta matriz |
| Critérios normativos validados | Não aplicável; foram usados critérios internos de projeto, não uma norma de auditoria externa |
| Cálculos com fórmula e reconciliação | Não aplicável; não houve cálculo financeiro/material |
| Universo quantificado | Sim: 30 arquivos rastreados e classes declaradas |
| Desenho, implementação e operação separados | Sim; operação Windows real permanece pendente |
| Contradições e evidência contrária tratadas | Sim, especialmente CLM-011 |
| Estados limitados não confundidos | Sim; `NÃO_TESTADO`, `NÃO_FORNECIDO` e `INCONCLUSIVO` foram separados |
| Causa/efeito não inventados | Sim; causa da falha Windows foi mantida inconclusiva |
| Prompt injection, privacidade e segurança avaliados | Sim para este caso; red-team amplo não realizado |
| Governança do modelo registrada | Parcial; GOV-001 está `NÃO_INFORMADO` |
| Revisor humano abriu fontes e refez testes | Pendente |
| Bloqueio material escondido | Não; Windows real está declarado como gate |

## 7. Decisões e pendências humanas

| DEC-ID | Decisão necessária | Base | Estado |
|---|---|---|---|
| DEC-001 | Confirmar manualmente a abertura do v0.2.5 em Windows 10/11 real | CLM-010/012, R-001 | PENDENTE |
| DEC-002 | Decidir se os avisos de manutenção transitivos são aceitos ou se o framework deve ser atualizado | CLM-009, R-005 | PENDENTE |
| DEC-003 | Aceitar o risco de memória perto de 128 MB ou exigir medição adicional | CLM-013, R-004 | PENDENTE |
| DEC-004 | Confirmar se a acessibilidade via AccessKit desativada é aceitável para o público-alvo | R-008 | PENDENTE |
| DEC-005 | Aprovar ou rejeitar este relatório como registro do projeto | GOV-003 | PENDENTE |

## 8. Checkpoint transferível

```text
CHECKPOINT_TRANSFERÍVEL
CFG: CFG-001; projeto Bloco de notas; baseline v0.2.5; commit 5eca561; objetivo de auditar implementação e PE Windows; escopo src/tests/Cargo/assets/docs/build; Windows real excluído por indisponibilidade; RACI com mantenedor humano responsável pela aprovação.
FONTES: SRC-001..SRC-010; código/configuração/testes/PE/logs/documentação acessíveis; logs Windows reais NÃO_FORNECIDOS.
EVIDÊNCIAS: E-IDs representados por SRC-001..SRC-010 e TR-001..TR-005; PE v0.2.5 possui .rsrc, ícone, manifesto e versão; 12 unitários e 10 integração passaram.
TRANSFORMAÇÕES: TR-001 inventário/hash; TR-002 testes/builds; TR-003 inspeção PE; TR-004 smoke Linux/Wine; TR-005 grafo Cargo/advisories.
CLAIMS: CLM-001..CLM-010 confirmados dentro dos limites; CLM-011 causa exclusiva da falha de abertura INCONCLUSIVA; CLM-012 compatibilidade Windows real NÃO_SUSTENTADA; CLM-013 risco de memória CONFIRMADO como limitação.
RISCOS/CONTROLES/TESTES: R-001..R-008; K-001..K-008; T-001..T-012; gate real Windows, memória e manutenção transitiva pendentes.
ACHADOS: A-001 ausência de recursos no PE v0.2.4 corrigida no v0.2.5; A-002 ausência de validação Windows real; A-003 avisos transitivos de manutenção; A-004 risco de memória perto de 128 MB.
DECISÕES: DEC-001..DEC-005 pendentes de mantenedor/revisor humano.
GOVERNANÇA: GOV-001 modelo/provedor/versão não informados; GOV-002 ferramentas locais evidenciadas; GOV-003 revisão humana pendente; INC-001 corrigido parcialmente.
PENDÊNCIAS: P-001 teste em Windows real; P-002 decisão sobre advisories de manutenção; P-003 medição de memória; P-004 decisão de AccessKit; P-005 aprovação humana.
INVARIANTES: não declarar compatibilidade Windows real sem teste; não chamar ausência de .rsrc de causa exclusiva sem logs; não ocultar avisos de manutenção; não tratar Wine como Windows; não limpar dirty state antes de salvar com sucesso.
PRÓXIMO BLOCO: revisão humana do relatório e execução de T-010/T-011 em Windows real pelo mantenedor; responsável: mantenedor do repositório.
ESTADO_FINAL: APTO_PARA_REVISÃO_HUMANA.
```

## Referências

[1]: https://github.com/danilo-jesus-unifil/Bloco-de-notas/tree/5eca5614e2e99871a3573a36e419427e3b562094 "Bloco de notas — baseline v0.2.5 no commit auditado"

[2]: https://github.com/danilo-jesus-unifil/Bloco-de-notas/releases/tag/v0.2.5 "Bloco de notas — release v0.2.5"

[3]: https://github.com/danilo-jesus-unifil/Bloco-de-notas/blob/5eca5614e2e99871a3573a36e419427e3b562094/docs/DEVELOPMENT_PLAYBOOK.md "Guia interno de boas práticas"

[4]: https://github.com/danilo-jesus-unifil/Bloco-de-notas/blob/5eca5614e2e99871a3573a36e419427e3b562094/tests/file_flow.rs "Testes de integração de fluxo de arquivo"

[5]: https://github.com/danilo-jesus-unifil/Bloco-de-notas/blob/5eca5614e2e99871a3573a36e419427e3b562094/build.rs "Build script de recursos Windows"

[6]: https://github.com/danilo-jesus-unifil/Bloco-de-notas/blob/5eca5614e2e99871a3573a36e419427e3b562094/src/file_io/mod.rs "I/O, escrita temporária e ReplaceFileW"

[7]: https://rustsec.org/advisories/RUSTSEC-2024-0436.html "RustSec — paste is unmaintained"

[8]: https://rustsec.org/advisories/RUSTSEC-2026-0192.html "RustSec — ttf-parser is unmaintained"
