# SCKK (új) API

Az új api már nem NodeJS/Typescript rendszerre épül hanem egy Rust alapúra.
Ennek célja a rendszer sebességénel javítása és a kapacitás növelése. Ez a rendszer már [Axum](https://github.com/tokio-rs/axum) és [SeaORM](https://www.sea-ql.org/SeaORM) használatával működik.

## Előkészületek

- A rendszer használata egyszerű. Nincs más dolgunk mint felrakni a [Rustot](https://rustup.rs).
- Ez után fel kell rakni a Cargo Watch-ot mely lehetővé teszi, hogy automatikusan újrainduljon az API ha új változtatást teszünk közzé.
- A Cargo Watch-ot így tudjuk feltelepíteni: `cargo install cargo-watch`.
- .env fájl létrehozása a .env.example fájl alapján.

## Development

Ha ez sikeresen felrakódott nincs más dolgunk mint a monorepo fő könyvtárában lefuttatni ezt a parancsot: `pnpm dev:api`.
Ha esetleg ennek a nyers változatát akarjuk lefuttatni akkor azt ebben a mappában így tudjuk megtenni: `cargo watch -x run`.
