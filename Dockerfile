FROM rust:latest as build

# Create empty shell project
RUN USER=root cargo new --bin Kanes-Maid
WORKDIR /Kanes-Maid


# Copy over dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


# Cache dependencies
RUN cargo build --release
RUN rm src/&.rs

# Copy src tree over
COPY ./src ./src
COPY ./config.toml ./config.toml

# use slim image
FROM rust:latest-slim-buster

COPY --from=build /Kanes-Maid/target/release/Kanes-Maid .

CMD ["./Kanes-Maid"]