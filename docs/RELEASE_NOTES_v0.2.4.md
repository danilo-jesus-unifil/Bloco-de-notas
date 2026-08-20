# Release v0.2.4

## Resumo

A versão `v0.2.4` é uma correção pontual identificada na repetição da auditoria completa pós-v0.2.3. O fechamento foi simplificado para manter uma única fonte de verdade sobre a confirmação de alterações não salvas.

## Correção

O comando **Sair** agora apenas solicita o fechamento da viewport. A confirmação de abas não salvas fica centralizada no evento `close_requested`, cobrindo de forma uniforme o menu, os atalhos e o botão externo da janela. Foi removido o estado temporário usado para pular a confirmação, eliminando o risco de esse sinal permanecer ativo caso o evento de fechamento fosse atrasado ou não chegasse.

## Verificações executadas

Foram repetidos `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo audit --no-fetch`, `cargo build --release`, build release Windows `x86_64-pc-windows-gnu`, `git diff --check`, scans estáticos de placeholders/panics/unwraps/unsafe e smoke test headless sob Xvfb.

O estado validado passou com 12 testes unitários e 10 casos de integração. O advisory scan não encontrou vulnerabilidades; permanecem somente os avisos transitivos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`.

## Limitações conhecidas

A validação visual e funcional manual em Windows 10/11 real continua necessária para clipboard, drag and drop, diálogos nativos, escalas DPI, múltiplos monitores e acessibilidade. Arquivos próximos do limite de 128 MB continuam podendo exigir memória adicional durante a edição e o salvamento.

## Artefatos

O release inclui pacote Windows x86-64 e checksum SHA-256 correspondente.
