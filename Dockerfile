# Build frontend
FROM node:latest as frontend-builder
WORKDIR /usr/src/nadamark
COPY frontend/ ./frontend/
WORKDIR /usr/src/nadamark/frontend
RUN npm install
RUN npm run build

# Build backend
FROM rust:latest as backend-builder
WORKDIR /usr/src/nadamark
COPY backend/ ./backend/
WORKDIR /usr/src/nadamark/backend

RUN apt-get update && apt-get install -y libwayland-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo clean
RUN cargo build --release

# Final image
FROM debian:bookworm-slim
WORKDIR /usr/local/bin/nadamark
# Copy the built frontend files
COPY --from=frontend-builder /usr/src/nadamark/frontend/build ./static
# Copy the backend binary
COPY --from=backend-builder /usr/src/nadamark/backend/target/release/nadamark ./

# Create a non-root user
RUN useradd -r -s /bin/false nadamark && \
    chown -R nadamark:nadamark /usr/local/bin/nadamark && \
    mkdir -p /bookmarks && \
    chown -R nadamark:nadamark /bookmarks

USER nadamark

# Set the static files path as an environment variable
ENV STATIC_FILES_PATH="./static"

EXPOSE 8663
CMD ["./nadamark"]
