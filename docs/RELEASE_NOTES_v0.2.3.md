# Release v0.2.3

## Resumo

A versão `v0.2.3` é uma atualização corretiva resultante da auditoria e do refinamento completo pós-v0.2.2. Ela preserva o escopo de bloco de notas simples e concentra-se em feedback visível, eficiência durante a edição, robustez de arquivos e comportamento consistente de fechamento.

## Correções e melhorias

| Área | Alteração |
|---|---|
| Feedback | O status interno agora é exibido na barra inferior; erros também aparecem em diálogo contextualizado, sem serem silenciosamente descartados. |
| Nova aba | O botão `+` agora executa diretamente a criação da aba, e a opção de janela separada é identificada como futura. |
| Barra de status | A contagem de caracteres é mantida pelo documento, e linha/coluna usam cache por aba, cursor e geração de conteúdo. |
| Arquivos grandes | A abertura reutiliza buffers quando possível, evita normalização desnecessária de arquivos LF e verifica novamente o tamanho após a leitura. |
| UTF-8 | Foi adicionada regressão para arquivo com UTF-8 inválido, confirmando rejeição controlada sem panic. |
| Fechamento | O comando Sair não repete a confirmação quando o evento de fechamento autorizado chega à viewport; fechamentos externos continuam protegidos. |

## Verificações executadas

Foram executados `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo audit --no-fetch`, `cargo build --release`, build release Windows `x86_64-pc-windows-gnu`, `git diff --check`, scans de placeholders/panics/unwraps/unsafe, smoke test headless sob Xvfb e interação headless de teclado com criação e fechamento de aba.

O estado final passou com 12 testes unitários e 10 casos de integração. O advisory scan não encontrou vulnerabilidades; permanecem apenas os avisos transitivos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`.

## Limitações conhecidas

A validação visual e funcional manual em Windows 10/11 real continua necessária para clipboard, drag and drop, diálogos nativos, escalas DPI, múltiplos monitores e acessibilidade. Arquivos próximos do limite de 128 MB continuam podendo exigir memória adicional durante a edição e o salvamento, embora a abertura tenha sido otimizada para reduzir cópias evitáveis.

## Artefatos

O release inclui pacote Windows x86-64 e checksum SHA-256 correspondente.
