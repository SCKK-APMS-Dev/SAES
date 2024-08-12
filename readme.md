# SCKK Extra Monorepo

Ez a [pnpm](https://pnpm.io) [monorepo](https://monorepo.tools) tartalmazza az sckk.hu weboldalt, az ahhoz tartozó api-t és a hívásrögzítő botot is.

Bármi probléma akadna [itt](https://github.com/SCKK-APMS-Dev/SCKKExtra/issues/new) lehet panaszt tenni

## Development

A monorepo mint fentebb leírva található pnpm-et használ. Ezt legegyszerűbben így tudjuk feltelepíteni: `npm install -g pnpm`.
Ha ez megvan mindenféle tevékenységet le tudunk folytatni.

## Használat előtt

Fontos megjegyezni, hogy minden külső változtatás és/vagy új klónozás után mindig ajánlott itt, a főkönyvtárban egy `pnpm install` parancsot futtatni.

## Extra npm csomagok telepítése

Telepíteni könnyen lehet, csak tudni kell a módját. Alapértelmezetten a pnpm nem enged feltelepíteni ide csomagokat, és ez így is van jól. Ha viszont tényleg fel szeretnénk telepíteni csomagokat akkor azt vagy a repo azon részében található mappában meg tudjuk tenni. Értem ezt úgy, hogy a weboldalhoz szeretnénk felrakni akkor ahhoz menjünk be a /web könyvtárba és ott megtehetjük.
Ezen kívül itt a főkönyvtárban is meg tudjuk tenni csak adjuk hozzá a végéhez, hogy `--filter ./könyvtár`

# API

Fontos megjegyezni, hogy az api NEM NodeJS-t használ már. Magyarul oda hiába akarunk felrakni bármilyen npm csomagot mivel még package.json sincs a /api könyvtárban. Részleteg a mappán belüli readme-ben.

# További információk

A monorepo egyes részeiről azok mappáiban található readme fájlból tudsz további információt gyűjteni.
