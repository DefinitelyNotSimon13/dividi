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
FROM node:23-alpine AS tailwind-builder
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm install --production
COPY styles/tailwind.css ./styles/tailwind.css
RUN npx @tailwindcss/cli -i ./styles/tailwind.css -o ./public/styles.css --minify

# Stage 3: Final image (temporarily using debian for debugging)
FROM gcr.io/distroless/cc-debian12

# Set env vars for logging/backtraces
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

COPY --from=builder /app/target/release/dividi .
COPY --from=tailwind-builder /app/public ./public
EXPOSE 8000
CMD ["./dividi"]
