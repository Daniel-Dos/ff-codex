# FF Codex

Projeto de estudos para aprender **Rust**, **Axum** e **SQLx** por meio do desenvolvimento de uma API inspirada no universo de *Final Fantasy*.

O objetivo não é construir um produto completo, mas sim praticar conceitos fundamentais da linguagem e do ecossistema Rust enquanto se constrói algo divertido e com escopo bem definido.

## Stack

| Tecnologia | Papel | Status |
|------------|-------|--------|
| [Rust](https://www.rust-lang.org/) | Linguagem principal | Implementado (scaffold) |
| [Axum](https://github.com/tokio-rs/axum) | Framework web (HTTP) | Implementado (rotas `/health`, `/ready`, `/ff-codex/games`) |
| [SQLx](https://github.com/launchbadge/sqlx) | Acesso a banco de dados | Implementado (pool + consulta no startup) |

> **Atenção:** o SQLx está **parcialmente integrado**. O `main.rs` cria um pool de conexões (`PgPool`) e consulta a tabela `games` no startup (log `Games: [...]`), mas o pool ainda **não** foi ligado aos handlers do Axum (falta o `State`) — `GET /ff-codex/games` continua devolvendo dados fixos. A próxima etapa é injetar o `GameRepository` nos handlers.

## Status

O projeto está em **estágio de API funcional com dados fixos e SQLx parcialmente integrado**:

- `Cargo.toml` configurado com `edition = "2024"` e package `ff-codex`.
- Servidor HTTP com Axum 0.8 em `src/rest/` — rotas `/health`, `/ready` e `/ff-codex/games`.
- Logs estruturados em JSON via `tracing`/`tracing-subscriber`.
- Banco de dados PostgreSQL configurado via `docker-compose.yml` (container efêmero, exposto na porta 5432 do host).
- Migrações SQLx criadas em `migrations/` (`001_create_table_game.sql`, `002_insert_game.sql` e `003_create_table_caracters.sql`).
- SQLx integrado ao `main.rs`: carrega o `.env` via `dotenvy`, cria o `PgPool` (feature `postgres`) e consulta a tabela `games` no startup (log `Games: [...]`).
- Camadas `domain/` (`Game` com `FromRow`) e `repository/` (`GameRepository::all_games`).
- **Ainda pendente:** o repositório não está ligado aos handlers — `GET /ff-codex/games` segue com dados fixos.

A próxima etapa é injetar o `GameRepository` nos handlers do Axum via `State`, sempre com foco em aprender um conceito por vez.

## Roadmap

Etapas planejadas para o aprendizado, em ordem sugerida:

1. **Servidor HTTP com Axum** ✅
   - Endpoint `GET /health` que retorna o status da API. ✅
   - Endpoint `GET /ready` (prontidão do serviço). ✅
   - Endpoint `GET/POST /ff-codex/games` (lista e cadastro, dados fixos por enquanto). ✅
   - Entender rotas, handlers e extração de parâmetros. ✅

2. **Modelagem de dados**
   - Definir entidades do universo *Final Fantasy* (ex.: criaturas, personagens, itens). 🔄
   - Introduzir tipos e estruturas em Rust (DTOs `GamesRequest`/`GamesResponse` já criados). 🔄

3. **Persistência com SQLx** 🔄 (em andamento)
   - Conectar ao PostgreSQL via Docker Compose (pool de conexões). ✅
   - Executar migrações e consultas reais no código (pool + `GameRepository::all_games` no startup). ✅
   - Substituir os dados fixos dos handlers por consultas ao banco (injetar o `GameRepository` via `State`). 🔄

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
- O `DATABASE_URL` do `.env` aponta para `localhost:5432` (mesma porta mapeada pelo `docker-compose.yml`).
- **`cargo run` agora exige o banco de pé:** o `main.rs` conecta ao PostgreSQL e consulta a tabela `games` no startup. Se o banco estiver parado ou o `DATABASE_URL` não estiver definido no `.env`, o programa logga o erro e encerra antes de subir o servidor.
- Para parar o banco: `docker compose down`.

A saída esperada ao executar `cargo run` é uma sequência de logs estruturados em JSON, por exemplo:

```json
{"timestamp":"...","level":"INFO","fields":{"message":"Iniciando a api de Final Fantasy."},"target":"ff_codex"}
{"timestamp":"...","level":"INFO","fields":{"message":"Games: [Game { id: 1, titulo: \"Final Fantasy\", ano_lancamento: 1987 }, Game { id: 2, titulo: \"Final Fantasy II\", ano_lancamento: 1988 }, ...]"},"target":"ff_codex"}
{"timestamp":"...","level":"INFO","fields":{"message":"Server starting on http://0.0.0.0:8080"},"target":"ff_codex::rest::server_app"}
```

Para testar a API com o servidor de pé:

```bash
curl http://localhost:8080/health
curl http://localhost:8080/ready
curl http://localhost:8080/ff-codex/games
```

## Endpoints

| Método | Rota | Descrição | Resposta |
|--------|------|-----------|----------|
| GET | `/health` | Verificação de saúde da API | `200` `{"status":"up"}` |
| GET | `/ready` | Prontidão do serviço | `200` (sem corpo) |
| GET | `/ff-codex/games` | Lista de jogos (dados fixos — o handler ainda não usa o banco) | `200` `[{"titulo":"Final Fantasy VII","ano_lancamento":1997}]` |
| POST | `/ff-codex/games` | Cadastra um jogo (eco, sem persistência) | `200` corpo ecoado |

> `GET /ff-codex/games` ainda devolve dados fixos (hardcoded). O pool já existe e consulta o banco no startup (`Games: [...]`), mas ainda não foi ligado ao handler via `State` — essa é a próxima etapa.

## Estrutura do projeto

```
ff-codex/
├── app/
│   ├── Cargo.toml         # Manifesto do projeto (dependências e configuração)
│   ├── Cargo.lock         # Versões travadas das dependências
│   ├── Dockerfile         # Build multi-stage (rust:1.97 → distroless, porta 8080)
│   ├── docker-compose.yml # PostgreSQL efêmero para desenvolvimento
│   ├── .env               # Variáveis de ambiente (DATABASE_URL)
│   ├── migrations/        # Migrações SQLx
│   │   ├── 001_create_table_game.sql
│   │   ├── 002_insert_game.sql
│   │   └── 003_create_table_caracters.sql
│   └── src/
│       ├── main.rs        # Ponto de entrada (dotenv + tracing JSON + pool SQLx + router + server)
│       ├── rest.rs        # Módulo raiz da API (re-exports)
│       ├── rest/
│       │   ├── dto.rs             # DTOs (GamesRequest, GamesResponse)
│       │   ├── dto/game.rs        # Definição dos DTOs de game
│       │   ├── error.rs           # AppError + IntoResponse centralizado
│       │   ├── handler.rs         # Handlers (health, ready, games)
│       │   ├── handler/health.rs  # GET /health e GET /ready
│       │   ├── handler/games_handler.rs # GET/POST /ff-codex/games
│       │   ├── routers.rs         # Definição das rotas
│       │   └── server_app.rs      # Bind + graceful shutdown (Ctrl+C/SIGTERM)
│       ├── domain.rs      # Módulo raiz de domínio
│       ├── domain/game.rs # Struct Game (FromRow, campos privados)
│       ├── repository.rs  # Módulo raiz de repositórios
│       └── repository/game.rs # GameRepository (pool PgPool + all_games)
└── .gitignore             # Arquivos ignorados pelo Git
```

O código-fonte fica em `app/` — todos os comandos (`cargo`, `sqlx`, `docker compose`) devem ser executados a partir dessa pasta. A estrutura será expandida conforme novas dependências e módulos forem adicionados.
