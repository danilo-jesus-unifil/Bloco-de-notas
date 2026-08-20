# Plano de complemento — requisitos visuais e de uso

## Diagnóstico

O projeto já possui uma base Rust modular com `egui/eframe`, tema escuro, edição Unicode, operações de arquivo, Undo/Redo, clipboard, localizar/substituir, manifesto DPI, ícone Windows, testes e release Windows x86-64. O novo prompt complementa essa base com uma experiência mais próxima do Bloco de Notas do Windows 11, mas mantém a regra de não transformar o aplicativo em um editor complexo.

| Requisito complementar | Estado atual | Decisão |
|---|---|---|
| Barra superior moderna | Menus funcionais, sem abas ou painel de configuração | Adicionar cabeçalho compacto com ícone textual, abas simples e configurações mínimas |
| Abas | Ausentes | Implementar abas locais, sem colaboração, plugins ou recursos avançados |
| Drag and drop | Ausente | Usar os eventos nativos já expostos pelo eframe e abrir o primeiro arquivo de texto soltado |
| Barra de status | Mostra status, caracteres e dirty state | Acrescentar linha, coluna, zoom, finais de linha e encoding; manter opção de ocultação |
| Zoom | O tamanho da fonte pode ser alterado por menu | Centralizar como zoom com atalhos `Ctrl++`, `Ctrl+-` e `Ctrl+0` |
| Configurações | Dispersas no menu Exibir | Criar painel pequeno para tema, fonte/tamanho, quebra de linha, status e comportamento de abertura |
| Persistência | Ausente | Persistir somente preferências mínimas via armazenamento do eframe; conteúdo de documentos não será persistido secretamente |
| Formatos | Filtro prioriza `.txt` | Aceitar `.txt`, `.md`, `.log`, `.csv`, `.ini` e `.json`, sem parsing específico |
| Finais de linha | Normalizados para `LF` sem guardar a preferência | Guardar o formato detectado e gravar de modo previsível |
| Visual Windows 11 | Tema escuro discreto | Refinar espaçamento, arredondamento e hierarquia sem copiar componentes exclusivos do Windows 11 |

## Limites deliberados

A nova versão não implementará uma janela secundária, recuperação de abas fechadas, múltiplos painéis, editor rico, seleção de família tipográfica, tema claro/system completo, instalador ou árvore de arquivos. Esses itens não são necessários para complementar o fluxo simples e aumentariam o risco de regressão.

O comportamento de abertura será uma preferência simples: abrir o arquivo na aba atual quando ela estiver vazia e, opcionalmente, abrir em uma nova aba. A preferência de tema será apresentada com Dark como opção ativa e as alternativas futuras claramente desabilitadas, evitando fingir suporte que ainda não existe.

## Ordem de implementação

A alteração será feita em unidades pequenas: primeiro modelo de abas e metadados de documento; depois arquivos e finais de linha; em seguida comandos, atalhos e drag-and-drop; depois cabeçalho, painel de configurações e status bar; por fim persistência, testes e documentação. Cada unidade deve passar por formatação, compilação e testes antes do próximo passo.
