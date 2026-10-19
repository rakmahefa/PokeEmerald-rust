# PokeEmerald-rust

Projet expérimental Rust + Bevy visant à reconstruire une expérience jouable inspirée de Pokémon Emerald, avec un overworld voxel 3D, un panneau compagnon de type Kanto Gear et une application de réglages moderne en Dioxus. Le projet cible d'abord Linux.

> **État initial :** fondations techniques uniquement. Le monde voxel, le panneau compagnon et les réglages sont des prototypes. Le moteur de jeu complet, l'import ROM, la logique Pokémon native, les sauvegardes et la synchronisation des écrans ne sont pas encore implémentés.

## Objectifs

- **Bevy** pour le rendu 3D, le monde voxel, la caméra, les interactions et la boucle de jeu.
- **Kanto Gear companion** comme panneau secondaire : équipe, Pokédex, carte, inventaire et progression sont des objectifs, pas encore des fonctions connectées.
- **Dioxus Desktop** pour une fenêtre moderne de réglages.
- **Import privé** de la ROM fournie par l'utilisateur, avec validation de version et génération future des données nécessaires au runtime.
- **Linux release** générée par GitHub Actions, téléchargeable comme artefact avec checksum SHA-256. La ROM n'est jamais intégrée à l'artefact.

## Structure

~~~text
.
├── .github/workflows/
│   ├── ci.yml
│   └── release-linux.yml
├── crates/
│   ├── emerald-core/       # types et configuration partagés
│   ├── emerald-game/       # runtime Bevy et prototype voxel/companion
│   └── emerald-settings/   # interface Dioxus Desktop
├── docs/
│   ├── ARCHITECTURE.md
│   └── ROM_SECURITY.md
├── spec.md
├── Cargo.toml
└── README.md
~~~

## Développement sous Linux

Pré-requis : Rust stable via [rustup](https://rustup.rs/) et les bibliothèques système Bevy/Dioxus. Sur Ubuntu 24.04 :

~~~bash
sudo apt update
sudo apt install build-essential pkg-config clang cmake \
  libasound2-dev libudev-dev libxkbcommon-dev libwayland-dev \
  libx11-dev libxi-dev libxcursor-dev libxrandr-dev libgl1-mesa-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev
~~~

Lancer le prototype Bevy :

~~~bash
cargo run -p pokeemerald-rust-game
~~~

Lancer la fenêtre de réglages Dioxus :

~~~bash
cargo run -p pokeemerald-rust-settings
~~~

Vérifications locales :

~~~bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
~~~

Le binaire de réglages utilise WebKitGTK sur Linux ; les bibliothèques runtime correspondantes doivent également être présentes sur la machine cible.

## Artefact Linux

Le workflow **Actions → Linux release artifact → Run workflow** compile les deux exécutables et publie un artefact téléchargeable. Un push de tag v* crée aussi un GitHub Release pour la version bootstrap, sans ROM ni données dérivées d'une ROM.

L'archive Linux contient les exécutables et les informations de lancement, mais **pas** la ROM. Après téléchargement :

~~~bash
tar -xzf pokeemerald-rust-linux-x86_64.tar.gz
sha256sum --check SHA256SUMS
~~~

> Le pipeline d'import ROM est préparé côté sécurité, mais l'importeur et la génération de données pour le runtime restent à implémenter. Le prototype actuel ne construit donc pas encore le jeu complet à partir de la ROM.

## ROM privée et sécurité

La ROM ne doit **jamais** être commitée dans ce dépôt public, même temporairement. La procédure prévue chiffre localement la ROM avec GPG, stocke uniquement le fichier chiffré dans un dépôt GitHub **privé séparé**, puis la déchiffre temporairement dans le runner au cours d'un lancement manuel depuis main. Le workflow vérifie l'empreinte du dump Pokémon Emerald (US/EU 1.0) attendu par les références, puis supprime les fichiers temporaires avant la création de l'artefact.

Voir [docs/ROM_SECURITY.md](docs/ROM_SECURITY.md). Tout futur artefact de données dérivées devra faire l'objet d'une décision explicite sur les droits et la distribution.

## Sources de référence

- [pret/pokeemerald](https://github.com/pret/pokeemerald) — comportement du jeu, tables et référence de décompilation.
- [ZallaxDev/emerald-3ds](https://github.com/ZallaxDev/emerald-3ds) — portage Emerald, overworld voxel, écrans doubles et pipeline de données.
- [AverageConsumer/kanto-gear](https://github.com/AverageConsumer/kanto-gear) — inspiration fonctionnelle et visuelle pour le compagnon Pokégear/C-Gear.

Ces dépôts ont des licences et périmètres différents. Le projet ne copie pas automatiquement leur code ou leurs assets. Consultez [spec.md](spec.md) et [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) avant toute intégration. La licence globale de ce projet reste à décider.

## Roadmap immédiate

1. Valider le squelette CI/build sous Linux.
2. Configurer le dépôt privé d'assets et les secrets décrits dans [docs/ROM_SECURITY.md](docs/ROM_SECURITY.md).
3. Implémenter un importeur qui valide et traite la ROM sans la distribuer.
4. Remplacer le monde placeholder par les systèmes overworld et Pokémon natifs.
5. Connecter les données partagées au panneau compagnon et aux réglages Dioxus.
