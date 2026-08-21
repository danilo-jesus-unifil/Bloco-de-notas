# Release v0.2.4

## O que mudou

O fluxo de fechamento foi simplificado. O comando **Sair** apenas solicita o fechamento da janela, e a confirmação de alterações não salvas fica centralizada no evento `close_requested`.

Esse fluxo cobre o menu, os atalhos e o botão externo da janela da mesma forma. O estado temporário usado para pular a confirmação foi removido, evitando que uma autorização antiga permanecesse ativa se o evento fosse atrasado ou não chegasse.

## Verificações

Foram executados `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo audit --no-fetch`, `cargo build --release`, build release Windows `x86_64-pc-windows-gnu`, `git diff --check`, scans estáticos e smoke test sob Xvfb.

O estado validado passou com 12 testes unitários e 10 casos de integração. O advisory scan não encontrou vulnerabilidades; permanecem os avisos transitivos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`.

## Limitações

A validação manual em Windows 10/11 ainda é necessária para clipboard, drag and drop, diálogos nativos, escalas DPI, múltiplos monitores e acessibilidade. Arquivos próximos de 128 MB podem exigir memória adicional durante a edição e o salvamento.

## Artefatos

O release inclui um pacote Windows x86-64 e o checksum SHA-256 correspondente.
