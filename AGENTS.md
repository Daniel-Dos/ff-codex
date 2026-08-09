# AGENTS.md

Projeto de estudo em Rust (edition 2024) para aprender Axum + SQLx via uma API inspirada em *Final Fantasy* (pacote `ff-codex`).

## Localização e comandos

Todo o código e configuração ficam em `app/`. **Todos os comandos (`cargo`, `sqlx`, `docker compose`) devem ser executados a partir de `app/`.** O CI (`./github/workflows/rust.yml`) também roda com `working-directory: ./app`.

- Subir o banco: `docker compose up -d` (PostgreSQL 18.4-alpine, efêmero — sem volume persistente; dados somem com `docker compose down`)
- Aplicar migrações: `sqlx migrate run` (requer o banco de pé; usa `DATABASE_URL` do `app/.env`)
- Sem par: `cargo build`, `cargo run`, `cargo test`, `cargo clippy`, `cargo fmt` (todos de `app/`)

## Estado atual (importante)

- **Axum ainda não está implementado.** `src/main.rs` apenas imprime `Hello, world!`; `Cargo.toml` não tem nenhuma dependência e o SQLx não está integrado ao código.
- Não inventar endpoints de API — o roadmap (README) define `/health` como próxima etapa.
- `Dockerfile` (multi-stage, rust:1.97 → distroless, user `nonroot`) expõe a porta 8080, mas ainda não existe servidor HTTP.
- `[profile.dev]` é customizado (`incremental=false`, `debug=1`, `codegen-units=256`) → builds dev lentos; não alterar sem necessidade.

## Migrações (SQLx) — gotchas

- Migração `003_create_table_caracters.sql` cria a tabela **`caracters`** (grafia incorreta de "characters" — usar o nome exato). Tem `game_id INTEGER NOT NULL` com FK `REFERENCES games(id) ON DELETE CASCADE`.
- Nomeação mista pt/en: `games` usa `titulo`, `ano_lancamento` (pt); `caracters` usa `name` (en). IDs são `GENERATED ALWAYS AS IDENTITY`.
- Migrações já commitadas não devem ser editadas — elas são o histórico versionado do banco; alterações vão como novas migrações (`00N_...`).

## Porta / DATABASE_URL

Porta efetiva: **5432** (`docker-compose.yml` mapeia `5432:5432`, `.env` usa `localhost:5432`, README pois corrigido). Ainda restam menções obsoletas a 5433 no comentário do `docker-compose.yml` e numa nota do README — ignore-as. Se mudar a porta, sincronize `.env`, o mapeamento e o comentário do compose.

## Referência de contexto

README.md (raiz) documenta o roadmap e a stack, mas está defasado (~55 linhas vs `feature/migration-banco`): confie em `docker-compose.yml`, `.env` e `app/migrations/` quando houver divergência.