# Prisma Rust Silent Exit Reproduction

This repo demonstrates an [issue](https://github.com/Brendonovich/prisma-client-rust/issues/198) with [prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust/issues/198) when running inside docker.

*note: while creating this I ran into another docker-related issue which is added at the end*

## Steps

### Step 1
Setup the DB:
```sh
docker-compose up -d db
cargo prisma migrate deploy
```

### Step 2
Run the app locally to verify
```sh
DATABASE_URL=postgresql://postgres:example@localhost:5432/postgres cargo run
```
This will output a bunch of TRACE logs verifying that the app is working.

### Step 3
Run the app in Docker
```
docker-compose up --build app
```
This will emit a few of the initial logs then exit with code 139 which is a SEGFAULT


## Bonus Issue
It seems running the prisma-cli inside of the container segfaults, for this reason I have committed an edited (the schema path) version of the generated prisma file directly.

To reproduce this simply add the following line:
```
RUN cargo run -p prisma-cli -- generate
```

before the following line in the Dockerfile:
```
17 | RUN cargo build -p prisma-rust-silent-exit-reproduction --target x86_64-unknown-linux-musl --release --locked
```

Then run using the following command to cause the SEGFAULT:
```
docker-compose up --build app
```