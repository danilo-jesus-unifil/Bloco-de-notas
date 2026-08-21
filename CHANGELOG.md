# Histórico de mudanças

## [0.2.4] — 2026-08-20

### Corrigido

- O comando Sair agora apenas solicita o fechamento da janela.
- A confirmação de alterações não salvas fica centralizada no evento `close_requested`, sem estado temporário entre o comando e o fechamento.

### Verificações

Foram repetidos os testes, Clippy, advisory scan, builds Linux e Windows, scans estáticos, `git diff --check` e smoke test headless.

## [0.2.3] — 2026-08-20

### Corrigido

- O status interno passou a aparecer na barra inferior, e os erros passaram a abrir diálogos contextualizados.
- O botão `+` passou a criar diretamente uma nova aba; a nova janela continua fora do escopo.
- A contagem de caracteres passou a ser mantida pelo documento, com cache para linha e coluna.
- A abertura de arquivos reduziu cópias evitáveis, revalidou o limite de 128 MB e passou a rejeitar UTF-8 inválido com teste de regressão.

## [0.2.2] — 2026-08-20

### Corrigido

- Undo e redo passaram a acompanhar corretamente o conteúdo salvo e o estado de alteração.
- Cancelar Salvar como durante o fechamento preserva a aba e o conteúdo.
- Cada aba recebeu um identificador estável para cursor e seleção.
- Os atalhos de edição respeitam o foco nos campos de localizar e substituir.
- Localizar anterior e Substituir respeitam a ocorrência selecionada.

## [0.2.1] — 2026-08-20

### Corrigido

- Busca reversa e substituição passaram a converter índices de byte e caractere com segurança para Unicode.
- Arquivos acima de 128 MB são recusados antes da leitura integral.
- Salvamento no Windows usa `ReplaceFileW` em um helper isolado; o caminho portátil preserva o original quando a substituição falha.
- O recurso opcional `accesskit` foi desativado para remover a cadeia vulnerável de `quick-xml 0.30.0`.

## [0.2.0] — 2026-08-20

### Adicionado

- Abas locais, nova aba, fechamento protegido e drag and drop de arquivos de texto.
- Barra de status com linha, coluna, caracteres, zoom, finais de linha, encoding e estado de salvamento.
- Zoom por `Ctrl++`, `Ctrl+-` e `Ctrl+0`.
- Painel de configurações para fonte, quebra de linha, status e comportamento de abertura.
- Persistência de preferências e geometria da janela.
- Detecção e preservação previsível de finais `LF`, `CRLF` e `CR`.

### Fora do escopo

Temas claro e do sistema, família tipográfica configurável, recuperação de abas fechadas, nova janela, instalador, plugins, edição rica, múltiplos painéis e sincronização.

## [0.1.0] — 2026-08-20

Primeiro release público com edição Unicode, operações de arquivo, Undo/Redo, clipboard, localizar/substituir, tema escuro, manifesto DPI-aware, ícone Windows e build x86-64.
