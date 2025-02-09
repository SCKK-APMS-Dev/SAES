# SAMT API v2 (Rust)

## Szükségletek:
- `rust`
- `cargo-watch`
- `just` (opcionális)

## Konfiguráció:
A Container Image futtatása esetén a `main.json` configot a `/usr/src/api/prod/config/main.json` helyre!

Alap config:
``` json
{
  "global": {
    "maintenance": null,
    "announcement": null
  },
  "taxi": {
    "allow_ucp": true,
    "allow_shift": true,
    "allow_fleet": true,
    "allow_faction": true,
    "shift_access": "SameShift"
  },
  "tow": {
    "allow_ucp": true,
    "allow_shift": true,
    "allow_fleet": true,
    "allow_faction": true,
    "shift_access": "SameShift"
  }
}
```