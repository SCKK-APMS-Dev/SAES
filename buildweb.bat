cd web
@REM docker build --platform linux/amd64,linux/arm64 . -t ghcr.io/sckk-apms-dev/taxiweb-web:latest
docker build --platform linux/amd64 . -t ghcr.io/sckk-apms-dev/taxiweb-web:test --push