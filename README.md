# FF Codex

Projeto de estudos para aprender **Rust**, **Axum** e **SQLx** por meio do desenvolvimento de uma API inspirada no universo de *Final Fantasy*.

O objetivo não é construir um produto completo, mas sim praticar conceitos fundamentais da linguagem e do ecossistema Rust enquanto se constrói algo divertido e com escopo bem definido.

## Stack

| Tecnologia | Papel | Status |
|------------|-------|--------|
| [Rust](https://www.rust-lang.org/) | Linguagem principal | Implementado (scaffold) |
| [Axum](https://github.com/tokio-rs/axum) | Framework web (HTTP) | Planejado — ainda não implementado |
| [SQLx](https://github.com/launchbadge/sqlx) | Acesso a banco de dados | Implementado (migrações) |

> **Atenção:** Axum ainda **não** está implementado. O projeto está na fase inicial de scaffold e essa dependência será adicionada conforme o roadmap de aprendizado avança. O banco de dados (PostgreSQL via Docker Compose) e as migrações SQLx já estão configurados.

## Status

O projeto está em **estágio inicial de scaffold**:

- `Cargo.toml` configurado com `edition = "2024"` e package `ff-codex`.
- `src/main.rs` contém apenas um `Hello, world!` básico.
- Banco de dados PostgreSQL configurado via `docker-compose.yml` (container efêmero, exposto na porta 5433 do host).
- Migrações SQLx criadas em `migrations/` (`001_create_table_game.sql` e `002_insert_game.sql`).
- Nenhuma dependência externa declarada no `Cargo.toml` ainda (o SQLx ainda não está integrado ao código).

As próximas etapas adicionam Axum e integram o SQLx ao código, sempre com foco em aprender um conceito por vez.

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
- [Docker](https://docs.docker.com/get-docker/) e [Docker Compose](https://docs.docker.com/compose/) instalados.
- `sqlx-cli` instalado para executar as migrações:

  ```bash
  cargo install sqlx-cli --no-default-features --features native-tls,postgres
  ```

### Passos

Todos os comandos abaixo são executados a partir da pasta `app/`:

```bash
cd app

# 1. Subir o banco de dados (PostgreSQL via Docker Compose)
docker compose up -d

# 2. Executar as migrações (usa o DATABASE_URL definido no .env)
sqlx migrate run

# 3. Compilar o projeto
cargo build

# 4. Executar o binário
cargo run
```

Notas:

- O banco é **efêmero**: o `docker-compose.yml` não define volume persistente, então os dados são perdidos ao recriar o container (`docker compose down` seguido de `docker compose up -d`).
- O `DATABASE_URL` do `.env` aponta para `localhost:5433`, pois a porta 5432 do host está ocupada por um PostgreSQL nativo.
- Para parar o banco: `docker compose down`.

A saída esperada no estado atual é:

```
Hello, world!
```

## Estrutura do projeto

```
ff-codex/
├── app/
│   ├── Cargo.toml         # Manifesto do projeto (dependências e configuração)
│   ├── Cargo.lock         # Versões travadas das dependências
│   ├── docker-compose.yml # PostgreSQL efêmero para desenvolvimento
│   ├── .env               # Variáveis de ambiente (DATABASE_URL)
│   ├── migrations/        # Migrações SQLx
│   │   ├── 001_create_table_game.sql
│   │   └── 002_insert_game.sql
│   └── src/
│       └── main.rs        # Ponto de entrada da aplicação
└── .gitignore             # Arquivos ignorados pelo Git
```

O código-fonte fica em `app/` — todos os comandos (`cargo`, `sqlx`, `docker compose`) devem ser executados a partir dessa pasta. A estrutura será expandida conforme novas dependências e módulos forem adicionados.