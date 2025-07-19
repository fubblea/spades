FROM rust:bullseye

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

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT [ "dx", "serve", "--platform", "web", "--release"]
