FROM rust:latest AS builder

# Install deps
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt update -y && \
    apt install nodejs -y && \
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall dioxus-cli

WORKDIR /app
COPY . .

# Build tailwindcss
RUN npm install tailwindcss @tailwindcss/cli && \
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css

ENV PORT=8100
ENV IP=0.0.0.0

EXPOSE 8100

RUN dioxus bundle --platform web --release --output target/dx/spades/release/web/

FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/dx/spades/release/web/ /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
