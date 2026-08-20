# Bloco de notas v0.2.0

## Resumo

Esta versão complementa o primeiro release com uma interface mais próxima da sensação visual do Bloco de Notas do Windows 11, sem abandonar a compatibilidade com Windows 10 nem a regra de manter o aplicativo pequeno, rápido e previsível.

## Mudanças principais

A janela agora possui cabeçalho compacto, ícone visual, abas locais, botão de nova aba, fechamento de aba e configuração para abrir arquivos em novas abas. O editor continua sendo a área dominante da janela e mantém Undo/Redo, clipboard, Unicode, localizar e substituir.

O sistema aceita os principais formatos de texto solicitados — `.txt`, `.md`, `.log`, `.csv`, `.ini` e `.json` — sem adicionar parsers específicos. O drag and drop abre o primeiro arquivo compatível soltado sobre a janela. A barra de status apresenta linha, coluna, caracteres, zoom, finais de linha, encoding e estado de salvamento.

O zoom pode ser alterado pelo menu ou por `Ctrl++`, `Ctrl+-` e `Ctrl+0`. O painel de configurações reúne tamanho da fonte, quebra automática, visibilidade da barra de status, comportamento de abertura e tema. O tema escuro continua ativo; Claro e Seguir o sistema estão documentados como futuras opções, sem implementação incompleta.

Preferências mínimas e geometria da janela usam o armazenamento do eframe em local apropriado ao sistema. Conteúdo de documentos não é salvo secretamente como recuperação automática. Finais de linha `LF`, `CRLF` e `CR` são detectados, normalizados internamente e gravados novamente de forma previsível.

## Validações executadas

| Verificação | Resultado |
|---|---|
| `cargo fmt --check` | Passou |
| `cargo check` | Passou |
| `cargo test` | Passou: testes unitários e de integração |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passou sem warnings |
| `cargo build --release` | Passou e gerou binário otimizado local |
| `cargo build --release --target x86_64-pc-windows-gnu` | Passou e gerou executável PE32+ Windows x86-64 |
| `git diff --check` | Passou |

## Limitações

A execução visual manual em Windows 10/11 não está disponível neste ambiente Linux. O pacote deve ser aberto em Windows antes de distribuição ampla para validar DPI, menus nativos, clipboard, drag and drop, abas, restauração de janela, arquivos somente leitura e fechamento com alterações.
