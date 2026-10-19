# Template de dépôt privé d'assets

Les workflows sont désormais installés dans [rakmahefa/PokeEmerald-rust-private-assets](https://github.com/rakmahefa/PokeEmerald-rust-private-assets). Ce dossier public reflète ces fichiers pour documentation/reproduction ; il ne contient aucune ROM.

Pour utiliser le dépôt existant, suis [docs/ROM_SECURITY.md](../ROM_SECURITY.md), et lance les workflows depuis le dépôt privé lui-même. Aucun secret GPG ni PAT inter-dépôts n'est requis avec l'entrée temporaire actuellement utilisée.

## Contenu du modèle

- `.github/workflows/prepare-pret-decomp.yml` : validation du dump fourni, comparaison avec la source pret épinglée, purge de l'historique de ROM et stockage des sources dans `decomp/pokeemerald/`.
- `.github/workflows/build-linux.yml` : build privé ROM-free des exécutables Rust et publication d'un artefact/release.
- `upstream.lock` : commits de référence.
- `.gitignore` : ignore les ROMs téléversées à la racine.
