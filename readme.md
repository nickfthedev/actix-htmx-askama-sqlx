## Actix-Askama-HTMX-SQLX-Template

###A implemented To-Do-List using the Techstack above
![image](https://github.com/nickfthedev/actix-htmx-askama-sqlx/assets/24594991/3956c8a6-a68d-4854-8dad-2009100ff91c)

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
