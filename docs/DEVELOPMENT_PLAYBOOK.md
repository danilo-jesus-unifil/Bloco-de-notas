# Guia interno de boas práticas de Git e de projeto

> **Propósito.** Este documento é um contrato de execução para o projeto **Bloco de notas**. Deve ser relido antes de cada categoria relevante, antes de qualquer commit e antes do release. Seu objetivo é manter o escopo pequeno, o código compreensível e o histórico de mudanças confiável.

## 1. Princípios permanentes

O produto é um bloco de notas simples para Windows 10, escrito em Rust, com aparência predominantemente escura, baixo consumo de recursos e comportamento previsível. A prioridade é **confiabilidade antes de quantidade de recursos**. Qualquer ideia que aproxime o projeto de um IDE, editor rico ou plataforma extensível deve ser recusada nesta versão, salvo se for indispensável para o fluxo básico de abrir, editar, salvar e fechar texto simples.

Cada alteração deve responder afirmativamente a três perguntas: ela funciona no Windows 10, respeita a arquitetura atual e mantém o produto simples? Se alguma resposta for negativa, a alteração deve ser simplificada, isolada por fallback ou adiada.

A implementação será feita em categorias sequenciais. Uma categoria só termina quando estiver compilável, testada no que for aplicável, sem erro conhecido relevante, com a responsabilidade no módulo correto e sem complexidade desnecessária.

## 2. Regras de Git

O branch `main` deve permanecer sempre em estado compilável. O trabalho deve ser dividido em commits pequenos, coerentes e reversíveis. Cada commit deve representar uma intenção única: fundação, documento, editor, arquivos, comandos, tema, testes ou release. Não misturar refatoração ampla, mudança visual e correção não relacionada no mesmo commit.

Antes de começar uma mudança, verificar o estado do repositório e confirmar que não existem alterações alheias. Ao concluir uma unidade de trabalho, executar as verificações adequadas, revisar o diff e só então criar o commit. Nunca usar `git add .` sem revisar os arquivos que serão incluídos; preferir adicionar explicitamente os caminhos pertinentes.

As mensagens de commit devem usar o padrão abaixo, em português ou inglês consistente com o histórico, com verbo no imperativo e escopo claro:

```text
<tipo>(<escopo>): <intenção objetiva>
```

| Tipo | Uso | Exemplo |
|---|---|---|
| `feat` | Nova capacidade observável | `feat(editor): add document editing state` |
| `fix` | Correção de comportamento | `fix(file-io): preserve document on save failure` |
| `refactor` | Mudança estrutural sem alterar o comportamento esperado | `refactor(ui): isolate theme palette` |
| `test` | Inclusão ou ajuste de testes | `test(document): cover undo and redo` |
| `docs` | Documentação e guias | `docs(project): add development playbook` |
| `chore` | Manutenção de build, dependências ou automação | `chore(build): configure release profile` |
| `release` | Preparação de uma versão publicada | `release: v0.1.0` |

Não reescrever o histórico remoto, não fazer force push em `main` e não criar tags que apontem para conteúdo diferente depois de publicadas. Tags de release devem ser semânticas, anotadas e imutáveis na prática, por exemplo `v0.1.0`.

## 3. Fluxo de trabalho por categoria

Para cada categoria, seguir exatamente o ciclo: inspecionar estado atual; identificar interfaces e arquivos relevantes; escrever um plano pequeno; implementar a menor mudança suficiente; formatar; compilar; testar; corrigir; revisar arquitetura e desempenho; simplificar; revisar o diff; criar commit.

Se uma categoria revelar defeito em uma categoria anterior, interromper o avanço, corrigir a causa e executar novamente as verificações da categoria afetada. Não acumular dívida conhecida sob a justificativa de corrigir tudo no final.

O contexto deve ser carregado por módulo. Antes de editar, ler primeiro `Cargo.toml`, o módulo de entrada e apenas as interfaces diretamente envolvidas. Se um arquivo central crescer demais ou assumir responsabilidades diferentes, dividi-lo antes de adicionar mais comportamento.

## 4. Arquitetura e limites de responsabilidade

O ponto de entrada deve permanecer pequeno. A lógica deve ser organizada por responsabilidade real, com separação entre estado, documento, edição, I/O, comandos, interface, tema, erros e integração específica do Windows. APIs de plataforma devem ficar isoladas em módulos próprios e, quando houver `unsafe`, este deve ser pequeno, documentado e exposto por uma interface segura.

O estado do documento deve ter uma única fonte de verdade para conteúdo, caminho, alteração não salva, histórico e configurações essenciais. A interface não deve duplicar esse estado nem declarar que uma operação terminou antes do resultado real da operação.

Preferir tipos simples, `Result` e `Option`, funções curtas e interfaces explícitas. Evitar `unwrap()` e `expect()` em operações normais de arquivos, configuração, clipboard, parsing e sistema. Evitar abstrações genéricas sem benefício comprovado, barramentos de eventos complexos, sistemas de plugins, injeção de dependência e dependências pesadas para tarefas pequenas.

## 5. Critérios técnicos de qualidade

A cada unidade relevante, executar o subconjunto aplicável da seguinte matriz:

| Verificação | Objetivo |
|---|---|
| `cargo fmt --check` | Garantir formatação determinística |
| `cargo check` | Verificar compilação rápida |
| `cargo test` | Validar lógica determinística e regressões |
| `cargo clippy --all-targets --all-features -- -D warnings` | Encontrar problemas idiomáticos e warnings relevantes |
| `cargo build --release` | Confirmar o build otimizado |
| Revisão de `git diff --check` | Detectar espaços problemáticos e ruído no diff |
| Revisão manual do escopo | Confirmar que a mudança continua sendo de um bloco de notas simples |

Se o ambiente Linux não puder validar o executável Windows, registrar explicitamente essa limitação e validar ao menos o alvo de compilação, o manifesto, os recursos e os testes que não dependem da GUI. Não declarar compatibilidade real com Windows 10 apenas porque o código compilou em outro sistema.

## 6. Dependências, segurança e desempenho

Toda dependência nova deve justificar sua existência no diff ou na documentação de arquitetura. Avaliar manutenção, licença, compatibilidade com Windows 10, tamanho do binário, tempo de compilação e sobreposição com dependências existentes. Remover dependências sem uso e evitar introduzir uma biblioteca grande para uma operação simples.

Falhas de arquivo, caminhos inválidos, acesso negado, arquivo somente leitura, falha de leitura, falha de gravação e espaço insuficiente devem resultar em erro controlado e mensagem compreensível, sem `panic` e sem perda silenciosa de conteúdo. Salvar deve confirmar sucesso antes de limpar o estado de alteração e, quando apropriado, deve usar escrita temporária e substituição segura.

Não manter timers, polling ou atualizações contínuas sem necessidade. Medir antes de otimizar. Priorizar resposta durante digitação, rolagem, abertura e salvamento, sem introduzir threads, filas ou cópias completas do documento onde uma solução simples for suficiente.

## 7. Checklist antes de cada commit

Antes de criar o commit, reler a seção relevante deste documento e confirmar que o diff tem uma intenção única. Verificar o estado do branch, arquivos não rastreados, testes afetados, mensagens de erro, `unwrap`/`expect`, `unsafe`, dependências novas, tamanho dos arquivos e qualquer recurso que aumente o escopo sem necessidade.

Em seguida, executar as verificações necessárias, inspecionar o diff com `git diff --stat` e `git diff`, confirmar que nenhum segredo, artefato de build ou arquivo local foi incluído e registrar no commit o motivo da alteração, não apenas o que foi editado.

## 8. Checklist de release

O release só pode ser criado depois de `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy` e `cargo build --release`, além da revisão arquitetural e de dependências. Deve existir uma nota de release com versão, mudanças entregues, verificações executadas, limitações conhecidas e instruções de uso.

A tag deve apontar para o commit final validado. O release do GitHub deve incluir o código-fonte gerado pela plataforma e, quando o ambiente de build produzir um artefato distribuível compatível com o alvo, o arquivo compactado correspondente. Se o executável Windows não puder ser gerado ou validado neste ambiente, declarar isso claramente em vez de publicar um artefato enganoso.

## 9. Regra de encerramento

Antes de concluir, responder honestamente:

1. **Funciona corretamente?**
2. **Está mais complexo do que precisa estar?**
3. **Ainda parece um excelente bloco de notas simples?**

Se alguma resposta for negativa, corrigir antes do release.

## Referências

[1]: https://git-scm.com/docs/git-commit "Git documentation: git-commit"
[2]: https://git-scm.com/docs/git-tag "Git documentation: git-tag"
[3]: https://doc.rust-lang.org/cargo/commands/cargo-test.html "The Cargo Book: cargo test"
[4]: https://doc.rust-lang.org/cargo/commands/cargo-release.html "The Cargo Book: release profiles"
[5]: https://rust-lang.github.io/rust-clippy/master/ "Clippy documentation"
