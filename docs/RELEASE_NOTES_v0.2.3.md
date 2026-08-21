# Release v0.2.3

## O que mudou

Esta versão melhora o feedback da interface, o desempenho da barra de status e a abertura de arquivos.

- O status interno agora aparece na barra inferior. Erros também abrem um diálogo com contexto.
- O botão `+` cria uma nova aba. A opção de nova janela continua desabilitada e identificada como futura.
- A contagem de caracteres é mantida pelo documento. Linha e coluna usam cache por aba, cursor e geração do conteúdo.
- A abertura reutiliza o buffer quando possível, evita normalização desnecessária de arquivos LF e verifica novamente o tamanho depois da leitura.
- Arquivos com UTF-8 inválido são rejeitados sem interromper a aplicação; há teste de regressão para esse caso.

## Verificações

Foram executados `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo audit --no-fetch`, `cargo build --release`, build release Windows `x86_64-pc-windows-gnu`, `git diff --check`, scans estáticos, smoke test sob Xvfb e interação headless de teclado.

O estado validado passou com 12 testes unitários e 10 casos de integração. O advisory scan não encontrou vulnerabilidades; permanecem os avisos transitivos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`.

## Limitações

A validação manual em Windows 10/11 ainda é necessária para clipboard, drag and drop, diálogos nativos, escalas DPI, múltiplos monitores e acessibilidade. Arquivos próximos de 128 MB podem exigir memória adicional durante a edição e o salvamento.

## Artefatos

O release inclui um pacote Windows x86-64 e o checksum SHA-256 correspondente.
