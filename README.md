# Recipe Web Service — Final Project

**Author:** Aura María Castellanos Calderón  
**Project Type:** Recipe Web Service  
**Tech Stack:** Rust, Axum, SQLite, SQLx, Askama, Utoipa, Yew  

## 📋 Project Summary

This project is a full-stack web application for managing and viewing recipes. It consists of:

- A backend REST API built with **Axum**, **SQLite**, and **SQLx**.
- An HTML user interface templated with **Askama**.
- API documentation using **Utoipa** and **Swagger UI**.
- A frontend client built with **Yew** (compiled to WebAssembly).
- Docker and Trunk used for bundling and serving the application.

## 🧱 Folder Structure

```
final-recipe-project/
│
├── recipe-server/         # Backend: Axum + SQLite
│   ├── src/
│   └── templates/
│
├── yew-client/            # Frontend client: Yew (WASM)
│   ├── src/
│   └── index.html
│
├── recipes.db             # SQLite database
├── Dockerfile             # Optional containerization
└── README.md              # This file
```

## 🚀 How to Run

1. Start the backend:
   ```bash
   cargo run
   ```

2. In a separate terminal, start the frontend client:
   ```bash
   trunk serve --proxy-backend=/api=http://127.0.0.1:3000 --open
   ```
## Challenges Encountered

- Yew client with Trunk setup took significant debugging time due to:
  - Proxy misconfigurations.
  - `rand` crate compatibility issues with `wasm32-unknown-unknown` (missing features like `std_rng`).
- Some WebAssembly tooling errors due to dependencies that don't support WASM out of the box.
- Syncing the Yew frontend and Axum backend routing paths correctly.
- Debugging compile-time errors with SQLx async calls and Askama templates.

## Outstanding Work / Future Improvements

- Fix `rand::thread_rng()` errors by using alternative randomization compatible with WASM or removing the dependency.
- Polish the Yew client design and add interactivity (e.g., search, filtering).
- Implement JWT authentication for secure recipe editing (optional).
- Add full Docker support for production deployment.

## Additional Notes

If you encounter errors on build, try:
```bash
cargo clean
cargo build
```

And ensure you have the proper target installed:
```bash
rustup target add wasm32-unknown-unknown
```

