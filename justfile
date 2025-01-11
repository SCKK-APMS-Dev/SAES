default:
  @just --list

rdev soft:
    cargo watch "-x run --bin saes-{{soft}}" 

wdev:
    cd web && pnpm dev