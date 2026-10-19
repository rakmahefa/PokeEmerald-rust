# Audit source et plan d'implémentation

Date de référence : 19 octobre 2026. Ce dossier sert de registre d'architecture vivant.

## Pourquoi ce dossier existe

L'application actuelle démarre, mais reste un prototype : terrain voxel généré artificiellement, avatar en cuboïdes et panneau compagnon statique. L'importeur produit un catalogue de cartes, pas les données nécessaires pour rendre une carte Emerald jouable.

Les décisions distinguent le domaine du jeu à conserver/porter de la couche matériel/presentation à remplacer. « GBA ancien » ne signifie pas que les règles de jeu sont obsolètes : Bevy remplace la plateforme et le renderer, pas les règles Emerald.

## Sources épinglées et observées

| Source | Révision | Fonction |
|---|---|---|
| [PokeEmerald-rust](https://github.com/rakmahefa/PokeEmerald-rust) | branche agent/project-foundation, commit 7c313cea0db99664048597e39bddfd35dbb27761 | Workspace et prototypes actuels. |
| [pret/pokeemerald](https://github.com/pret/pokeemerald) | pin 76463dac15cad36aca5e2b3f6366abf9e53f814f; snapshot privé observé au commit a89f73dbb2d97f76700e66cb66ccaf56261c3643 | Référence de gameplay, données, cartes et événements. |
| [ZallaxDev/emerald-3ds](https://github.com/ZallaxDev/emerald-3ds) | commit b198c08defb94081ae995796cf75ab8cdb51f2d0 | Référence de pipeline/map voxel et patch « visible wild »; cible initiale 3DS, pas Bevy. |
| [AverageConsumer/kanto-gear](https://github.com/AverageConsumer/kanto-gear) | commit c4232bea88bb32927e3ba8eb3406dd92408aeb93 | Référence UX/fonctionnalités, modèles, adaptateurs Gen 3 et tests. |

## Index

- 01-source-inventory.md — arbre source complet, taxonomie des fichiers sous src et ressources.
- 02-gba-bevy-categorization.md — que garder, porter, remplacer et retirer du runtime.
- 03-map-data-and-renderer.md — passer du catalogue au monde réel.
- 04-gameplay-and-event-systems.md — mouvement, collision, événements, scripts et combats.
- 05-overworld-wild-encounters.md — audit et port OWE.
- 06-kanto-gear.md — audit de la source Kanto Gear et stratégie d'intégration.
- 07-settings-input-audio-save.md — paramètres, entrée, audio, sauvegardes.
- 08-security-provenance-licensing.md — sécurité du paquet privé/public et licences.
- 09-decision-register.md — registre des décisions par fichier/sous-système.
- ../IMPLEMENTATION_ROADMAP.md — phases, gates et définition de « terminé ».

## Ce qui est réel et ce qui ne l'est pas

État du code : Rust 2024, quatre crates, Bevy 0.20.0 et Dioxus Desktop 0.7.10 épinglés; MapCatalog schéma 1; scène voxel procédurale; settings locaux non persistés. L'exécutable qui se lance est une preuve de compilation et de démarrage, pas de gameplay.

## Méthode et limites explicites

L'arbre Git du snapshot a été recensé en entier; les fichiers du sous-arbre src sont listés ci-dessous et classés une fois chacun. Les crates Rust, les principales interfaces publiques, le patch OWE, les documents d'architecture 3DS et les points d'entrée Kanto Gear ont été examinés. La classification initiale des 468 fichiers source est fondée sur leurs chemins et noms et doit être précisée à la migration. Les fichiers binaires du snapshot ne sont pas tous décodés visuellement ici; leur import doit produire validation et erreurs par fichier.

Chaque phase est terminée uniquement après ses tests et critères d'acceptation. Les fonctions non implémentées ne sont pas masquées par des données mock en mode normal.
