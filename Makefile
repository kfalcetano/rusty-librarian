build:
	cargo build --release
	docker build -t librarian:1.0 .
	docker save --output librarian.tar librarian
	tar -czvf rusty-librarian.tar.gz static/ pages/ librarian.tar Makefile
run:
	docker load --input librarian.tar
	docker compose up -d
certs:
	mkdir -p certs
	openssl req -x509 -newkey rsa:4096 -nodes -keyout certs/key.pem -out certs/cert.pem -days 365 -subj '/C=US/ST=Washington'