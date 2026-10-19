# Spécification — PokeEmerald-rust

- Statut : spécification initiale, à affiner avant toute réutilisation substantielle de code tiers.
- Cible principale : Linux desktop x86_64.
- Langage/runtime : Rust, workspace Cargo, Bevy pour le jeu et Dioxus Desktop pour les réglages.
- État actuel : prototypes d'interface ; aucune promesse de gameplay complet.

## 1. Vision

Créer une application desktop inspirée de Pokémon Emerald dont le runtime et la logique sont implémentés en Rust. L'utilisateur fournit sa propre ROM ; le logiciel doit extraire ou préparer localement les données nécessaires à l'exécution, sans inclure la ROM dans le code source public, les logs ou l'artefact déployable.

Le résultat visé est une application desktop native, pas un émulateur GBA générique. Le projet doit choisir explicitement une stratégie de portage/recompilation et ne pas laisser entendre que le code C de pret/pokeemerald s'exécute directement dans Bevy.

## 2. Expérience utilisateur

### Fenêtre de jeu

La fenêtre principale Bevy utilise une disposition à deux zones :

1. **Zone principale — overworld 3D voxel** : caméra, terrain, bâtiments, avatar, transitions d'intérieur/extérieur, éclairage, collisions et interactions.
2. **Zone compagnon — Kanto Gear** : panneau de données lié à l'état réel du jeu, avec Pokédex, équipe, carte, inventaire, outils, notes et progression.

Dans le squelette actuel, ces deux zones sont dans la même fenêtre : monde voxel placeholder et panneau compagnon Bevy UI. « Deux écrans » désigne deux zones logiques de l'interface, pas une dépendance à deux moniteurs physiques. Une sortie multi-fenêtre pourra être ajoutée après validation de l'expérience.

### Réglages

Une application/fenêtre Dioxus Desktop indépendante est prévue pour les réglages : rendu voxel, audio, caméra, échelle UI, visibilité du compagnon et chemin des données du jeu.

Le squelette Dioxus ne partage pas encore de configuration persistée ni d'état avec Bevy. La couche de configuration partagée existe comme premier contrat ; persistance et IPC sont des étapes séparées.

## 3. Architecture logique

~~~text
pokeemerald-rust-game (Bevy)
  ├── Game state / ECS
  ├── Native overworld + voxel renderer
  ├── Interaction and input
  ├── Kanto Gear companion view
  └── consumes shared configuration/data
                   ▲
                   │ crates/emerald-core
                   ▼
pokeemerald-rust-settings (Dioxus Desktop)
  ├── Settings UI
  └── future persistence / IPC

Private ROM ──> validator/importer ──> local derived game data ──> runtime
     │                    │
     └─ encrypted at rest └─ decrypted temporarily in a trusted build
~~~

Responsabilités :
- **emerald-core** : modèles sérialisables indépendants du moteur, options de jeu et contrats de données.
- **pokeemerald-rust-game** : application Bevy, monde, caméra, rendu, input et panneau compagnon.
- **pokeemerald-rust-settings** : application de réglages Dioxus Desktop.

Un futur crate importeur ROM ou un binaire xtask devra être ajouté après validation de la version ROM prise en charge, du schéma des assets générés et des limites de licence.

## 4. Source de données ROM

### Version initialement visée

Les références pret/pokeemerald et emerald-3ds identifient le dump Pokémon Emerald (USA, Europe) v1.0 par le SHA-1 :

~~~text
f3ae088181bf583e55daf962a92bb46f4f1d07b7
~~~

Le workflow initial n'accepte que cette empreinte. La prise en charge d'autres régions/révisions nécessite une table explicite d'empreintes acceptées. Une empreinte vérifie l'identité et l'intégrité du fichier, mais ne prouve pas un droit de distribution.

### Règles

- Pas de ROM en clair dans Git, les Issues, les fichiers de conversation ou les logs.
- Pas de ROM ou de fichier dérivé contenant du contenu protégé dans un artefact public.
- Déchiffrement dans un chemin temporaire du runner ; nettoyage même en cas d'échec.
- Validation avant extraction ; échec fermé pour une ROM inconnue ou corrompue.
- Les données produites doivent être documentées et examinées avant toute distribution.
- Un utilisateur doit détenir le jeu et vérifier les droits et règles applicables. Un dépôt privé ne transfère aucun droit sur le contenu.

## 5. Build et distribution

### CI

À chaque push ciblé et pull request :
- vérification du format Rust ;
- compilation de vérification du workspace ;
- tests unitaires ;
- aucun secret ROM accessible à la CI standard.

### Build Linux

Le workflow manuel génère une archive Linux x86_64 comme artefact Actions avec SHA-256. Un tag v* génère une release publique de la **fondation logicielle seulement**, sans ROM. Le workflow manuel qui utilise la ROM ne se lance que depuis main et ne publie pas de GitHub Release.

Quand l'importeur sera raccordé au jeu, une décision distincte sera nécessaire sur la distribution des données dérivées. Par défaut, les artefacts contenant des ressources générées à partir de la ROM ne seront pas publiés publiquement.

## 6. Sécurité des workflows

- CI PR : permissions minimales en lecture ; pas de secrets ROM.
- Import ROM : lancement manuel depuis main uniquement.
- ROM chiffrée conservée dans un dépôt privé séparé ; passphrase GPG uniquement en secret Actions.
- PAT à portée limitée en lecture seule sur le dépôt privé.
- Pas de pull_request_target pour compiler/exécuter du code provenant d'une PR.
- Les releases publiques sont conditionnées à un tag et ne consomment pas la ROM privée.
- Les artefacts Actions sont téléchargeables par les personnes ayant accès au dépôt ; ils ne sont pas un coffre-fort pour des fichiers protégés.

## 7. Références et propriété intellectuelle

- **pret/pokeemerald** : référence pour comprendre le comportement et les structures. La présence d'une décompilation publique ne signifie pas que les données/code Pokémon sont libres de redistribution.
- **ZallaxDev/emerald-3ds** : référence pour un portage homebrew 3DS. Son fichier LICENSE-PORT.md limite la licence MIT aux ajouts originaux désignés ; cette licence ne couvre ni le contenu Pokémon ni la décompilation upstream. Le portage 3DS n'est pas une implémentation Bevy.
- **AverageConsumer/kanto-gear** : mod Lua pour Gen1Recomp et code couvert par MIT. Il sert d'inspiration fonctionnelle et visuelle ; aucune image, police ou ressource ne doit être copiée sans vérifier ses crédits/licences.

Ne pas déclarer le projet complet MIT tant que la licence globale et les obligations provenant de chaque fichier intégré ne sont pas clarifiées. Ce scaffold n'ajoute aucune ressource ROM ni aucun binaire tiers.

## 8. Phases de livraison

### Phase 0 — fondation (en cours)
- workspace Rust ;
- prototypes Bevy et Dioxus ;
- CI, archive Linux et procédure ROM privée documentée.

### Phase 1 — importeur local
- valider type de fichier et empreintes approuvées ;
- extraire les données nécessaires ;
- tests négatifs de ROM invalide ;
- classifier les données générées avant packaging.

### Phase 2 — runtime de jeu
- chargement des cartes/données ;
- contrôles, collisions, interactions et transitions ;
- caméra, terrain voxel et intérieurs ;
- sauvegardes locales et tests de compatibilité.

### Phase 3 — compagnon natif
- modèle de données partagé dérivé de l'état du jeu ;
- équipe, Pokédex, carte, sac, outils et progression ;
- navigation, accessibilité et contrôles.

### Phase 4 — Pokémon overworld natif
- apparitions et sprites/modèles ;
- comportement de suivi/interaction ;
- animations, despawns et transitions de carte.

### Phase 5 — réglages et distribution
- persistance de configuration ;
- synchronisation Dioxus ↔ runtime ;
- bundling Linux, dépendances, checksums et smoke tests ;
- distribution sans ROM ni données non autorisées.
