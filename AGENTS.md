# AGENTS.md

Small, fun Rust project for public group food ordering. Keep playful dev tone unless user-facing change needs calmer/polished voice. Existing casual error/loading text OK.

## Project Shape

- `napoli-lib`: shared protobuf-generated API, domain helpers, money handling.
- `napoli-server`: Tonic gRPC/gRPC-web backend, SQLite via SeaORM.
- `napoli-server-migrations`: SeaORM migrations.
- `napoli-server-persistent-entities`: generated SeaORM entities. Careful editing generated files.
- `napoli-pain`: Yew/WASM web frontend.
- `napoli-pain/src-napoli-app`: Tauri wrapper around frontend.
- `napoli-client-grpc-web`: small raw gRPC-web test/debug client.

## Core Concepts

- Orders: public group orders with menu URL, state, timestamp, cutoff time, entries.
- Order entries: buyer, food, price, paid state.
- Prices use `Millicents` / `price_in_millicents`; avoid new floating-point money logic.
- Protobuf contract in `napoli-lib/proto` shared by frontend/backend; API changes usually affect both.
- Frontend talks to backend through gRPC-web.
- Live order updates use `StreamOrderUpdates` on API and watch channels in server.

## Intended Happy Path

Product makes shared restaurant order easy:

1. Someone opens app, creates new order with restaurant/menu URL and optional cutoff time.
2. App creates public order page like `/order/:id`.
3. People share order page and add entries with buyer name, food, price.
4. Everyone on order page sees live updates as entries are added, removed, or marked paid.
5. Order detail page shows individual entries, total price, grouped food summary.
6. At cutoff time, someone uses summary to call/place actual restaurant order.
7. People mark entries paid so group tracks who settled up.

Authentication and payment processing intentionally out of scope.

## Development Guidelines

- Prefer existing simple patterns over larger abstractions.
- Keep changes scoped; not becoming enterprise platform despite flake description.
- Preserve repo humor where present, but avoid hostile/confusing/user-hostile messages in serious paths.
- Backend data contract changes: update protobuf, adapters, entities/migrations, frontend service calls together.
- Frontend changes: keep Yew component state straightforward; use existing service wrapper in `napoli-pain/src/service.rs`.
- Database schema changes: add SeaORM migration in `napoli-server-migrations`; keep persistent entities in sync.
- Do not hand-roll money parsing/formatting when `napoli_lib::Millicents` can do it.
- Keep validation consistent with `napoli-lib/src/limits.rs` and `napoli-server/src/validate.rs`.

## Useful Commands

```sh
cargo check
cargo test
cargo run -p napoli-server
cd napoli-pain && trunk serve
```

Frontend defaults to `http://[::1]:50051` for backend unless built with `BACKEND_URL`.

## Notes For Agents

- Use `rg` for searching.
- Use `apply_patch` for manual edits.
- Do not rewrite generated SeaORM entities unless task requires it.
- Do not remove fun just because informal; project can have personality.
