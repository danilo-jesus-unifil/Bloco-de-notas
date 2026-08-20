# Bloco de notas v0.1.0

## Resumo

A primeira versão entrega um bloco de notas desktop simples, rápido e predominantemente escuro, escrito em Rust com `egui/eframe` e direcionado a Windows 10. O escopo foi mantido deliberadamente pequeno para priorizar estabilidade, responsividade, edição de texto confiável e salvamento seguro.

## Incluído

A versão inclui edição Unicode, novo documento, abrir, salvar, salvar como, confirmação para alterações não salvas, desfazer, refazer, copiar, recortar, colar, selecionar tudo, localizar, substituir, substituição em massa, quebra automática de linha, ajuste de fonte entre 10 e 32 pontos, menus familiares, atalhos convencionais e barra de status.

Arquivos UTF-8 são lidos com suporte a BOM. Finais de linha `CRLF` e `CR` são normalizados para `LF`. O salvamento escreve em arquivo temporário no mesmo diretório, faz `flush` e `sync_all`, e só então tenta substituir o destino. Falhas mantêm o documento em memória como não salvo e são apresentadas de forma compreensível.

O build Windows incorpora um ícone multi-resolução e um manifesto com `PerMonitorV2` para conscientização de DPI. O `main.rs` permanece pequeno, e a lógica está separada em módulos de aplicação, documento, editor, I/O, comandos, interface, tema e erros.

## Validações executadas

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | Passou |
| `cargo check` | Passou |
| `cargo test` | Passou: 7 testes no total, incluindo fluxo de salvar e reabrir |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passou |
| `cargo build --release` | Passou no ambiente Linux |
| `cargo build --release --target x86_64-pc-windows-gnu` | Passou; executável Windows gerado |
| `git diff --check` | Passou |

## Artefato

O pacote `bloco-de-notas-v0.1.0-windows-x86_64.zip` contém o executável Windows GNU x86-64 e um README curto de distribuição. O executável foi compilado como aplicação GUI PE32+ e não depende de DLLs MinGW adicionais na inspeção das importações; a execução manual em Windows 10 continua recomendada antes de distribuição ampla.

## Limitações

A validação visual e comportamental em Windows 10 não foi executada neste ambiente Linux. O release deve ser testado em Windows 10 com escalas de 100%, 125%, 150%, 175% e 200%, incluindo clipboard, diálogos, fechamento com alterações, arquivos somente leitura e movimento entre monitores com DPI diferente.
