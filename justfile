default:
    @just --list

set windows-powershell

[windows]
r_dev soft:
    cd {{ soft }}; cargo watch -x run

[linux]
r_dev soft:
    cd {{ soft }} && cargo watch -x run

[windows]
w_dev:
    cd web; pnpm dev

[linux]
w_dev:
    cd web && pnpm dev

[windows]
w_i:
    cd web; pnpm install

[linux]
w_i:
    cd web && pnpm install

[windows]
d_build soft tag:
    cd {{ soft }}; docker build --platform linux/amd64 . -t ghcr.io/sckk-apms-dev/saes-{{ soft }}:{{ tag }}

[linux]
d_build soft tag:
    cd {{ soft }} && docker build --platform linux/amd64 . -t ghcr.io/sckk-apms-dev/saes-{{ soft }}:{{ tag }}

[windows]
w_build:
    cd web; pnpm build

[linux]
w_build:
    cd web && pnpm build

[windows]
r_build soft:
    cd {{ soft }}; cargo build --release

[linux]
r_build soft:
    cd {{ soft }} && cargo build --release