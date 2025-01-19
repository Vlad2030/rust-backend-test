# --- BUILD --- #
FROM rust:1.84.0-slim-bookworm as builder

WORKDIR /usr/backend

RUN rustup target add x86_64-unknown-linux-musl

# Copy only necessary files into the container
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --target x86_64-unknown-linux-musl --release

# --- RUN --- #
FROM scratch

COPY --from=builder /usr/backend/target/x86_64-unknown-linux-musl/release/rust-backend /usr/local/bin/rust-backend

ENV BACKEND_TITLE rust-backend
ENV BACKEND_VERSION 0.0.1
ENV BACKEND_HOST localhost
ENV BACKEND_PORT 6969
ENV BACKEND_LOG_LEVEL info
ENV BACKEND_WORKERS 1

CMD ["rust-backend"]



# # --- BUILD --- #
# FROM rust:1.84.0-slim-bookworm as builder

# WORKDIR /usr/backend

# RUN rustup target add x86_64-unknown-linux-musl

# COPY . ./usr/backend

# RUN cargo build --target x86_64-unknown-linux-musl --release

# # --- RUN --- #
# FROM scratch

# COPY --from=builder /usr/backend/target/x86_64-unknown-linux-musl/release/rust-backend /usr/local/bin/rust-backend

# ENV BACKEND_TITLE rust-backend
# ENV BACKEND_VERSION 0.0.1
# ENV BACKEND_HOST localhost
# ENV BACKEND_PORT 6969
# ENV BACKEND_LOG_LEVEL info
# ENV BACKEND_WORKERS 1

# CMD ["rust-backend"]
