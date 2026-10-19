# PokeEmerald-rust

Projet expérimental **Rust + Bevy** qui vise une expérience de jeu inspirée de Pokémon Emerald : overworld 3D voxel, compagnon de type Kanto Gear et fenêtre de réglages moderne en Dioxus Desktop. La priorité est Linux x86_64.

> **État actuel :** socle compilable et prototypes d'interface. Ce n'est pas encore un jeu Pokémon Emerald complet. Le runtime ne lit pas encore les données issues de la décompilation, les Pokémon overworld sont des objectifs de roadmap et les réglages Dioxus ne sont pas encore persistés ou synchronisés avec Bevy.

## Architecture

- **Bevy** : fenêtre de jeu, scène 3D, ECS, input et interface compagnon.
- **Dioxus Desktop** : application indépendante de réglages.
- **emerald-core** : contrats et configuration partagés.
- **Dépôt privé séparé** `rakmahefa/PokeEmerald-rust-private-assets` : snapshots source privés, métadonnées et éventuelles données locales dérivées ; aucun binaire ROM en clair dans ce dépôt public.

Le panneau compagnon et le monde 3D sont actuellement dans une même fenêtre Bevy. Ce sont deux zones de l'interface, pas deux moniteurs physiques.

## Sources

- [pret/pokeemerald](https://github.com/pret/pokeemerald) : décompilation source existante, logique du jeu et ressources versionnées.
- [ZallaxDev/emerald-3ds](https://github.com/ZallaxDev/emerald-3ds) : référence pour l'overworld voxel, la visualisation à deux zones et les pipelines d'assets.
- [AverageConsumer/kanto-gear](https://github.com/AverageConsumer/kanto-gear) : référence UX du compagnon Pokégear/C-Gear.

**Important :** `pret/pokeemerald` est déjà une décompilation maintenue. Le processus ne « décompile » pas automatiquement n'importe quelle ROM pour produire du code C. La ROM fournie sert ici à vérifier la version personnelle ; le workflow construit et compare le résultat de la source upstream, puis conserve un snapshot des fichiers source suivis par Git. Le binaire ROM n'est jamais joint à l'artefact.

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

Le workflow `CI` vérifie formatage, compilation et tests. Le workflow `Linux release artifact` crée une archive Linux avec les deux exécutables prototypes et SHA-256. Un tag `v*` peut publier une release de la fondation logicielle.

Ces workflows **ne lisent jamais le dépôt privé d'assets et ne demandent aucun secret ROM**. L'artefact est actuellement un bootstrap technique ; il ne contient ni ROM ni ressources de jeu intégrées, et n'est pas encore un jeu complet.

## Import ROM et décompilation de référence

La préparation du dépôt privé est documentée dans [docs/ROM_SECURITY.md](docs/ROM_SECURITY.md). Le workflow destiné au dépôt privé est fourni comme modèle dans [docs/private-assets-template](docs/private-assets-template).

Le flux visé est :
1. fournir localement un dump autorisé, le chiffrer et placer uniquement le fichier chiffré dans le dépôt privé ;
2. lancer manuellement le workflow situé **dans le dépôt privé** ; il déchiffre brièvement la ROM, vérifie son SHA-1, compile et compare la source pret épinglée, puis détruit le fichier clair ;
3. stocker un snapshot source ROM-free dans une release privée et un artefact d'exécution ;
4. utiliser ce snapshot pour le développement suivant. Les builds quotidiens n'ont alors plus besoin de la ROM ni du passphrase initial ;
5. ne demander une nouvelle ROM que si une étape future choisie explicitement en a besoin, par exemple une réimportation locale des données pour un build final.

Ce flux utilise le `GITHUB_TOKEN` scoped au dépôt privé pour publier sa propre release. Il ne requiert pas de PAT pour consulter un autre dépôt privé. Le modèle public n'a pas la permission de lire un second dépôt privé ; les workflows de build qui utilisent les assets doivent donc s'exécuter depuis le dépôt privé.

## Prochaines phases

1. Créer le dépôt privé et y installer les templates de workflow.
2. Générer le snapshot source privé, vérifier les checksums et confirmer que la ROM n'est pas dans l'artefact.
3. Implémenter un importeur Rust pour produire des données structurées à partir des ressources de référence.
4. Charger cartes, collisions, objets et environnements dans Bevy.
5. Ajouter le système Pokémon overworld, connecter l'état de jeu au compagnon, puis synchroniser/persister les réglages Dioxus.
6. Définir précisément le modèle de distribution des assets et la nécessité éventuelle d'une ROM personnelle au premier lancement.
