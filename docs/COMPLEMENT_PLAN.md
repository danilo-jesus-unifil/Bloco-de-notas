# Decisões de escopo

Este documento registra as decisões que orientaram a interface do projeto. O aplicativo continua sendo um bloco de notas simples para Windows 10 e 11, com tema escuro, edição Unicode e foco em abrir, editar, localizar, salvar e fechar arquivos de texto.

## O que está incluído

A interface tem cabeçalho compacto, abas locais, botão de nova aba, menus, painel de configurações, barra de status, zoom e arrastar e soltar arquivos de texto. Os formatos priorizados são `.txt`, `.md`, `.log`, `.csv`, `.ini` e `.json`; o aplicativo edita o texto e não interpreta esses formatos.

As preferências mínimas e a geometria da janela são persistidas pelo eframe. O conteúdo dos documentos não é salvo automaticamente. Finais de linha `LF`, `CRLF` e `CR` são detectados, mantidos e gravados de forma previsível.

O tema escuro é o único tema ativo. As opções Claro e Seguir o sistema aparecem desabilitadas para não sugerir um suporte que ainda não existe.

## O que fica fora do escopo

O projeto não inclui nova janela, recuperação de abas fechadas, múltiplos painéis, editor rico, família tipográfica configurável, tema claro completo, instalador, árvore de arquivos, plugins ou sincronização. Esses recursos não são necessários para o fluxo básico e aumentariam a superfície de manutenção.

A acessibilidade nativa via AccessKit também permanece desativada enquanto não houver uma versão do framework compatível com a interface atual e sem a cadeia de dependências problemática já registrada no histórico do projeto.

## Critério para novas mudanças

Uma mudança só deve entrar se melhorar diretamente a edição, a abertura, o salvamento, a navegação ou o fechamento de arquivos de texto. O código deve continuar simples, modular e compatível com Windows 10 e 11. Recursos maiores devem ser tratados como uma decisão nova, não como extensão automática deste plano.
