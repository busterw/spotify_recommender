 # Use the official Rust image as the build stage
 FROM rust:latest AS builder

 # Set the working directory inside the container
 WORKDIR /usr/src/app
 
 # Copy the Cargo.toml and Cargo.lock files
 COPY Cargo.toml Cargo.lock ./
 
 # Copy the source code
 COPY src ./src
 
 # Copy the static files and the SQLite database
 COPY static ./static
 COPY Moods.db3 ./Moods.db3
 
 # Build the application in release mode
 RUN cargo build --release
 
 FROM debian:bookworm-slim

# Install SQLite and OpenSSL 3 libraries
RUN apt-get update && \
    apt-get install -y --no-install-recommends libsqlite3-0 libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*
 
 # Set the working directory inside the container
 WORKDIR /usr/src/app
 
 # Copy the built application from the builder stage
 COPY --from=builder /usr/src/app/target/release/spotify_recommender .
 
 # Copy the static files and the SQLite database
 COPY --from=builder /usr/src/app/static ./static
 COPY --from=builder /usr/src/app/Moods.db3 ./Moods.db3
 
 # Expose the port that the application will run on
 EXPOSE 8080
 
 # Set the command to run the application
 CMD ["./spotify_recommender"]