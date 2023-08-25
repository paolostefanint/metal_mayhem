from rust:1.72.0

COPY . /core

WORKDIR /core

RUN cargo build --release

CMD ["cargo", "run", "--release"]

