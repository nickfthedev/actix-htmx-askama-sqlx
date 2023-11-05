## Actix-Askama-Template

### Tailwind make CSS
Watching changes
```
npm run css:dev
```
Just build:
```
npm run css:build
```

### Run Cargo Watch (must be installed)

Install
```
cargo install cargo-watch
```
Run:
```
cargo watch -x run'  OR  'npm run dev
```

### Database SQLx

Install SQLx CLI
```
cargo install sqlx-cli --no-default-features --features rustls,postgres

```
Copy .env.sample to .env and change database url