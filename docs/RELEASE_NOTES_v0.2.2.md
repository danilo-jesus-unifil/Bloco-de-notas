# Release v0.2.2

## Resumo

A versão `v0.2.2` é uma atualização corretiva derivada de uma investigação adicional de comportamentos não cobertos pelas auditorias anteriores. O objetivo foi melhorar a confiabilidade do fluxo básico de editar, localizar, salvar, desfazer e fechar, sem ampliar o escopo do aplicativo.

## Correções

- O estado **não salvo** agora acompanha revisões do documento. Desfazer até o conteúdo salvo remove corretamente o marcador de alteração; refazer volta a marcar o documento como alterado.
- Cancelar a caixa **Salvar como…** durante o fechamento de uma aba ou do aplicativo não é mais interpretado como salvamento bem-sucedido. O conteúdo permanece aberto e a operação de fechamento é cancelada.
- Cada aba passou a ter um identificador estável para o editor. Fechar uma aba anterior não desloca os IDs de cursor e seleção das abas remanescentes.
- Atalhos de edição como `Ctrl+Z`, `Ctrl+Y`, `Ctrl+X`, `Ctrl+C`, `Ctrl+V` e `Ctrl+A` não são mais interceptados pelo documento quando o foco está nos campos de localizar ou substituir.
- **Localizar anterior** e **Substituir** respeitam a ocorrência atualmente selecionada, evitando repetir a mesma correspondência ou substituir uma ocorrência diferente da destacada.
- O manifesto Windows foi atualizado para a versão `0.2.2.0`.

## Validações executadas

- `cargo fmt --check`
- `cargo check`
- `cargo test`: 11 testes unitários e 8 casos de integração aprovados
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo audit --no-fetch`: nenhuma vulnerabilidade; permanecem apenas os avisos transitivos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`
- `cargo build --release`
- Build release Windows `x86_64-pc-windows-gnu`, gerando executável PE32+ x86-64
- `git diff --check` e scans estáticos de placeholders, panics, unwraps e `unsafe`
- Smoke test headless do binário Linux sob Xvfb por 10 segundos, com timeout controlado esperado

## Limitações conhecidas

A execução visual manual em Windows 10/11 real continua necessária para validar diálogos nativos, clipboard, drag and drop, escalas DPI, múltiplos monitores e acessibilidade. Arquivos próximos do limite de 128 MB podem exigir memória adicional durante normalização, histórico e salvamento, porque o editor mantém o documento em memória.

## Artefatos

O release distribuível contém o executável Windows x86-64 e o checksum SHA-256 correspondente.
