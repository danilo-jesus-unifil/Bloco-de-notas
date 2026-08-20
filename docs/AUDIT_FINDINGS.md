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
