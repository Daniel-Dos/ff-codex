# FF Codex

Projeto de estudos para aprender **Rust**, **Axum** e **SQLx** por meio do desenvolvimento de uma API inspirada no universo de *Final Fantasy*.

O objetivo não é construir um produto completo, mas sim praticar conceitos fundamentais da linguagem e do ecossistema Rust enquanto se constrói algo divertido e com escopo bem definido.

## Stack

| Tecnologia | Papel | Status |
|------------|-------|--------|
| [Rust](https://www.rust-lang.org/) | Linguagem principal | Implementado (scaffold) |
| [Axum](https://github.com/tokio-rs/axum) | Framework web (HTTP) | Planejado — ainda não implementado |
| [SQLx](https://github.com/launchbadge/sqlx) | Acesso a banco de dados | Planejado — ainda não implementado |

> **Atenção:** Axum e SQLx ainda **não** estão implementados. O projeto está na fase inicial de scaffold e essas dependências serão adicionadas conforme o roadmap de aprendizado avança.

## Status

O projeto está em **estágio inicial de scaffold**:

- `Cargo.toml` configurado com `edition = "2024"` e package `ff-codex`.
- `src/main.rs` contém apenas um `Hello, world!` básico.
- Nenhuma dependência externa adicionada ainda.

As próximas etapas adicionam Axum e SQLx de forma incremental, sempre com foco em aprender um conceito por vez.

## Roadmap

Etapas planejadas para o aprendizado, em ordem sugerida:

1. **Servidor HTTP com Axum**
   - Criar um endpoint `GET /health` que retorna o status da API.
   - Entender rotas, handlers e extração de parâmetros.

2. **Modelagem de dados**
   - Definir entidades do universo *Final Fantasy* (ex.: criaturas, personagens, itens).
   - Introduzir tipos e estruturas em Rust.

3. **Persistência com SQLx**
   - Conectar a um banco de dados (ex.: PostgreSQL ou SQLite).
   - Executar migrações e consultas básicas.

4. **CRUD da API**
   - Implementar operações de criação, leitura, atualização e exclusão para as entidades.
   - Praticar serialização com `serde`.

5. **Funcionalidades temáticas (ideias)**
   - **Bestiário**: listar e consultar criaturas com atributos (HP, MP, fraquezas).
   - **Personagens**: cadastro de personagens e seus jobs.
   - **Jobs**: catálogo de classes com habilidades.
   - **Itens**: inventário de itens consumíveis e equipamentos.
   - **Busca e filtros**: consultar entidades por nome, tipo ou atributo.

> As funcionalidades temáticas são ideias para guiar o aprendizado e podem mudar conforme o progresso.

## Como executar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) instalado (toolchain com suporte à `edition 2024`).
- `cargo` disponível no `PATH`.

### Passos

```bash
# Compilar o projeto
cargo build

# Executar o binário
cargo run
```

A saída esperada no estado atual é:

```
Hello, world!
```

## Estrutura do projeto

```
ff-codex/
├── Cargo.toml        # Manifesto do projeto (dependências e configuração)
├── Cargo.lock        # Versões travadas das dependências
├── src/
│   └── main.rs       # Ponto de entrada da aplicação
└── .gitignore        # Arquivos ignorados pelo Git
```

A estrutura será expandida conforme novas dependências e módulos forem adicionados.