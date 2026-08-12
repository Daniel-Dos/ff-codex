# FF Codex

Projeto de estudos para aprender **Rust**, **Axum** e **SQLx** por meio do desenvolvimento de uma API inspirada no universo de *Final Fantasy*.

O objetivo não é construir um produto completo, mas sim praticar conceitos fundamentais da linguagem e do ecossistema Rust enquanto se constrói algo divertido e com escopo bem definido.

## Stack

| Tecnologia | Papel | Status |
|------------|-------|--------|
| [Rust](https://www.rust-lang.org/) | Linguagem principal | Implementado (scaffold) |
| [Axum](https://github.com/tokio-rs/axum) | Framework web (HTTP) | Implementado (rotas `/health`, `/ready`, `/ff-codex/games`) |
| [SQLx](https://github.com/launchbadge/sqlx) | Acesso a banco de dados | Implementado (GET lê do banco) |

> **Atenção:** o SQLx está integrado ao fluxo de leitura: `GET /ff-codex/games` consulta a tabela `games` via `State` + `GameService`. O `POST /ff-codex/games` ainda **não** persiste — ele devolve um eco do payload. A próxima etapa é persistir o cadastro (INSERT).

## Status

O projeto está em **estágio de API funcional com leitura no banco (GET) e cadastro sem persistência (POST)**:

- `Cargo.toml` configurado com `edition = "2024"` e package `ff-codex`.
- Servidor HTTP com Axum 0.8 em `src/rest/` — rotas `/health`, `/ready` e `/ff-codex/games`.
- Logs estruturados em JSON via `tracing`/`tracing-subscriber`.
- Banco de dados PostgreSQL configurado via `docker-compose.yml` (container efêmero, exposto na porta 5432 do host).
- Migrações SQLx criadas em `migrations/` (`001_create_table_game.sql`, `002_insert_game.sql` e `003_create_table_caracters.sql`).
- SQLx integrado: `.env` via `dotenvy`, `PgPool` (feature `postgres`) criado no `main.rs`, consulta `games` no startup (log `Games: [...]`) e **`GET /ff-codex/games` lê do banco** via `State` (`GameService` → `GameRepository`).
- Camadas `domain/` (`Game` com `FromRow`), `repository/` (`GameRepository::all_games`), `service/` (`GameService`) e `rest/app_state.rs` (`AppState`) — o `GameService` retorna domínio e a conversão `Game` → `GamesResponse` acontece no handler (camada de apresentação).
- **Ainda pendente:** `POST /ff-codex/games` devolve um eco do payload — a persistência (INSERT) é a próxima etapa.

As próximas etapas adicionam a escrita (INSERT no POST), sempre com foco em aprender um conceito por vez.

## Roadmap

Etapas planejadas para o aprendizado, em ordem sugerida:

1. **Servidor HTTP com Axum** ✅
   - Endpoint `GET /health` que retorna o status da API. ✅
   - Endpoint `GET /ready` (prontidão do serviço). ✅
   - Endpoint `GET/POST /ff-codex/games` (lista do banco; cadastro como eco). ✅
   - Entender rotas, handlers e extração de parâmetros. ✅

2. **Modelagem de dados**
   - Definir entidades do universo *Final Fantasy* (ex.: criaturas, personagens, itens). 🔄
   - Introduzir tipos e estruturas em Rust (DTOs `GamesRequest`/`GamesResponse` já criados). 🔄

3. **Persistência com SQLx** 🔄 (em andamento)
   - Conectar ao PostgreSQL via Docker Compose (pool de conexões). ✅
   - Executar migrações e consultas reais no código (`GameRepository::all_games` no startup). ✅
   - Ligar o repositório aos handlers via `State` (`GameService` + `AppState`) — `GET /ff-codex/games` lê do banco. ✅
   - Persistir o cadastro no `POST /ff-codex/games` (INSERT). 🔄

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
| GET | `/ff-codex/games` | Lista de jogos do banco (via `GameService`) | `200` `[{"titulo":"Final Fantasy VII","ano_lancamento":1997}]` |
| POST | `/ff-codex/games` | Cadastra um jogo (eco, sem persistência) | `200` corpo ecoado |

> `GET /ff-codex/games` lê da tabela `games` (SQLx). O `POST` ainda não persiste — devolve um eco do payload; a persistência (INSERT) é a próxima etapa.

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
│       │   ├── app_state.rs       # AppState (GameService) injetado via State
│       │   ├── dto.rs             # DTOs (GamesRequest, GamesResponse)
│       │   ├── dto/game.rs        # DTOs + impl From<Game> for GamesResponse
│       │   ├── error.rs           # AppError + IntoResponse centralizado
│       │   ├── handler.rs         # Handlers (health, ready, games)
│       │   ├── handler/health.rs  # GET /health e GET /ready
│       │   ├── handler/games_handler.rs # GET/POST /ff-codex/games
│       │   ├── routers.rs         # Definição das rotas (+ with_state)
│       │   └── server_app.rs      # Bind + graceful shutdown (Ctrl+C/SIGTERM)
│       ├── domain.rs      # Módulo raiz de domínio
│       ├── domain/game.rs # Struct Game (FromRow, campos pub)
│       ├── repository.rs  # Módulo raiz de repositórios
│       ├── repository/game.rs # GameRepository (pool PgPool + all_games)
│       ├── service.rs     # Módulo raiz de serviços
│       └── service/game_service.rs # GameService (GameError + all_games)
└── .gitignore             # Arquivos ignorados pelo Git
```

O código-fonte fica em `app/` — todos os comandos (`cargo`, `sqlx`, `docker compose`) devem ser executados a partir dessa pasta. A estrutura será expandida conforme novas dependências e módulos forem adicionados.
