FROM ubuntu:16.04

MAINTAINER Jorge Brunal <jebrunal@correo.unicordoba.edu.co>

RUN apt-get update \
    && apt-get install -y curl nano file net-tools build-essential

RUN curl https://sh.rustup.rs -s > /home/install.sh && \
    chmod +x /home/install.sh && \
    sh /home/install.sh -y --verbose --default-toolchain nightly

ENV PATH "/root/.cargo/bin:$PATH"

ENV SOURCES=/sources

RUN mkdir -p $SOURCES

ADD ./ $SOURCES

WORKDIR $SOURCES

RUN rustup update

RUN cargo update

# to remove
RUN rustup --version
# to remove
RUN cargo --version
# to remove
RUN rustc --version

RUN cargo build --release

EXPOSE 80
EXPOSE 6969
EXPOSE 8000
EXPOSE 8080

# CMD ROCKET_ENV=development ./target/release/rocket-rust
CMD ROCKET_ENV=production ./target/release/rocket-rust
