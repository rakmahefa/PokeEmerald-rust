# Spécification — PokeEmerald-rust

- Cible principale : desktop Linux x86_64.
- Runtime : Rust et Bevy.
- Réglages desktop : Dioxus Desktop.
- État : socle compilable, prototypes visuels ; pas de gameplay complet.
- Source de référence : décompilation publique [pret/pokeemerald](https://github.com/pret/pokeemerald), épinglée à un commit connu dans le workflow privé.

## 1. Vision du produit

Créer une expérience desktop inspirée de Pokémon Emerald, avec une zone de jeu 3D voxel et un panneau compagnon, en implémentant le runtime et l'expérience utilisateur en Rust/Bevy. Dioxus fournit une fenêtre de configuration moderne.

Ce projet n'est pas encore un portage de jeu fonctionnel. Le code C de pret/pokeemerald ne s'exécute pas automatiquement dans Bevy : les données, la logique gameplay et le rendu doivent être intégrés au runtime Rust ou via une stratégie d'interopération formellement choisie.

## 2. UX cible

### Fenêtre de jeu Bevy

1. Zone principale : monde voxel, environnement, bâtiments, avatar, caméra, collisions, interactions et transitions.
2. Zone compagnon : interface inspirée de Kanto Gear affichant l'état réel du jeu — Pokédex, équipe, carte, inventaire, outils, notes et progression.

Dans le prototype actuel, un terrain de blocs générés et un panneau compagnon statique représentent cette disposition. Le terme « deux écrans » signifie deux zones visuelles dans la même fenêtre ; une fenêtre séparée est une amélioration possible ultérieurement.

### Fenêtre Dioxus

Les réglages cibles comprennent le rendu voxel, audio, caméra, échelle UI, visibilité du compagnon et chemin des données. Les options actuelles sont uniquement locales à la fenêtre de réglages ; persistance et communication IPC ne sont pas encore implémentées.

## 3. Architecture

- `crates/emerald-core` : modèles et contrats sérialisables.
- `crates/emerald-game` : runtime Bevy, monde, contrôles et compagnon.
- `crates/emerald-settings` : application Dioxus Desktop.
- dépôt privé séparé : source upstream épinglée/snapshot, métadonnées de version et données locales supplémentaires si nécessaires.

Le dépôt privé ne doit pas être une dépendance secrète des workflows CI du dépôt public. Un workflow exécuté dans le dépôt privé peut lire son propre contenu avec son `GITHUB_TOKEN` et cloner les dépôts publics sans PAT inter-dépôts.

## 4. Réalité du pipeline pret

`pret/pokeemerald` est **déjà la décompilation source**. Le workflow ne convertit pas le fichier ROM en nouveaux fichiers C : il vérifie le dump fourni, récupère un commit upstream déterminé, construit la ROM de référence depuis ce code et exécute `make compare`. Une archive `git archive` des fichiers source suivis capture ensuite le code et les ressources versionnées ; elle exclut les résultats de build non suivis comme la ROM générée.

Commit de référence initial :
- pret/pokeemerald : `76463dac15cad36aca5e2b3f6366abf9e53f814f`
- pret/agbcc : `da598c1d918402c42c0c0d7128ba14567f3175e9`
- SHA-1 ROM Emerald USA/Europe v1.0 attendu : `f3ae088181bf583e55daf962a92bb46f4f1d07b7`

Ces versions sont des pins initiaux reproductibles, pas une promesse de compatibilité de chaque commit amont futur. Le manifeste du snapshot doit conserver les pins et les checksums.

## 5. Flux privé en deux temps

### Bootstrap — lancement manuel dans le dépôt privé

1. Le workflow lit le dump temporaire téléversé à la racine du dépôt privé, calcule son SHA-1 sans afficher ses octets et refuse les versions non prises en charge.
2. Il clone les commits épinglés de pret/pokeemerald et pret/agbcc, construit la ROM de référence et exécute `make compare`.
3. Après une comparaison réussie, il crée une archive et un manifeste ROM-free.
4. Le workflow supprime ensuite le dump des fichiers courants et réécrit l'historique Git de `main` pour retirer ce chemin ; il stocke les sources suivies dans `decomp/pokeemerald/`.
5. Il publie un artefact Actions et une release **du dépôt privé uniquement** avec archive, manifeste et checksum.

Aucun passphrase ni PAT inter-dépôts n'est requis pour ce flux : l'entrée initiale est actuellement un fichier ROM temporaire dans le dépôt privé ; les builds quotidiens n'en ont plus besoin après le snapshot. Le dépôt est privé et le workflow n'inclut jamais la ROM dans les sorties. Une réécriture Git retire le fichier de l'historique accessible normalement, mais des objets Git inaccessibles peuvent rester dans les caches de GitHub pendant un certain temps.

### Développement quotidien

Après la création du snapshot, les builds ordinaires consomment le snapshot privé et le code Rust public sans recharger ni redéchiffrer la ROM. Le workflow de développement qui consomme les assets doit s'exécuter dans le dépôt privé ; son jeton courant peut lire ses propres releases/assets.

Le prototype Bevy ne lit pas encore ces données. Avant qu'elles soient intégrées au runtime ou à une archive finale, un importeur explicite et des tests de format devront être développés.

### Release finale

Le build Rust peut, en principe, être compilé sans ROM dès que les données requises sont disponibles. Si le produit choisi doit extraire les données depuis la ROM de chaque utilisateur, la ROM devra être fournie et traitée localement au premier lancement ou pendant une commande d'import explicite. Ne pas publier la ROM ou un paquet contenant des données protégées sans une évaluation distincte des droits et du modèle de distribution.

## 6. CI et distribution

- CI publique : formatage, compilation et tests ; aucun accès aux secrets ROM.
- Release publique actuelle : archive bootstrap des exécutables uniquement, aucune ROM et aucune ressource dérivée.
- Workflow privé de préparation : validation ROM, reconstruction de référence, snapshot de sources et checksum dans le dépôt privé.
- Workflow privé de développement : checkout du projet Rust public et accès au snapshot privé depuis la même exécution.
- Chaque archive est vérifiée avec SHA-256 ; les artefacts Actions expirent selon la rétention configurée, alors que le snapshot de release privée est le stockage durable du flux.

## 7. Sécurité et droits

- Le bootstrap a été conçu pour utiliser le dump temporaire déjà téléversé dans le dépôt privé et pour le supprimer après la comparaison réussie.
- Ne jamais placer une ROM en clair dans le dépôt public, un artefact public ou une release publique.
- Ne pas déclencher le workflow avec ROM depuis une branche non fiable ou une PR.
- Utiliser des permissions minimales et limiter l'accès au dépôt privé aux comptes de confiance.
- La connexion GitHub de l'assistant et le `GITHUB_TOKEN` Actions sont deux mécanismes distincts : l'accès de l'assistant à un dépôt privé ne donne pas à un workflow public le droit de le cloner.
- Le contenu source et les assets upstream conservent leurs licences propres ; ne pas appliquer une licence globale avant revue.

## 8. Roadmap

1. Bootstrap source privé reproductible.
2. Importeur et tests sur données ROM-free non sensibles.
3. Lecture des données/cartes et scène voxel réelle dans Bevy.
4. Contrôles, collisions, interactions, sauvegardes.
5. Pokémon overworld natifs et synchronisation de l'état compagnon.
6. Réglages persistés et communication Dioxus ↔ Bevy.
7. Packaging Linux et décision de distribution finale.
