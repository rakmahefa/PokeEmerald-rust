# Registre des décisions : conserver, porter, remplacer ou supprimer

## Projet Rust existant

| Chemin | Décision | Pourquoi/action |
|---|---|---|
| Cargo.toml, rust-toolchain.toml | KEEP / REVIEW | garder versions épinglées jusqu'à une PR d'upgrade testée; ne pas mettre à jour « juste parce que neuf ». |
| crates/emerald-core/src/lib.rs | KEEP + EXPAND | GameSettings est un bon point de partage mais le modèle de jeu n'est pas encore défini; ajouter domaine pur sans Bevy. |
| crates/emerald-data/src/lib.rs | KEEP + EXPAND | garder MapCatalog schéma 1 comme index; ajouter schémas distincts pour layouts, métatuiles, events et encounters. |
| crates/emerald-data/src/main.rs | KEEP + EXPAND | importer déterministe/diagnostique; commandes explicites et pas de chemins privés codés en dur. |
| crates/emerald-game/src/main.rs | REPLACE INCREMENTALLY | monolithe prototype (cuboïdes procéduraux, joueur cube, compagnon statique); découper plugins puis remplacer la scène de test par la vraie carte. |
| crates/emerald-settings/src/main.rs et style.css | KEEP + CONNECT | garder Dioxus settings si utiles; persistance et transport vers Bevy à ajouter. |
| docs/*.md, session_handoff.md, README.md | KEEP + UPDATE | documentation vivante, doit refléter les capacités vraiment terminées. |
| docs/private-assets-template/** | KEEP + HARDEN | conserver séparation public/privé; ajouter audit strict du paquet. |
| .github/workflows/** | KEEP + HARDEN | format/check/test, intégration headless, vérification du paquet et provenance. |

## Source pret/pokeemerald

| Famille | Décision |
|---|---|
| Overworld, field control, fieldmap, métatuiles, collisions | PORT : règles métier du monde et déplacement. |
| wild encounters + tables | KEEP les règles, PORT le moteur et tester la parité, intégrer OWE ensuite. |
| battle setup/main/controllers/script commands | KEEP le domaine, PORT en machine d'état Rust. |
| Pokémon, party, items, moves, save, dex, flags | KEEP/PORT les données et règles utiles. |
| script/event data, NPC, warp et map events | PORT vers données typées + VM d'événements non bloquante. |
| data/maps, data/layouts, tilesets, tables et graphics | KEEP AS INPUT → TRANSFORM dans le pipeline privé; droits avant distribution. |
| registres GBA, DMA, VRAM/OAM, BIOS, interruptions, ABI | REMOVE-RUNTIME; conserver la source privée comme oracle/build si besoin. |
| m4a/mixer, sprite/background/GPU et hardware input | REPLACE par audio/assets/UI/input Bevy. |
| Makefile GBA, ROM header, linker, scripts de génération ROM | KEEP PRIVATE TOOLING si nécessaires; REMOVE de l'application Rust. |
| Link/RFU/e-Reader/Frontier/Contest/minijeux | REVIEW / LATER plutôt que supprimer en premier. |

## Emerald 3DS

- Logique de voxel sous 3ds_port/src/voxel : PORT sélectif des algorithmes, backend Citro3D remplacé; préserver notices MIT si le code/logique est adapté.
- scripts/gen_map_data.py : étudier/adopter le concept de bundle des layouts avec tests propres.
- patch 0014-visible-wild.patch : PORT sémantique en ECS; aucun hook CB1/ressource sprite C ne passe tel quel.
- couche libctru/Citro2D/Citro3D, écran tactile natif, fixed heap, bridge C/3DS : REMOVE-RUNTIME/REPLACE.

## Kanto Gear

- core/contracts.lua, navigation, habitats/progression : PORT CONCEPTS en Rust.
- adaptateurs Gen1/2/3, pointers, memory/save hooks et native menus : REPLACE; la source de vérité devient GameSession.
- runtime, pages, menus : PORT UX par étape et test des commandes.
- UI/font atlases/icons : REVIEW par asset, licences/notices individuelles.
- tests : porter les scénarios applicables et réécrire les fixtures dépendantes de Gen1Recomp.

Une suppression définitive exige un motif documenté : hors cible, remplacé avec tests, hors périmètre explicitement décidé, ou interdit pour sécurité/droits. Jusqu'à cette preuve, marquer REVIEW.
