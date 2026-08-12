# AGENTS.md

Projeto de estudo em Rust (edition 2024) para aprender Axum + SQLx via uma API inspirada em *Final Fantasy* (pacote `ff-codex`).

## Localização e comandos

Todo o código e configuração ficam em `app/`. **Todos os comandos (`cargo`, `sqlx`, `docker compose`) devem ser executados a partir de `app/`.** O CI (`.github/workflows/rust.yml`) também roda com `working-directory: ./app`.

- Subir o banco: `docker compose up -d` (PostgreSQL 18.4-alpine, efêmero — sem volume persistente; dados somem com `docker compose down`)
- Aplicar migrações: `sqlx migrate run` (requer o banco de pé; usa `DATABASE_URL` do `app/.env`)
- Sem par: `cargo build`, `cargo run`, `cargo test`, `cargo clippy`, `cargo fmt` (todos de `app/`)

## Estado atual (importante)

- **API HTTP implementada com Axum 0.8.** `src/main.rs` carrega o `.env` (dotenvy), inicializa `tracing` (logs JSON), cria o `PgPool`, consulta `games` no startup, monta o `router()` e chama `server(app)`. Rotas ativas: `GET /health`, `GET /ready`, `GET/POST /ff-codex/games`.
- **SQLx integrado ao GET** — `main.rs` cria `PgPool` + `GameRepository` + `GameService` e injeta via `AppState` no `router()`. `GET /ff-codex/games` lê do banco (a consulta também roda no startup, logada como `Games: [...]`). `POST /ff-codex/games` ainda **não** persiste — devolve um eco do payload. Próxima etapa: INSERT no POST. Não inventar endpoints além dos existentes.
- `Dockerfile` (multi-stage, rust:1.97 → distroless, user `nonroot`) expõe a porta 8080 e o servidor HTTP já roda nela.
- `[profile.dev]` é customizado (`incremental=false`, `debug=1`, `codegen-units=256`) → builds dev lentos; não alterar sem necessidade.

## Migrações (SQLx) — gotchas

- Migração `003_create_table_caracters.sql` cria a tabela **`caracters`** (grafia incorreta de "characters" — usar o nome exato). Tem `game_id INTEGER NOT NULL` com FK `REFERENCES games(id) ON DELETE CASCADE`.
- Nomeação mista pt/en: `games` usa `titulo`, `ano_lancamento` (pt); `caracters` usa `name` (en). IDs são `GENERATED ALWAYS AS IDENTITY`.
- Migrações já commitadas não devem ser editadas — elas são o histórico versionado do banco; alterações vão como novas migrações (`00N_...`).

## Código — gotchas

- **Target do binário é `ff_codex` (underscore).** O Cargo converte o nome do pacote `ff-codex` (hífen) para `ff_codex` no target. Por isso o `EnvFilter` em `main.rs` usa `ff_codex=trace` — se escrever `ff-codex=trace`, o filtro não casa e os logs do crate somem.
- **SQLx usa a feature `postgres`** — `Cargo.toml` declara `sqlx = { version = "0.8", features = ["postgres","runtime-tokio","macros"], default-features = false }`, compatível com o PostgreSQL do docker-compose e com o `DATABASE_URL`. A antiga divergência (`sqlite`) foi corrigida; não reverter.
- **Erros centralizados em `src/rest/error.rs`:** enum `AppError` (NotFound, BadRequest, Internal) com `impl IntoResponse` que monta o corpo JSON `{error, code}` e o status HTTP; `Internal` logga via `tracing::error!`. Novos erros devem seguir esse padrão — não criar respostas de erro avulsas nos handlers.
- **Módulos no padrão moderno (sem `mod.rs`):** `src/rest/handler.rs` (arquivo) + `src/rest/handler/` (diretório com sub-módulos). Ao adicionar um handler, criar o arquivo em `src/rest/handler/` e declará-lo em `handler.rs`.
- **`dotenvy` carrega o `.env` antes do `tracing`:** `main.rs` chama `dotenv().ok()` como primeiro passo do runtime, então `DATABASE_URL` (e `RUST_LOG`, se estiver no `.env`) ficam disponíveis antes do `EnvFilter`. Não mover o `dotenv()` para depois do `tracing`.
- **Camadas `domain/`, `repository/` e `service/`:** `domain/game.rs` define a struct `Game` (`#[derive(FromRow, Debug)]`, campos `pub` `id`/`titulo`/`ano_lancamento`; `id` tem `#[expect(dead_code)]` pois ainda não é exposto na API); `repository/game.rs` define `GameRepository { pool: PgPool }` (`#[derive(Clone)]`) com `all_games() -> Result<Vec<Game>, sqlx::Error>` via `select * from games`; `service/game_service.rs` define `GameService::all_games() -> Result<Vec<Game>, GameError>` (thiserror, `Internal(#[from] sqlx::Error)`) — **retorna domínio, nunca DTO**; o `service` não importa `rest::dto`. A conversão `Game` → `GamesResponse` é feita **no handler** (`map(GamesResponse::from)`) via `impl From` em `rest/dto/game.rs`. Consultas vão no repositório, nunca no handler.
- **Responsabilidades das camadas:** `repository` = SQL/persistência; `service` = regras de negócio, validação e orquestração (ex.: transações, unicidade); `handler` = traduz HTTP (DTO, status, `AppError`). O service não conhece HTTP/DTO; a conversão de resposta fica no handler.
- **Injeção via `State`:** `main.rs` monta `GameRepository` → `GameService` → `AppState` (`src/rest/app_state.rs`, `#[derive(Clone)]`) e chama `router(app_state)`; `routers.rs` aplica `.with_state(state)`. Handler recebe `State(state): State<AppState>` e mapeia `GameError` → `AppError::Internal`.

## Porta / DATABASE_URL

Porta efetiva: **5432** (`docker-compose.yml` mapeia `5432:5432`, `.env` usa `localhost:5432`). Se mudar a porta, sincronize `.env`, o mapeamento e o comentário do compose.

## Referência de contexto

README.md (raiz) documenta o roadmap, o status e a stack — ao divergir do código, confie em `docker-compose.yml`, `.env`, `app/migrations/` e `app/src/`.