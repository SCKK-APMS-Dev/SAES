cd api
@REM docker build --platform linux/amd64,linux/arm64 . -t ghcr.io/sckk-apms-dev/taxiweb-api:latest
docker build --platform linux/amd64 . -t ghcr.io/sckk-apms-dev/sckkextra-api:test --push