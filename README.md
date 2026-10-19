# PokeEmerald-rust

Projet expérimental **Rust + Bevy** qui vise une expérience de jeu inspirée de Pokémon Emerald : overworld 3D voxel, compagnon de type Kanto Gear et fenêtre de réglages moderne en Dioxus Desktop. La priorité est Linux x86_64.

> **État actuel :** socle compilable et prototypes d'interface. Ce n'est pas encore un jeu Pokémon Emerald complet. Le runtime ne lit pas encore les données issues de la décompilation, les Pokémon overworld sont des objectifs de roadmap et les réglages Dioxus ne sont pas encore persistés ou synchronisés avec Bevy.

## Architecture

- **Bevy** : fenêtre de jeu, scène 3D, ECS, input et interface compagnon.
- **Dioxus Desktop** : application indépendante de réglages.
- **emerald-core** : contrats et configuration partagés.
- **Dépôt privé séparé** [`rakmahefa/PokeEmerald-rust-private-assets`](https://github.com/rakmahefa/PokeEmerald-rust-private-assets) : source pret épinglée et snapshot ROM-free, manifestes et checksums.

Le panneau compagnon et le monde 3D sont actuellement dans une même fenêtre Bevy. Ce sont deux zones de l'interface, pas deux moniteurs physiques.

## Sources

- [pret/pokeemerald](https://github.com/pret/pokeemerald) : décompilation source existante, logique du jeu et ressources versionnées.
- [ZallaxDev/emerald-3ds](https://github.com/ZallaxDev/emerald-3ds) : référence pour l'overworld voxel, la visualisation à deux zones et les pipelines d'assets.
- [AverageConsumer/kanto-gear](https://github.com/AverageConsumer/kanto-gear) : référence UX du compagnon Pokégear/C-Gear.

**Important :** `pret/pokeemerald` est déjà une décompilation maintenue. Le processus ne « décompile » pas automatiquement une ROM pour produire du code C. Le workflow valide le dump fourni, reconstruit une version précise de la source upstream et exécute `make compare`. Il stocke ensuite une copie de la source suivie par Git. Le binaire ROM n'est jamais inclus dans le snapshot ni dans les artefacts.

Les dépôts et ressources de référence ont des périmètres de licence différents. Aucune licence globale n'est déclarée avant d'avoir défini le périmètre exact de code et de données intégrés.

## Linux — développement

Installe Rust stable via [rustup](https://rustup.rs/), puis les dépendances Linux. Sur Ubuntu 24.04 :

~~~bash
sudo apt update
sudo apt install build-essential pkg-config clang cmake \
  libasound2-dev libudev-dev libxkbcommon-dev libwayland-dev \
  libx11-dev libxi-dev libxcursor-dev libxrandr-dev libgl1-mesa-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev
~~~

Lancer les prototypes :

~~~bash
cargo run -p pokeemerald-rust-game
cargo run -p pokeemerald-rust-settings
~~~

Vérifier le workspace :

~~~bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
~~~

## CI et artefact public

Le workflow `CI` vérifie formatage, compilation et tests. Le workflow public `Linux release artifact` crée une archive Linux de la fondation logicielle avec SHA-256. Un tag `v*` peut publier cette version bootstrap.

Ces workflows **ne lisent pas le dépôt privé** et n'ont aucun secret ROM. L'artefact public reste un prototype technique : il ne contient ni ROM ni ressources de jeu intégrées, et n'est pas encore un jeu complet.

## Bootstrap ROM et décompilation

Le dépôt privé a été créé et contient les deux workflows nécessaires. La procédure détaillée est dans [docs/ROM_SECURITY.md](docs/ROM_SECURITY.md).

1. Dans le dépôt privé, lancer manuellement **Actions → Prepare private pret source snapshot** depuis `main`.
2. Le workflow vérifie le SHA-1 du dump temporaire, reconstruit et compare la source pret épinglée, puis enlève le dump de `main` et réécrit son historique Git. En cas de succès, les fichiers source sont conservés dans `decomp/pokeemerald/`, accompagnés d'un manifeste sans ROM.
3. Le workflow publie aussi une release privée et un artefact Actions contenant l'archive source, le manifeste et le checksum.
4. Pour les builds de développement, lancer **Actions → Build Linux from private snapshot** dans le dépôt privé. Cette étape n'utilise ni ROM, ni passphrase, ni PAT inter-dépôts. Elle produit un artefact et une release privée contenant l'archive Linux.
5. Le code Rust ne consomme pas encore les données pret : importer les cartes et ressources et les connecter au runtime reste un travail à venir.

Le dump actuellement présent dans le dépôt privé est temporaire. Ne lance pas plusieurs opérations Git simultanées pendant que le bootstrap réécrit l'historique. Si la vérification de compilation échoue avant de produire le snapshot, le workflow conserve un dump dont le SHA-1 est accepté pour permettre un nouvel essai ; un dump invalide est purgé après vérification.

## Prochaines phases

1. Terminer le bootstrap source privé et vérifier la disparition de la ROM de l'historique Git normal.
2. Implémenter un importeur Rust pour produire des données structurées à partir des ressources de référence.
3. Charger cartes, collisions, objets et environnements dans Bevy.
4. Ajouter le système Pokémon overworld et relier l'état du jeu au compagnon.
5. Persister et synchroniser les réglages Dioxus.
6. Définir le modèle final de distribution des assets et la nécessité éventuelle d'une ROM personnelle lors d'un import local.
