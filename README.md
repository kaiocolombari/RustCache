# Mecanismo de Cache Concorrente

Um mecanismo de cache genérico e seguro para uso em ambientes multithread (thread-safe), desenvolvido em Rust, projetado para explorar o gerenciamento de memória, o compartilhamento de dados e o processamento concorrente.

O projeto visa implementar um cache capaz de lidar com solicitações simultâneas provenientes de múltiplas threads, reutilizando resultados calculados anteriormente e evitando operações redundantes.

## Funcionalidades

- Armazenamento genérico de chave-valor.
- Capacidade máxima configurável.
- Estratégia de remoção (evicção) FIFO (*First In, First Out* — primeiro a entrar, primeiro a sair).
- Prevenção contra chaves duplicadas.
- Recuperação de valores baseada em referências.
- Inserção e remoção de elementos.
- Limpeza do cache e verificação de tamanho.
- Acesso compartilhado seguro para threads (*thread-safe*).
- Processamento concorrente de solicitações.
- Prevenção contra cálculos redundantes.

## Tecnologias

- **Rust** — Linguagem principal.
- **Biblioteca Padrão (`std`)** — Funcionalidades fundamentais.
- **Arc** — Propriedade compartilhada entre threads.
- **Mutex / RwLock** — Sincronização entre threads.
- **HashMap / Vec** — Estruturas para armazenamento de dados.

Não são necessárias dependências externas.

## Arquitetura do Projeto

```text
cacheRust/
├── src/
│   ├── main.rs
│   ├── cache.rs
│   ├── user.rs
│   ├── worker.rs
│   └── metrics.rs
└── Cargo.toml
```

## Conceitos Principais

Este projeto explora diversos conceitos da linguagem Rust por meio de uma implementação prática:

- **Propriedade e Empréstimo (*Ownership & Borrowing*):** Gerenciamento da propriedade de dados e referências, evitando cópias desnecessárias.
- **Genéricos e *Traits*:** Construção de estruturas de dados reutilizáveis ​​com restrições de tipo.
- **Segurança de Memória:** Compreensão de referências, tempos de vida (*lifetimes*) e acesso seguro à memória.
- **Concorrência:** Compartilhamento de dados entre threads, prevenindo condições de corrida (*race conditions*).
- **Sincronização:** Coordenação de operações simultâneas de leitura e escrita.
- **Segurança em *Threads* (*Thread Safety*):** Exploração de `Send`, `Sync`, `Arc`, `Mutex` e `RwLock`.
- **Desempenho:** Avaliação de estratégias de cache e redução de cálculos redundantes.

## Comportamento Esperado

O cache será compartilhado entre múltiplas threads de processamento (*worker threads*).

Quando várias threads solicitam o mesmo recurso, o sistema deve:

1. Verificar se o valor solicitado já existe no cache.
2. Retornar o valor armazenado no cache, caso esteja disponível. 3. Execute a operação de alto custo apenas uma vez, quando o valor estiver ausente.
4. Armazene o resultado e torne-o disponível para outras threads.

Esse comportamento visa minimizar o processamento redundante, mantendo a consistência dos dados.

## Objetivos de Aprendizado

O objetivo principal é compreender como a linguagem Rust lida com propriedade compartilhada, sincronização e segurança de memória em aplicações concorrentes.

O projeto também explora decisões práticas de design de cache, estratégias de sincronização e compromissos de desempenho em sistemas multithread.

---

**Status:** Em desenvolvimento

**Linguagem:** Rust
