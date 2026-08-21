# Auditoria e achados

Este documento resume as revisões técnicas do projeto e as correções que chegaram aos releases publicados. O foco foi verificar o funcionamento do editor, os fluxos de arquivo, a interface, a segurança, o desempenho e a compatibilidade com Windows 10 e 11.

## Estado atual

O baseline atual é o release `v0.2.4`, no commit `e5301b9`. O projeto está organizado em módulos de aplicação, documento, editor, I/O, comandos, interface, tema e erros. O ponto de entrada permanece pequeno, e o código específico do Windows está isolado no helper de substituição atômica.

| Área | Evidência | Situação |
|---|---|---|
| Edição | Unicode, Undo/Redo, clipboard, localizar e substituir | Confirmado |
| Arquivos | UTF-8, BOM, `LF`, `CRLF`, `CR`, escrita temporária, `flush` e `sync_all` | Confirmado |
| Abas | Estado por aba, cursor e identificador estável do editor | Confirmado |
| Interface | Tema escuro, menus, configurações, zoom, status e drag and drop | Confirmado |
| Persistência | Preferências e geometria da janela pelo eframe | Confirmado |
| Windows | Manifesto DPI-aware, ícone e build PE32+ x86-64 | Build confirmado; execução manual ainda pendente |
| Dependências | Nenhuma vulnerabilidade no advisory scan | Confirmado, com dois avisos de manutenção transitivos |

## Correções incorporadas

### v0.2.1

A busca reversa e a substituição passaram a converter explicitamente índices de byte e de caractere, evitando faixas incorretas em acentos e emoji. A abertura recusa arquivos acima de 128 MB antes da leitura integral. O salvamento no Windows usa `ReplaceFileW` em uma interface pequena e isolada; em outros sistemas, a substituição preserva o original quando o rename falha.

O recurso opcional `accesskit` foi desativado no eframe 0.29.1. A mudança removeu a cadeia que trazia `quick-xml 0.30.0` e suas vulnerabilidades altas, sem exigir uma migração incompatível do framework.

### v0.2.2

O estado não salvo passou a acompanhar revisões do documento. Undo até o conteúdo salvo limpa o estado de alteração, e redo volta a marcá-lo. Cancelar Salvar como durante o fechamento preserva a aba. Cada aba recebeu um identificador estável, os atalhos respeitam o foco nos campos de busca e Substituir respeita a ocorrência selecionada.

### v0.2.3

O status interno passou a aparecer na interface e as falhas passaram a abrir diálogos contextualizados. A contagem de caracteres e a posição do cursor ganharam caches adequados ao tamanho do documento. A abertura reduziu cópias desnecessárias, revalidou o limite de 128 MB e passou a rejeitar UTF-8 inválido com teste de regressão.

### v0.2.4

O fechamento foi centralizado no evento `close_requested`. O comando Sair apenas solicita o fechamento, e o mesmo caminho confirma alterações não salvas para o menu, os atalhos e o botão externo da janela. O estado temporário de autorização foi removido.

## Validação atual

A validação mais recente passou com 12 testes unitários, 10 casos de integração, `cargo fmt --check`, `cargo check`, Clippy sem warnings, `cargo audit --no-fetch`, build release Linux, build release Windows `x86_64-pc-windows-gnu`, `git diff --check`, scans estáticos, smoke test sob Xvfb e interação headless de teclado.

O advisory scan não encontrou vulnerabilidades. Permanecem os avisos de manutenção transitivos para `paste 1.0.15` e `ttf-parser 0.25.1`; ambos vêm de dependências do framework e não têm substituição direta compatível com eframe 0.29.1.

As revisões posteriores ao v0.2.4 não encontraram novos problemas de código, segurança, desempenho, interface, arquitetura ou documentação. Por isso, não houve release posterior.

## Riscos e limitações

A execução visual em Windows 10/11 real ainda é necessária para validar diálogos nativos, clipboard, drag and drop, escalas DPI, múltiplos monitores e acessibilidade. O único `unsafe` continua no helper Windows que chama `ReplaceFileW`.

Arquivos próximos do limite de 128 MB podem exigir memória adicional durante normalização, histórico, edição e salvamento, porque o editor mantém o documento em memória. A acessibilidade nativa via AccessKit permanece desativada até que o framework possa ser atualizado sem uma migração desproporcional.

## Decisão de escopo

O projeto continua deliberadamente pequeno. Nova janela, recuperação de abas fechadas, múltiplos painéis, editor rico, temas claro e do sistema, família tipográfica configurável, instalador, plugins, árvore de arquivos e sincronização permanecem fora do escopo atual.

## v0.2.5: recursos Windows no cross-build

A execução em Windows real revelou que o executável distribuído não iniciava para o usuário e também não apresentava o ícone esperado. A inspeção do PE v0.2.4 mostrou que a entrada `Resource Directory` estava vazia: não havia seção `.rsrc`, embora o projeto tivesse manifesto e arquivo ICO no código-fonte.

A causa foi o uso de `#[cfg(windows)]` no `build.rs`. Em um cross-build iniciado no Linux, essa condição descrevia o host e impedia a execução do `winres` para o target Windows. Depois que a condição foi corrigida, o objeto de recursos ainda precisava ser ligado diretamente ao executável GNU, porque a biblioteca estática sem símbolos referenciados podia ser descartada pelo linker.

O v0.2.5 detecta o target por `TARGET`, configura as ferramentas MinGW corretas e liga diretamente `resource.o`. A inspeção do novo PE confirmou a seção `.rsrc`, sete imagens de ícone, grupo de ícone, versão Windows e manifesto DPI-aware. O executável e o candidato anterior permaneceram ativos sob Wine até o timeout controlado; a validação final em Windows 10/11 real continua necessária.

A validação passou com 12 testes unitários, 10 casos de integração, Clippy sem warnings, advisory scan sem vulnerabilidades, builds Linux e Windows, inspeção de recursos PE e `git diff --check`. Permanecem os avisos transitivos de manutenção para `paste` e `ttf-parser`.
