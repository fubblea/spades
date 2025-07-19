FROM rust:slim-bullseye

# Install deps
RUN apt update && \
    apt install -y npm nodejs && \
    cargo install dioxus-cli

WORKDIR /app
COPY . .

# Build tailwindcss
RUN npm install tailwindcss @tailwindcss/cli && \
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT [ "dx", "serve", "--platform", "web", "--release"]
