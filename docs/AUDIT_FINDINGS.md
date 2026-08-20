# Auditoria do projeto — primeira passagem

## Escopo da auditoria

A auditoria foi iniciada sobre o commit publicado `1a42eec` e o release `v0.2.0`, verificando código-fonte, manifesto, histórico Git, artefatos, testes, execução headless, dependências, caminhos de entrada externa, documentação e configuração de build.

## Confirmado no estado atual

| Área | Evidência | Situação |
|---|---|---|
| Rust e Cargo | `Cargo.toml`, `Cargo.lock`, build local e target Windows | Implementado |
| Modularização | `main.rs`, `app`, `document`, `editor`, `file_io`, `ui`, `theme`, `error` e `commands` | Coerente, com `app` e `ui` candidatos a divisão adicional |
| Arquivos | Leitura UTF-8, BOM, filtros, escrita temporária, `flush` e `sync_all` | Implementado |
| Edição | Unicode, Undo/Redo, clipboard, busca e substituição | Implementado com cobertura unitária |
| Abas | Estado por aba e ID de editor por aba | Implementado |
| Drag and drop | `ViewportBuilder::with_drag_and_drop(true)` e leitura de `dropped_files` | Implementado |
| Persistência | Preferências via eframe e persistência de janela | Implementado |
| Windows | Manifesto DPI, ícone, build PE32+ x86-64 | Build confirmado; execução visual ainda depende de Windows |
| Smoke test | Binário release Linux executado por 10 segundos sob Xvfb sem saída de erro | Inicialização confirmada |
| Git/release | Branch limpo, tags anotadas `v0.1.0` e `v0.2.0`, releases publicados | Confirmado |

## Problemas e riscos encontrados

### 1. Busca com Unicode poderia usar índices de byte incorretos

O editor usa `String::find` e `String::rfind` em alguns caminhos e mistura índices retornados pela busca com índices de caracteres usados pelo cursor egui. Em texto ASCII isso parece correto, mas em consultas ou conteúdo com acentos e emojis pode selecionar ou substituir uma faixa incorreta. Esta é uma falha funcional real e deve ser corrigida centralizando a conversão entre índices de byte e índices de caractere.

### 2. O primeiro `cargo audit` encontrou duas vulnerabilidades altas transitiveis

A configuração original de eframe puxava `quick-xml 0.30.0` por `zbus_xml 4.0.0` e a cadeia Linux de acessibilidade. O advisory database identificou `RUSTSEC-2026-0194` e `RUSTSEC-2026-0195`, ambos com solução `quick-xml >=0.41.0`. O caminho vinha de `eframe -> egui-winit -> accesskit_unix -> atspi -> zbus_xml`; não há ocorrência dessa cadeia no `cargo tree` do target Windows.

A correção de menor risco é desativar o recurso opcional `accesskit` no eframe 0.29, mantendo o editor e os recursos Windows estáveis. O advisory scan posterior não encontrou vulnerabilidades, mas ainda reportou avisos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`, dependências transitivas do renderizador/fontes que não possuem substituição direta sem trocar o framework gráfico.

### 3. Arquivo de aplicação concentra várias responsabilidades

`src/app/mod.rs` possui aproximadamente 666 linhas e reúne estado de abas, persistência, comandos, I/O, confirmação de saída, clipboard, seleção e ciclo de vida. `src/ui/mod.rs` possui aproximadamente 438 linhas e reúne composição geral, menus, busca, configurações, status e atalhos. A separação atual é compreensível e não é um monólito absoluto, mas a auditoria recomenda dividir por responsabilidades reais se uma correção posterior tocar novamente esses arquivos.

### 4. Validação visual Windows não está disponível no sandbox

O binário Windows foi gerado como PE32+ x86-64, e o binário Linux iniciou sob Xvfb. Não foi possível testar manualmente menus, clipboard, drag and drop, DPI, persistência de janela e diálogos no Windows 10/11 real. Essa limitação não é tratada como falha corrigida automaticamente; permanece registrada para a entrega.

## Correções aplicadas nesta auditoria

A busca e a substituição agora convertem explicitamente entre índices de byte e índices de caractere. Foram adicionados testes com acentos, emoji, busca reversa e substituição Unicode, eliminando o risco de selecionar ou alterar uma faixa incorreta.

A abertura passou a consultar o tamanho do arquivo antes de alocar o conteúdo e recusa arquivos acima de 128 MB com mensagem clara. Também foi adicionada uma regressão com arquivo esparso acima do limite.

O salvamento foi dividido por plataforma: em Windows, a substituição de um arquivo existente usa `ReplaceFileW` por uma interface pequena e isolada; em outros sistemas, permanece o fluxo de rename controlado já testado. O projeto agora declara `windows-sys` somente para o target Windows.

A configuração do eframe deixou de ativar o recurso opcional `accesskit` nesta versão. O `cargo audit` deixou de reportar as duas vulnerabilidades altas de `quick-xml 0.30.0`; permanecem somente dois avisos de dependências transitivas não mantidas (`paste 1.0.15` e `ttf-parser 0.25.1`), sem vulnerabilidade reportada e sem substituição direta compatível com eframe 0.29.1. A decisão e a limitação de acessibilidade nativa estão documentadas no README.

## Segunda revisão concluída

A segunda revisão repetiu os scans de placeholders, panics, unsafe, dependências, diff e tamanho dos módulos. Não foram encontrados placeholders funcionais, código morto evidente ou `unsafe` fora do helper Windows documentado. O `cargo audit --no-fetch` permaneceu sem vulnerabilidades, com os dois avisos transitivos de manutenção já registrados.

Também foram repetidos os testes unitários e de integração, Clippy, advisory scan, build release Linux, check do target Windows, smoke test headless sob Xvfb e verificação do estado publicado. O branch v0.2.1 continua limpo e sincronizado, sem alterações acidentais fora do escopo auditado. Nenhum novo problema relevante foi encontrado, portanto não foi criada uma mudança de código apenas para gerar um novo release. A única limitação relevante remanescente é a validação visual e funcional em Windows real, especialmente DPI, clipboard, drag and drop, diálogos e acessibilidade nativa.

## Terceira execução da auditoria

A auditoria foi executada novamente sobre o mesmo baseline publicado e reproduziu os resultados anteriores: 10 testes unitários, 7 casos no harness de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, build Linux release, check Windows e smoke test headless passaram. Como não houve problema novo nem correção de código, o release v0.2.1 permanece a versão correta e não foi criado um v0.2.2 artificial.

## Quarta execução da auditoria

A quarta execução repetiu a análise funcional, os scans de segurança e placeholders, a revisão arquitetural, a análise de dependências, os testes de regressão, o build otimizado, o check Windows e a inicialização headless. Os resultados permaneceram consistentes e nenhuma falha nova justificou uma alteração de código ou um novo release.

## Quinta execução da auditoria

A quinta execução reproduziu novamente os testes, Clippy, advisory scan, build Linux release, check Windows, scans estáticos e smoke test headless. Não foram observados problemas novos de funcionamento, segurança, desempenho, arquitetura ou compatibilidade. O release v0.2.1 continua sendo o baseline correto.

## Sexta execução da auditoria

A sexta execução repetiu o ciclo completo de qualidade, incluindo testes unitários e de integração, Clippy, advisory scan, build otimizado, check Windows, scans estáticos, `git diff --check` e smoke test headless. Os resultados permaneceram estáveis e não surgiu nenhuma falha real que justificasse correção de código ou novo release.

## Sétima execução da auditoria

A sétima execução repetiu o mesmo ciclo completo, com 10 testes unitários, 7 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, build Linux release, check Windows, scans estáticos e smoke test headless aprovados. Nenhum novo problema de funcionamento, segurança, desempenho, arquitetura ou compatibilidade foi encontrado.

## Oitava execução da auditoria

A oitava execução repetiu novamente o ciclo de verificação completo. Os testes unitários e de integração, Clippy, advisory scan, build Linux release, check Windows, scans estáticos, `git diff --check` e smoke test headless passaram sem alterações de comportamento. O baseline v0.2.1 permanece estável e nenhum novo release é justificável.

## Nona execução da auditoria

A nona execução confirmou os mesmos resultados em uma nova rodada independente: testes unitários e de integração, Clippy, advisory scan, build Linux release, check Windows, scans estáticos, verificação de diff e smoke test headless passaram. Não houve mudança de comportamento nem falha nova que justificasse alteração funcional ou nova versão.

## Investigação adicional de comportamentos não cobertos

Uma revisão orientada a fluxos de usuário, além da bateria tradicional de compilação e testes, encontrou problemas reais que não apareciam no baseline das auditorias anteriores. As correções foram mantidas pequenas e limitadas ao comportamento do editor, das abas e do fechamento seguro.

| Área | Problema confirmado | Correção aplicada |
|---|---|---|
| Dirty state | Depois de editar um documento salvo e desfazer até o conteúdo salvo, o documento continuava marcado como não salvo e poderia pedir confirmação desnecessária ao fechar. | O histórico agora carrega revisões; undo e redo comparam a revisão atual à revisão salva. Foi adicionado teste de regressão, incluindo redo posterior. |
| Salvamento cancelado | Ao confirmar “Salvar” para fechar ou descartar uma aba sem nome, cancelar a caixa “Salvar como…” retornava sucesso lógico e podia permitir o fechamento com perda do conteúdo em memória. | Os fluxos de salvamento distinguem sucesso (`Ok(true)`) de cancelamento (`Ok(false)`), e as confirmações preservam a operação quando o usuário cancela. |
| Identidade das abas | O ID do editor era baseado no índice da aba. Ao fechar uma aba anterior, as abas seguintes mudavam de índice e podiam perder ou herdar cursor e seleção de outra aba. | Cada aba recebeu um ID estável durante sua vida; o identificador do `TextEdit` não depende mais da posição no vetor. |
| Atalhos em campos auxiliares | Ctrl+Z, Ctrl+Y, Ctrl+X, Ctrl+C, Ctrl+V e Ctrl+A eram processados pelo documento mesmo quando o foco estava no campo de localizar/substituir, impedindo a edição nativa do campo e podendo alterar o documento errado. | Esses atalhos agora são tratados pelo documento somente quando o editor principal está focado; os comandos de arquivo, busca e zoom continuam globais. |
| Localizar anterior e substituir | Com uma ocorrência selecionada, a busca anterior podia incluir a própria ocorrência selecionada e repeti-la. A substituição única podia ignorar a seleção e substituir a próxima ocorrência. | A integração usa o início da seleção como ponto de busca reversa e reconhece uma seleção que corresponde à consulta antes de substituir. A posição do cursor é mantida após a substituição. |

A validação posterior às correções passou com 11 testes unitários, 8 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, build release Linux, build release PE32+ x86-64 para Windows, scans estáticos, `git diff --check` e smoke test headless sob Xvfb. O `cargo audit` continua registrando apenas os avisos de manutenção transitiva já conhecidos para `paste 1.0.15` e `ttf-parser 0.25.1`.

Permanecem riscos que não puderam ser reproduzidos integralmente neste ambiente. O limite de 128 MB protege o tamanho do arquivo, mas a leitura, normalização e o histórico podem gerar cópias temporárias adicionais; arquivos próximos do limite podem consumir centenas de megabytes durante a abertura ou salvamento. Também continuam sem cobertura automatizada completa os diálogos nativos, clipboard, drag and drop, múltiplos monitores/DPI, acessibilidade e execução visual no Windows 10/11 real. Esses pontos devem ser validados manualmente antes de uma distribuição ampla; não foram tratados como falhas confirmadas nesta rodada.

## Auditoria completa pós-v0.2.2

A revisão do prompt de auditoria e do projeto inteiro confirmou que o escopo anterior continua preservado: editor UTF-8, BOM, finais de linha, abas locais, drag and drop, status, zoom, busca/substituição Unicode, clipboard, undo/redo, persistência mínima, tema escuro, manifesto DPI e build Windows. Não foram encontrados botões funcionais sem ação, integrações simuladas, placeholders de produto ou dependências diretas sem uso. A opção “Nova janela” continua deliberadamente desabilitada e agora é identificada como futura, coerente com o plano de escopo.

A auditoria funcional encontrou quatro pontos reais adicionais, todos corrigidos sem ampliar o produto:

| Área | Constatação | Tratamento |
|---|---|---|
| Feedback de erro | O campo `status` era atualizado em vários caminhos, mas não era renderizado; falhas poderiam ficar invisíveis para quem não inspecionasse o título ou o estado interno. | O status agora aparece na barra inferior e falhas também abrem diálogo de erro contextualizado, preservando a mensagem no status. |
| Barra de status em documentos grandes | A interface recalculava `text.chars().count()` e percorria todo o prefixo do documento para linha/coluna em cada frame. | A contagem de caracteres é mantida pelo documento e a posição de linha/coluna usa cache invalidado por aba, cursor e geração de conteúdo. |
| Abertura próxima do limite | O carregamento podia copiar o buffer ao retirar BOM, normalizar mesmo quando não havia `CR` e aceitar uma alteração de tamanho entre `metadata` e `read`. | O buffer é reutilizado quando possível, a normalização retorna o texto original para arquivos LF e o tamanho lido é verificado novamente. |
| Fechamento pelo menu/atalho | O fechamento iniciado pelo comando Sair podia ser confirmado pelo próprio comando e novamente pelo evento de fechamento da viewport. | Uma autorização de fechamento é consumida no evento seguinte, evitando confirmação duplicada; fechamentos externos continuam protegidos. |

Foi adicionada cobertura de entrada UTF-8 inválida, e a validação após as mudanças passou com 12 testes unitários, 10 casos no target de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, build release Linux, build release PE32+ x86-64 para Windows, scans estáticos, `git diff --check`, smoke test headless e interação headless de teclado com criação/fechamento de aba. A auditoria também confirmou que o único `unsafe` permanece isolado no helper `ReplaceFileW`, com buffers UTF-16 terminados em NUL e comentário de segurança.

A revisão arquitetural não justificou uma divisão artificial de `app/mod.rs` ou `ui/mod.rs`: os arquivos são maiores que o ideal, mas as responsabilidades ainda estão agrupadas por fluxo e não há dependência circular ou módulo sem coesão. A limitação material remanescente é a validação visual e funcional em Windows 10/11 real, incluindo clipboard, diálogos nativos, drag and drop, escalas DPI, múltiplos monitores e acessibilidade.

## Auditoria repetida pós-v0.2.3

A repetição integral deste prompt revalidou os requisitos anteriores, os fluxos funcionais, a interface, a arquitetura, o I/O, as dependências e a compatibilidade de build. Foram confirmados 12 testes unitários, 10 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, build release Linux, build PE32+ x86-64 para Windows, scans estáticos, `git diff --check` e smoke test headless.

A revisão focada no ciclo de encerramento identificou uma simplificação necessária: o v0.2.3 mantinha um sinal temporário para pular a confirmação depois do comando Sair, mas esse estado poderia ficar stale se o evento de fechamento não chegasse imediatamente. O fluxo foi corrigido para enviar apenas o pedido de fechamento; a confirmação agora fica centralizada no evento `close_requested`, que cobre tanto o menu/atalho quanto o botão externo da janela, sem estado auxiliar. A alteração foi recompilada e testada, e não foram encontrados outros problemas reais novos nesta rodada.

O baseline continua simples e modular o suficiente para o escopo. Não há placeholders funcionais, dependências diretas sem uso, cadeia `accesskit` habilitada ou vulnerabilidade reportada. Permanecem a validação visual em Windows real e o consumo adicional de memória de arquivos próximos do limite de 128 MB como limitações conhecidas.

## Nova repetição da auditoria completa

A nova execução do prompt confirmou novamente os requisitos funcionais, de UX, arquitetura, segurança, desempenho e compatibilidade já documentados. A bateria passou com 12 testes unitários, 10 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, build release Linux, build PE32+ x86-64 para Windows, scans de placeholders e `unsafe`, `git diff --check`, smoke test headless e interação headless com identificação da janela e atalhos básicos.

Não foram encontrados novos bugs reais, placeholders funcionais, mensagens de sucesso enganosas, dependências diretas sem uso ou regressões no fluxo de fechamento. O único `unsafe` continua isolado no helper Windows de substituição atômica. A execução visual em Windows 10/11 real permanece a limitação conhecida, especialmente para diálogos nativos, clipboard, drag and drop, DPI, múltiplos monitores e acessibilidade.

Como não houve mudança de código ou de comportamento do produto nesta rodada, não foi criado um release artificial. O v0.2.4 continua sendo o release correto; esta execução gera apenas um registro documental.

## Auditoria do pasted_content_9.txt

A nova revisão completa revalidou o baseline v0.2.4 e encontrou uma inconsistência documental real: `CHANGELOG.md` registrava somente até o v0.2.1, embora os releases v0.2.2, v0.2.3 e v0.2.4 estivessem publicados e documentados em notas próprias. O histórico foi atualizado com entradas fiéis para as três versões, preservando o conteúdo anterior e alinhando a ordem do arquivo às tags existentes.

Após a correção documental, a validação passou novamente com 12 testes unitários, 10 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, builds release Linux e Windows PE32+ x86-64, scans estáticos, `git diff --check`, smoke test headless e interação headless com identificação da janela e atalhos básicos. Não foram encontrados novos problemas de código, segurança, desempenho, interface ou compatibilidade.

Como o achado foi exclusivamente documental e não alterou o comportamento do aplicativo, não foi criado um novo release. O v0.2.4 continua sendo a versão funcional correta; esta rodada acrescenta somente a atualização do CHANGELOG e o registro desta auditoria.

## Auditoria do pasted_content_10.txt

A nova revisão revalidou os requisitos, os fluxos funcionais, a arquitetura, a segurança, o desempenho, a interface, as dependências, o I/O, o fechamento e a compatibilidade do v0.2.4. Ela encontrou uma pequena inconsistência na correção documental anterior: uma linha sobre o fechamento centralizado, pertencente ao v0.2.4, havia sido repetida indevidamente na seção do v0.2.3 do `CHANGELOG.md`. A linha foi removida, mantendo cada release alinhado ao commit que realmente o introduziu.

Depois dessa correção, passaram novamente 12 testes unitários, 10 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, builds release Linux e Windows PE32+ x86-64, scans estáticos, `git diff --check`, smoke test headless e interação headless com identificação da janela e atalhos básicos. Não foram encontrados novos problemas de código, segurança, desempenho, interface ou compatibilidade.

O achado permaneceu exclusivamente documental e não alterou o comportamento do aplicativo. Não foi criado novo release; o v0.2.4 continua sendo a versão funcional correta.
