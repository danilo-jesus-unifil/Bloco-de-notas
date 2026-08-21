# Release v0.2.5

## O que mudou

O cross-build Windows agora incorpora corretamente os recursos do executável quando o build é realizado em Linux para o target `x86_64-pc-windows-gnu`.

O build script deixou de depender de `#[cfg(windows)]`, que descrevia o sistema do host em vez do target Windows. Ele passa a detectar o target por `TARGET`, configura `x86_64-w64-mingw32-windres` e `x86_64-w64-mingw32-ar`, e liga diretamente o objeto de recursos ao binário GNU. Essa ligação direta evita que o linker descarte a seção de recursos por ela não possuir símbolos referenciados pelo código Rust.

O executável v0.2.5 contém o manifesto DPI-aware, a versão Windows `0.2.5.0`, o ícone multi-resolução e o grupo de ícone. O pacote também fornece o arquivo `bloco-de-notas.ico` separadamente para criação manual de atalhos no Windows.

## Verificações

Foram executados `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo audit --no-fetch`, `cargo build --release`, cross-build release Windows `x86_64-pc-windows-gnu`, inspeção dos recursos PE com `wrestool`, `git diff --check` e execução controlada do executável sob Wine.

A suíte passou com 12 testes unitários e 10 casos de integração. O advisory scan não encontrou vulnerabilidades; permanecem os avisos transitivos de manutenção para `paste 1.0.15` e `ttf-parser 0.25.1`.

O executável antigo e o candidato corrigido permaneceram ativos sob Wine até o timeout controlado. A confirmação final em Windows 10/11 real continua necessária para validar SmartScreen/Defender, drivers OpenGL, diálogos nativos, DPI, clipboard e drag and drop.

## Artefatos

O release inclui o pacote Windows x86-64, o arquivo `.ico` e os checksums SHA-256 correspondentes.
