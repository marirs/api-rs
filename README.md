# api-rs

Bare-bones API server using Rust Rocket framework. You can add API Key verification using JWT and/or database, etc by taking the barebones project from here.
you can also add as many endpoints you would want.  

If you are looking for an API server with database and api key verification etc, you can check out my other [Repo](https://github.com/marirs/rocketapi).

### Requirements
- Rust 1.52+ (Stable)

### Running
- Build
```bash
cargo b
```

- Run  

Start with server defaults
```bash
cargo run
```

Start with a config file
```bash
cargo run -- -c config.yml
```

#### Generate SSL cert
```bash
sudo openssl req -x509 -nodes -days 30 -newkey rsa:2048 -keyout key.pem -out cert.pem
```
---
License: MIT