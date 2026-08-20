# Changelog

Todas as mudanças relevantes do projeto são registradas neste arquivo.

## [0.2.4] — 2026-08-20

A auditoria repetida simplificou o ciclo de encerramento para manter uma única fonte de verdade sobre a confirmação de alterações não salvas.

### Corrigido

- O comando Sair agora apenas solicita o fechamento da viewport.
- A confirmação de abas não salvas fica centralizada no evento real `close_requested`, cobrindo menu, atalhos e botão externo sem estado auxiliar temporário.

### Auditoria

Foram repetidos os testes unitários e de integração, Clippy, advisory scan, builds Linux e Windows, scans estáticos, `git diff --check` e smoke test headless. Não foram encontradas novas vulnerabilidades; permanecem apenas os avisos transitivos de manutenção já documentados.

## [0.2.3] — 2026-08-20

A auditoria completa refinou feedback, desempenho e robustez de I/O sem ampliar o escopo do bloco de notas.

### Corrigido

- O status interno passou a ser exibido na barra inferior, e falhas passaram a abrir diálogo contextualizado.
- O botão `+` passou a criar diretamente uma nova aba; a opção de janela separada foi identificada como futura.
- A contagem de caracteres passou a ser mantida pelo documento, com cache de linha e coluna por aba, cursor e geração de conteúdo.
- A abertura de arquivos reduziu cópias evitáveis, revalidou o limite de 128 MB e rejeitou UTF-8 inválido com teste de regressão.
- O fechamento deixou de depender de autorização temporária entre o comando e o evento da viewport.

## [0.2.2] — 2026-08-20

Atualização corretiva derivada de investigação adicional de comportamentos não cobertos pelas auditorias anteriores.

### Corrigido

- O estado não salvo passou a acompanhar revisões do documento, incluindo Undo/Redo até o conteúdo salvo.
- Cancelar Salvar como durante o fechamento deixou de ser interpretado como salvamento bem-sucedido.
- Cada aba recebeu identificador estável para preservar cursor e seleção ao fechar abas anteriores.
- Atalhos de edição passaram a respeitar o foco nos campos de localizar e substituir.
- Localizar anterior e Substituir passaram a respeitar a ocorrência selecionada.

## [0.2.1] — 2026-08-20

A auditoria completa corrigiu falhas sutis de Unicode, endureceu o fluxo de arquivos e revisou a cadeia de dependências sem ampliar o escopo do aplicativo.

### Corrigido

- Busca reversa e substituição agora usam índices de caracteres seguros para conteúdo Unicode, com regressões para acentos e emoji.
- Arquivos acima de 128 MB são recusados antes da leitura integral para evitar consumo de memória imprevisível.
- Salvamento em Windows usa `ReplaceFileW` isolado e documentado; o caminho portátil não remove o original quando a substituição falha.
- A cadeia opcional Linux de `accesskit` foi desativada no eframe 0.29.1 para eliminar as vulnerabilidades altas reportadas em `quick-xml 0.30.0`.

### Auditoria

`cargo fmt --check`, `cargo check`, `cargo test`, Clippy sem warnings, `cargo audit`, build release Linux, check/build target Windows, smoke test headless sob Xvfb e `git diff --check` foram executados. O advisory scan não reporta vulnerabilidades, mas ainda registra os avisos de manutenção transitivos de `paste 1.0.15` e `ttf-parser 0.25.1`.

## [0.2.0] — 2026-08-20

A versão complementar aproxima a interface da experiência visual do Bloco de Notas do Windows 11, mantendo a simplicidade e a compatibilidade com Windows 10.

### Adicionado

- Abas locais com nome, conteúdo, caminho, estado modificado e cursor persistido por aba.
- Nova aba, fechamento de aba, `Ctrl+W` e proteção contra perda de alterações.
- Drag and drop de arquivos `.txt`, `.md`, `.log`, `.csv`, `.ini` e `.json`.
- Barra de status com linha, coluna, caracteres, zoom, finais de linha, encoding e estado de salvamento.
- Zoom por `Ctrl++`, `Ctrl+-` e `Ctrl+0`.
- Painel pequeno de configurações para tema, fonte, quebra de linha, status e comportamento de abertura.
- Persistência de preferências e geometria da janela pelo armazenamento do eframe.
- Detecção e preservação previsível de finais `LF`, `CRLF` e `CR`.
- Cabeçalho compacto com ícone, abas e ação de nova aba.
- Testes adicionais de encoding, finais de linha e integração de salvar/reabrir.

### Mantido deliberadamente fora do escopo

Temas Claro e Seguir o sistema, família tipográfica configurável, recuperação de abas fechadas, nova janela, instalador, plugins, edição rica, syntax highlighting, múltiplos painéis e sincronização continuam reservados para versões futuras, se forem necessários.

## [0.1.0] — 2026-08-20

Primeiro release público com edição de texto Unicode, operações de arquivo, Undo/Redo, clipboard, localizar/substituir, tema escuro, manifesto DPI-aware, ícone Windows e build x86-64.
