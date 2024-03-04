FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
# Copy all files from our working environment to our Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true

# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release --bin rinha

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rinha rinha
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./rinha"]