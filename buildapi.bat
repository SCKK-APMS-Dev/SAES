cd api
docker build --platform linux/amd64,linux/arm64 . -t ghcr.io/sckk-apms-dev/taxiweb-api:latest
docker push ghcr.io/sckk-apms-dev/taxiweb-api:latest
