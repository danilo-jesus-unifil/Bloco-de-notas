# Bloco de notas

Um bloco de notas simples, rápido e predominantemente escuro, escrito em Rust com `egui/eframe`. O projeto foi desenhado para manter a experiência direta de um editor de texto básico: abrir um arquivo de texto, digitar, localizar, salvar e fechar sem a complexidade de um editor de programação.

## Escopo entregue

A versão `0.2.2` mantém a experiência visual mais próxima do Bloco de Notas do Windows 11 sem abandonar a compatibilidade com Windows 10. A janela possui cabeçalho compacto com ícone, abas locais, botão de nova aba, menus familiares, painel de configurações mínimo, status bar detalhada e drag and drop de arquivos de texto.

O aplicativo inclui edição Unicode, quebra automática de linha, zoom por tamanho da fonte, novo documento, abrir, salvar, salvar como, confirmação para alterações não salvas, desfazer, refazer, copiar, recortar, colar, selecionar tudo, localizar, localizar anterior/próximo, substituir e substituição em massa. Os atalhos principais incluem `Ctrl+N`, `Ctrl+O`, `Ctrl+S`, `Ctrl+Shift+S`, `Ctrl+W`, `Ctrl+Z`, `Ctrl+Y`, `Ctrl+X`, `Ctrl+C`, `Ctrl+V`, `Ctrl+A`, `Ctrl+F`, `Ctrl+H`, `Ctrl++`, `Ctrl+-` e `Ctrl+0`.

Cada aba mantém seu documento, caminho, estado de alteração, histórico de edição, cursor persistido pelo identificador do editor e preferências de visualização. O comportamento de abertura pode ser configurado para usar uma nova aba. Fechar uma aba ou sair verifica alterações não salvas sem descartar conteúdo silenciosamente.

## Arquivos e encoding

Os formatos de texto priorizados são `.txt`, `.md`, `.log`, `.csv`, `.ini` e `.json`; o aplicativo não interpreta esses formatos, apenas edita texto. Arquivos são lidos como UTF-8, com suporte a BOM UTF-8 e mensagens controladas para bytes inválidos. Finais de linha `CRLF`, `LF` e `CR` são detectados, normalizados internamente e serializados novamente de forma previsível. O salvamento usa arquivo temporário no mesmo diretório, `flush`, `sync_all` e substituição controlada para reduzir o risco de deixar o arquivo original parcialmente gravado.

Para evitar consumo de memória imprevisível, a abertura recusa arquivos maiores que 128 MB e informa o limite ao usuário antes de ler o conteúdo. Essa é uma proteção deliberada para um editor simples que mantém o documento em memória.

## Configurações persistentes

O tema escuro é o padrão. O painel de configurações persiste somente preferências mínimas por meio do armazenamento do eframe em local apropriado ao sistema: tamanho da fonte, quebra automática, visibilidade da barra de status, comportamento de abertura e modo de tema. As opções Claro e Seguir o sistema são apresentadas como futuras e permanecem desabilitadas nesta versão, sem simular suporte incompleto. O tamanho e a posição da janela também podem ser persistidos pelo eframe.

O recurso opcional `accesskit` do eframe permanece desativado nesta versão para não incluir a cadeia Linux de acessibilidade que trazia vulnerabilidades altas em `quick-xml 0.30.0`. O teclado e os atalhos continuam sendo tratados pela interface, e a retomada de acessibilidade nativa deverá ocorrer junto com uma versão atualizada e compatível do framework.

## Estrutura

| Caminho | Responsabilidade |
|---|---|
| `src/main.rs` | Inicialização da janela, DPI, persistência de geometria e entrada |
| `src/app/` | Estado geral, abas, comandos, configurações, drag and drop e ciclo de vida |
| `src/document/` | Conteúdo, caminho, dirty state, finais de linha e Undo/Redo |
| `src/editor/` | Busca e substituição determinísticas |
| `src/file_io/` | Leitura UTF-8, detecção de finais de linha e gravação temporária |
| `src/ui/` | Cabeçalho, abas, menus, editor, configurações, busca e status bar |
| `src/theme/` | Paleta escura semântica |
| `src/error/` | Erros técnicos e mensagens compreensíveis |
| `src/commands/` | Vocabulário central de ações |
| `assets/` | Manifesto DPI e ícone Windows |
| `docs/DEVELOPMENT_PLAYBOOK.md` | Guia interno de Git e de projeto |
| `docs/COMPLEMENT_PLAN.md` | Análise e limites do complemento |
| `docs/AUDIT_FINDINGS.md` | Registro da auditoria, correções e limitações |

## Desenvolvimento

É necessário ter Rust estável instalado. No diretório do projeto, execute:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
cargo build --release
```

Para executar localmente:

```bash
cargo run
```

Para gerar o executável Windows x86-64 no ambiente GNU:

```bash
rustup target add x86_64-pc-windows-gnu
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
  cargo build --release --target x86_64-pc-windows-gnu
```

O executável cross-compiled fica em `target/x86_64-pc-windows-gnu/release/bloco-de-notas.exe`. Em um ambiente Windows com o toolchain MSVC, o comando usual é `cargo build --release`, produzindo o arquivo em `target/release/bloco-de-notas.exe`.

O `build.rs` usa `winres` somente em builds Windows para incorporar o manifesto DPI-aware e o ícone multi-resolução. A aplicação não depende de arquivos presentes na máquina de desenvolvimento para iniciar.

## Distribuição

O diretório `dist/` é reservado para artefatos gerados e não é commitado. Um pacote Windows contém o executável e um README curto de distribuição. O release oficial acompanha um arquivo de checksum SHA-256.

## Limitações conhecidas da validação

O ambiente de desenvolvimento desta entrega é Linux. Os testes automatizados, a análise estática, o advisory scan e o cross-build PE validam a lógica Rust, o formato do executável Windows e a integração de recursos, mas não substituem a execução manual em Windows 10/11. Antes de distribuir amplamente, valide o arquivo final em Windows 10 e 11 com escalas de 100%, 125%, 150%, 175% e 200%, incluindo abertura, salvamento, clipboard, drag and drop, abas, menus, fechamento com alterações e movimento entre monitores com DPI diferente.

## Licença

Este projeto é distribuído sob a licença MIT. Consulte [`LICENSE`](LICENSE).
