# Architecture

## Applications

### pokeemerald-rust-game

- Fenêtre desktop et boucle de jeu Bevy.
- Scène voxel procédurale de démonstration.
- Panneau compagnon dans la fenêtre Bevy.
- Lit facultativement assets/map_catalog.json, valide le schéma et affiche le nombre de cartes ainsi que la révision de source.
- Ne rend pas encore les layouts, collisions, tilesets ou sprites du jeu.

### pokeemerald-rust-settings

- Fenêtre Dioxus Desktop indépendante avec une interface de réglages de style desktop.
- Les contrôles modifient uniquement l'état visuel local ; persistance et synchronisation avec Bevy restent à implémenter.

### emerald-core

Contrats de configuration simples, partagés par les deux interfaces et indépendants de Bevy/Dioxus.

### emerald-data

- Définit le schéma versionné du catalogue de cartes.
- Importe les groupes et les métadonnées des fichiers map.json d'une copie épinglée de pret/pokeemerald.
- Valide le hash source, les noms de chemin, les doublons, les champs et les comptages.
- Ne lit aucun binaire ROM, layout, sprite, tileset ou script.
- Ne contient aucune donnée amont dans le dépôt public.

## Échanges de données

Le runtime Bevy demeure l'autorité sur l'état de la partie. Le panneau compagnon affiche une projection de cet état plutôt qu'un état séparé. Le panneau peut rester dans Bevy UI pour limiter le nombre de boucles d'évènements. Dioxus reste une fenêtre indépendante jusqu'à ce qu'une méthode de persistance et de communication soit choisie et testée.

## Import et livraison

Le workflow du dépôt privé valide et conserve un snapshot source ROM-free. Au moment du build privé, l'importeur génère un catalogue compact à partir du snapshot local. Le paquet privé contient les exécutables Rust et ce catalogue, mais pas le checkout source ou les fichiers bruts amont. Le workflow public ne clone jamais le dépôt privé.

Le catalogue est la première tranche de l'importeur, pas une map jouable. Le prochain travail technique porte sur les layouts et leur interprétation en tuiles/métatuiles, puis sur les collisions et objets d'évènement.

## Références

- pret/pokeemerald : source de référence pour structures et comportements Emerald. Ses données ne sont pas automatiquement redistribuables.
- emerald-3ds : référence sur le split d'affichage, l'extraction des données et le monde voxel. Le portage cible la 3DS, pas Bevy.
- kanto-gear : référence UX du compagnon Pokégear/C-Gear. Il s'agit d'un mod Lua destiné à Gen1Recomp, non d'un composant Dioxus. Ne copier aucune ressource avant vérification des crédits et licences.
