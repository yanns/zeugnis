FROM scratch
MAINTAINER Yann Simon <yann.simon.fr@gmail.com>

COPY ./target/x86_64-unknown-linux-musl/release/zeugnis /zeugnis
EXPOSE 8080
ENTRYPOINT ["/zeugnis"]