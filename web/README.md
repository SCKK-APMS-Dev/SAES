# SCKK Weboldal

Ez a része a monoreponak tartalmazza a weboldalt.
A weboldalt SvelteKit futtatja, dizájnokért a [Tailwind CSS](https://tailwindcss.com) és a [Flowbite](https://flowbite.com) felel.

## Előkészületek

- A weboldal futtatásához pnpm-re lesz szükség amelyet a főkönyvtári readme-ből fel is tudunk telepíteni.
- Egy `pnpm install` lefuttatása ha még nem történt meg.

## Development

Nincs más dolgunk mint vagy a főkönyvtárból lefuttatni a `pnpm dev:web` parancsot, vagy itt a `pnpm dev` parancsot.

# FONTOS

A weboldal szorosan egybefügg az apival. Ezt úgy kell érteni, hogy az api nélkül eléggé limitáltak a lehetőségek.

# Környezeti változók

- `secret_key`: A secret_key az api eléréséhez.
- `api_prod_url`: Az api nyilvános url elérhetősége.
