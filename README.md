# PokeEmerald-rust

Projet expérimental **Rust + Bevy** qui vise une expérience inspirée de Pokémon Emerald : overworld voxel 3D, compagnon de type Kanto Gear et fenêtre de réglages moderne en Dioxus Desktop. La priorité est Linux x86_64.

> **État actuel :** socle compilable et prototypes d'interface. L'importeur Rust peut générer un catalogue privé de métadonnées des cartes depuis une source pret épinglée et le runtime affiche le catalogue lorsqu'il est présent. Le terrain Bevy reste procédural : layouts réels, collisions, tilesets, sprites, interactions, sauvegardes et progression ne sont pas encore implémentés.

## Architecture

- **Bevy** : fenêtre de jeu, scène 3D, ECS et interface compagnon.
- **Dioxus Desktop** : application indépendante de réglages.
- **emerald-core** : configuration et contrats partagés.
- **emerald-data** : schéma versionné, validation et importeur de catalogue.
- **Dépôt privé séparé** [rakmahefa/PokeEmerald-rust-private-assets](https://github.com/rakmahefa/PokeEmerald-rust-private-assets) : snapshot de source pret épinglé et fichiers de provenance.

Le panneau compagnon et le monde 3D sont actuellement dans une même fenêtre Bevy. Ce sont deux zones de l'interface, pas deux moniteurs physiques.

## Sources

- [pret/pokeemerald](https://github.com/pret/pokeemerald) : décompilation source existante, logique du jeu et ressources versionnées.
- [ZallaxDev/emerald-3ds](https://github.com/ZallaxDev/emerald-3ds) : référence pour l'overworld voxel, la visualisation à deux zones et les pipelines d'assets.
- [AverageConsumer/kanto-gear](https://github.com/AverageConsumer/kanto-gear) : référence UX du compagnon Pokégear/C-Gear.

**Important :** pret/pokeemerald est déjà une décompilation maintenue ; le workflow ne décompile pas automatiquement une ROM pour produire du code C. Le bootstrap historique a vérifié l'empreinte du dump temporaire, reconstruit une version précise de la source amont puis exécuté make compare. La ROM temporaire a ensuite été retirée de l'arbre et de l'historique Git normal du dépôt privé. Le snapshot source privé ne contient aucun binaire ROM.

Les sources et ressources upstream ont leurs propres licences. Aucune licence globale n'est déclarée avant d'avoir défini le périmètre exact du code et des données intégrés. Le catalogue généré est conservé dans les artefacts privés pour le moment.

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

## Pipeline de données

Voir docs/DATA_PIPELINE.md pour le contrat JSON et la commande d'import. L'outil lit les métadonnées de cartes de la copie locale de pret/pokeemerald et n'extrait aucun fichier ROM, sprite, tileset, layout ou script.

Le workflow privé Build Linux from private snapshot construit l'application, génère le catalogue et publie une archive Linux privée. Le runtime lit assets/map_catalog.json à côté de son exécutable, si le fichier est présent. Le workflow public n'accède pas au dépôt privé.

## CI et artefact public

Le workflow CI vérifie le formatage, la compilation et les tests. Le workflow public Linux release artifact crée une archive Linux de la fondation logicielle avec SHA-256. Un tag v* peut publier cette version bootstrap.

Ces workflows ne lisent pas le dépôt privé et n'ont aucun secret ROM. L'artefact public reste un prototype technique sans ROM, snapshot amont ni données privées.

## État et prochaines phases

1. Faire passer les layouts de cartes du format amont à un modèle tuilé interprété par Rust.
2. Importer et rendre les données de layout et collisions dans la scène Bevy.
3. Gérer les transitions de cartes, warps, objets et interactions de scripts.
4. Ajouter le système Pokémon overworld et relier l'état du jeu au compagnon.
5. Persister et synchroniser les réglages Dioxus.
6. Définir la politique finale de distribution des données et des assets.
