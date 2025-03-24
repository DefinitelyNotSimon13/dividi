# Stage 1: Build the Rust binary
FROM rust:1 AS builder
WORKDIR /app

# Cache dependencies: copy manifest files first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY . .
RUN cargo build --release

# Stage 2: Build TailwindCSS/DaisyUI assets
FROM oven/bun:latest AS tailwind-builder
WORKDIR /app
COPY package.json bun.lock ./
COPY public ./
RUN bun install

COPY styles/tailwind.css ./styles/tailwind.css

RUN bunx @tailwindcss/cli -i ./styles/tailwind.css -o ./public/styles.css

# Stage 3: Final image (temporarily using debian for debugging)
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/dividi .
COPY --from=tailwind-builder /app/public ./public
EXPOSE 8000
CMD ["./dividi"]
