FROM ubuntu:latest
RUN apt-get update
RUN apt-get install ca-certificates -y
WORKDIR /app
COPY target/release/librarian .
ENTRYPOINT ["/app/librarian"]