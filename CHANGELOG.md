# Changelog

Todas as mudanças relevantes do projeto são registradas neste arquivo.

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
