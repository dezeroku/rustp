FROM python:3.9 as z3
RUN mkdir -p /z3_built
COPY z3 /z3
WORKDIR /z3
RUN python3 scripts/mk_make.py --prefix=/z3_built
WORKDIR /z3/build
RUN make -j2
RUN make install

FROM rust:1.50.0-alpine as rust
RUN cargo new rustp
WORKDIR ./rustp

# Get dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release && \
    rm src/*.rs && \
    rm ./target/release/deps/rustp*

# Build the actual binary
ADD ./src/ ./src/
RUN cargo build --release

FROM ubuntu:20.04 as final
COPY --from=z3 /z3_built/bin /usr/bin
COPY --from=z3 /z3_built/lib /usr/lib
COPY --from=z3 /z3_built/include /usr/include
COPY --from=rust /rustp/target/release/rustp /usr/bin

CMD ["rustp"]