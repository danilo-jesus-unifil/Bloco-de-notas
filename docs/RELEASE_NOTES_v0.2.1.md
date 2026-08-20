# Bloco de notas v0.2.1

## Resumo

Esta é uma versão de correção após uma auditoria completa do projeto. O escopo permanece o de um bloco de notas simples para Windows 10, mas os fluxos de Unicode, abertura e salvamento foram revisados para reduzir riscos funcionais, de memória e de perda de dados.

## O que foi encontrado e corrigido

A auditoria identificou que a busca reversa e alguns caminhos de substituição poderiam misturar índices de bytes com índices de caracteres. Isso era silencioso em ASCII e podia produzir seleções erradas em acentos ou emoji. A implementação agora centraliza a conversão e possui regressões específicas para conteúdo Unicode.

A leitura de arquivos era integral por desenho, mas não possuía limite. A abertura agora consulta metadados antes de alocar e recusa arquivos acima de 128 MB com uma mensagem clara. Esse limite é adequado ao modelo de editor que mantém o documento em memória e evita consumo imprevisível em arquivos acidentalmente enormes.

No Windows, a substituição de um arquivo existente passou a usar `ReplaceFileW` através de uma pequena função com FFI isolado e comentário de segurança. No caminho portátil, o salvamento não remove mais o original depois de uma falha de rename. O arquivo temporário continua sendo criado no mesmo diretório, recebe flush e sincronização antes da substituição.

A auditoria de dependências encontrou duas vulnerabilidades altas em `quick-xml 0.30.0`, trazido pela cadeia opcional Linux de acessibilidade do eframe. O recurso `accesskit` foi desativado nesta versão para remover essa cadeia vulnerável sem migrar todo o código para eframe 0.36, que tem APIs incompatíveis com a interface atual. O advisory scan final não reporta vulnerabilidades, mas registra avisos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`, dependências transitivas do framework e sem substituição direta compatível com eframe 0.29.1.

## Verificações executadas

| Verificação | Resultado |
|---|---|
| `cargo fmt --check` | Passou |
| `cargo check` | Passou |
| `cargo test` | Passou: 10 testes unitários e 7 testes no harness de integração |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passou sem warnings |
| `cargo audit` | Passou sem vulnerabilidades; dois avisos transitivos de manutenção permanecem |
| `cargo build --release` | Passou |
| `cargo check --target x86_64-pc-windows-gnu` | Passou |
| `git diff --check` | Passou |
| Smoke test `xvfb-run` do binário release | Iniciou sem erro observável e foi encerrado pelo timeout controlado |

## Limitações restantes

A execução manual em Windows 10/11 real não está disponível no ambiente de auditoria. Ainda é necessário validar visualmente DPI, menus nativos, clipboard, drag and drop, persistência, diálogos de confirmação e arquivos somente leitura em uma máquina Windows. A acessibilidade nativa via AccessKit está explicitamente desativada até que uma versão atualizada do eframe possa ser adotada sem introduzir uma migração desproporcional.
