# Architecture de démarrage

## Applications

### pokeemerald-rust-game

- Fenêtre desktop et boucle de jeu Bevy.
- Scène 3D provisoire composée de blocs générés en code.
- Panneau compagnon dans la fenêtre Bevy.
- Les données Pokémon, contrôles, overworld importé, sauvegardes et interactions restent à implémenter.

### pokeemerald-rust-settings

- Fenêtre Dioxus Desktop indépendante, avec une interface de réglages de style desktop.
- Les interrupteurs actuels modifient uniquement l'état visuel local de cette fenêtre.
- Persistance sur disque et communication avec Bevy non raccordées.

### emerald-core

Contrats simples qui ne dépendent ni de Bevy ni de Dioxus. Les deux interfaces doivent utiliser les mêmes types. À terme, ce crate hébergera les modèles stables pour les paramètres et les données partagées, mais pas un parseur ROM géant.

## Échanges de données

Le runtime Bevy doit être l'autorité sur l'état de la partie. Kanto Gear affiche une projection de cet état et ne doit pas inventer un état distinct. Le panneau compagnon peut rester dans Bevy UI pour limiter le nombre de boucles d'évènements. Dioxus reste une fenêtre indépendante au départ ; IPC ou fichier de configuration sera choisi et testé avant de lier les deux applications.

## Import ROM

L'importeur sera un outil explicite et versionné :
1. valider l'extension, une taille raisonnable et l'empreinte approuvée ;
2. n'extraire que les catégories nécessaires ;
3. valider indices/offsets et refuser les fichiers hors limites ;
4. produire des données locales documentées et versionnées par schéma ;
5. ne jamais copier le fichier source dans l'archive de déploiement.

Il faudra choisir entre données extraites à l'installation, pack local privé et chargement direct runtime. Aucun contenu n'est encore extrait par le scaffold.

## Références

- pret/pokeemerald : source de référence pour structures et comportements Emerald. Ce dépôt n'est pas une dépendance Rust ; ses données ne sont pas automatiquement redistribuables.
- emerald-3ds : référence sur le split d'affichage, l'extraction des données et le monde voxel. Le portage cible l'architecture 3DS, pas Bevy.
- kanto-gear : référence d'UX et d'organisation fonctionnelle. Il s'agit d'un mod Lua destiné à Gen1Recomp, non d'un composant Dioxus. Ne copier aucune ressource avant vérification des crédits et licences.
