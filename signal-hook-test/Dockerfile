# Start with the rust image
FROM rust:alpine AS builder

# Update and upgrade the image
RUN apk update && apk upgrade
RUN apk add --no-cache musl-dev

# Set the Current Working Directory inside the container
WORKDIR /app

# Copy source and secret files
RUN pwd
COPY Cargo.toml /app/
COPY src/ /app/src/

# Build and install the app
RUN cargo build --release

# Create a new stage for the final image
FROM alpine:latest

# Update and upgrade the image
RUN apk update && apk upgrade

# Set the Current Working Directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/signal-hook-test /app/signal-hook-test

# Command to run the executable
ENTRYPOINT [ "/app/signal-hook-test" ]
