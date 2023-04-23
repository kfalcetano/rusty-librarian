FROM ubuntu:latest
WORKDIR /app
COPY target/release/librarian .
COPY cert.pem .
COPY key.pem .
ENTRYPOINT ["/app/librarian"]