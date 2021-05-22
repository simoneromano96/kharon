FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/kharon

COPY . .

RUN cargo install --path .

FROM debian:stable-slim as production

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /kharon

COPY --from=builder /usr/local/cargo/bin/kharon /kharon

COPY ./environments/ /kharon/environments/

CMD ["/kharon/kharon"]
