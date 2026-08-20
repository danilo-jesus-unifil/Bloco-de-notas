# Bloco de notas

Um bloco de notas simples, rápido e predominantemente escuro, escrito em Rust com `egui/eframe`. O projeto foi desenhado para manter a experiência direta de um editor de texto básico: abrir um arquivo `.txt`, digitar, localizar, salvar e fechar sem a complexidade de um editor de programação.

## Escopo entregue

A versão `0.1.0` inclui uma janela desktop redimensionável, tema escuro, edição de texto Unicode, quebra automática de linha, ajuste limitado de fonte, menus familiares, atalhos de teclado, novo, abrir, salvar, salvar como, confirmação para alterações não salvas, desfazer, refazer, copiar, recortar, colar, selecionar tudo, localizar, substituir e substituição em massa.

O modelo de documento é separado da interface, mantém uma única fonte de verdade para o conteúdo e limita o histórico a 128 estados. Arquivos são lidos como UTF-8, com suporte a BOM UTF-8; finais de linha `CRLF` e `CR` são normalizados para `LF` de maneira previsível. A gravação usa um arquivo temporário no mesmo diretório, `flush`, `sync_all` e substituição controlada para reduzir o risco de deixar o arquivo original parcialmente gravado.

A integração de recursos do Windows fica isolada no `build.rs` e nos assets. O manifesto declara DPI awareness, incluindo `PerMonitorV2`, e o build Windows incorpora o ícone multi-resolução por meio do recurso nativo da plataforma.

## Estrutura

| Caminho | Responsabilidade |
|---|---|
| `src/main.rs` | Inicialização da janela e entrada do aplicativo |
| `src/app/` | Estado geral, comandos, diálogos e ciclo de vida |
| `src/document/` | Conteúdo, caminho, dirty state e Undo/Redo |
| `src/editor/` | Busca e substituição determinísticas |
| `src/file_io/` | Leitura UTF-8 e gravação temporária controlada |
| `src/ui/` | Menus, editor, painel de busca e barra de status |
| `src/theme/` | Paleta escura semântica |
| `src/error/` | Erros técnicos e mensagens compreensíveis |
| `src/commands/` | Vocabulário central de ações |
| `assets/` | Manifesto DPI e ícone Windows |
| `docs/DEVELOPMENT_PLAYBOOK.md` | Guia interno de Git e de projeto |

## Desenvolvimento

É necessário ter Rust estável instalado. No diretório do projeto, execute:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
```

Para executar localmente:

```bash
cargo run
```

Para criar um build Windows com os recursos nativos, use um ambiente Windows com o toolchain MSVC ou GNU escolhido para a distribuição. O `build.rs` usa `winres` somente em builds Windows; no Linux, os testes de lógica e o build de desenvolvimento continuam independentes dessa etapa.

## Limitações conhecidas da validação

O ambiente de desenvolvimento desta entrega é Linux. Portanto, os testes automatizados e o build local validam a lógica Rust e a integração de desenvolvimento da GUI, mas não substituem a execução manual em Windows 10. Antes de distribuir um executável, valide o arquivo final em Windows 10 com escalas de 100%, 125%, 150%, 175% e 200%, incluindo abertura, salvamento, clipboard, menus, fechamento com alterações e movimentação entre monitores com DPI diferente.

## Licença

Este projeto é distribuído sob a licença MIT. Consulte [`LICENSE`](LICENSE).
