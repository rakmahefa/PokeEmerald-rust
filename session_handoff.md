# Session Handoff — PokeEmerald Rust

Dernière mise à jour : 2026-10-19

Ce fichier sert de point de reprise pour la prochaine session. Lire cette page en premier, puis vérifier les derniers commits et exécutions GitHub Actions avant de modifier le code : les branches et workflows peuvent évoluer après cette date.

## 1. Objectif du projet

Construire progressivement une application Rust inspirée de Pokémon Emerald, avec :
- **Bevy** pour le runtime/jeu ;
- **Dioxus** pour l’interface de paramètres ;
- une pipeline de données qui lit des métadonnées provenant d’un snapshot **pret/pokeemerald** épinglé ;
- un dépôt privé séparé pour les sources décompilées et les artefacts de build, sans distribuer la ROM.

**État important :** ce dépôt est encore un scaffold technique, pas un remake jouable. L’application Bevy contient encore un terrain voxel/procédural de test et un joueur placeholder. Le rendu des vraies cartes Emerald, les tilesets/sprites, collisions, déplacements finalisés, warps, scripts d’événements, combats et progression de jeu restent à implémenter. Les paramètres Dioxus sont un prototype local, pas encore une configuration persistante/synchronisée complète.

## 2. Dépôts, branche et PR

- **Dépôt public :** https://github.com/rakmahefa/PokeEmerald-rust
- **Branche de travail :** **agent/project-foundation**
- **PR ouverte :** https://github.com/rakmahefa/PokeEmerald-rust/pull/1
  - Titre : feat: scaffold Rust + Bevy runtime and Dioxus settings
  - Base : main
- **Dépôt privé des sources/assets :** https://github.com/rakmahefa/PokeEmerald-rust-private-assets
- **Dernier commit fonctionnel du code public avant l’ajout de ce fichier :** 2f78db00f3eb046a705ce1e132136b547f05cf2a
- **HEAD privé du workflow au dernier build réussi :** a89f73dbb2d97f76700e66cb66ccaf56261c3643
- **Commit du snapshot privé du répertoire décompilé :** 78717a88bdf309c6d5a154e07be2f42d3f02ef84

Le commit qui ajoute ce document est un commit de documentation supplémentaire sur agent/project-foundation. Consulter la tête de branche courante avant de supposer que 2f78db0 est toujours le dernier commit.

## 3. État des workflows — dernière vérification

### CI public : succès

Les deux exécutions CI ci-dessous sont **vertes** pour le commit de code 2f78db0 :
- https://github.com/rakmahefa/PokeEmerald-rust/actions/runs/39293737683
- https://github.com/rakmahefa/PokeEmerald-rust/actions/runs/39293726739

Le workflow valide le formatage Rust, vérifie le workspace et exécute les tests.

### Build Linux depuis le snapshot privé : succès

Le run privé **n° 4** est terminé avec succès :
- https://github.com/rakmahefa/PokeEmerald-rust-private-assets/actions/runs/39294798769
- Le workflow a bien utilisé le commit public 2f78db00f3eb046a705ce1e132136b547f05cf2a.
- Les étapes compilation, import du catalogue, assemblage du paquet, publication et upload de l’artefact sont toutes réussies.
- Catalogue produit et validé : **518 cartes dans 34 groupes**, schéma 1.
- Le manifeste du paquet confirme rom_included: false.

Artefact Linux :
- Nom : **pokeemerald-rust-linux-x86_64**
- Téléchargement : https://github.com/rakmahefa/PokeEmerald-rust-private-assets/actions/runs/39294798769/artifacts/12212180647
- Taille annoncée : 42 871 588 octets
- SHA-256 de l’archive : 1ecfafd16cc53d60606f65b45918872a957ed8204fac05088ca7d52c0eb558c6
- Expiration actuelle indiquée par GitHub : **2026-11-18 18:06 UTC**. Retélécharger/conserver l’artefact avant cette date s’il faut le garder.

Les anciens runs privés 39292398521 et 39280723281 ont échoué avant le succès final ; ils sont utiles pour l’historique du diagnostic, mais le run 39294798769 est la référence positive actuelle.

## 4. Snapshot privé et règles de sécurité des données

Pins documentés :
- pret/pokeemerald : 76463dac15cad36aca5e2b3f6366abf9e53f814f
- pret/agbcc : da598c1d918402c42c0c0d7128ba14567f3175e9
- SHA-1 ROM Emerald USA/Europe v1.0 attendu dans le processus de préparation : f3ae088181bf583e55daf962a92bb46f4f1d07b7

La préparation du snapshot a réussi dans le run :
https://github.com/rakmahefa/PokeEmerald-rust-private-assets/actions/runs/39264781440

Le fichier decomp/MANIFEST.json indique rom_included: false. L’historique Git normal a été réécrit après le bootstrap pour exclure la ROM ; le chemin du fichier ROM avait ensuite retourné 404 et aucun fichier .gba n’était présent à la racine. Cette vérification ne garantit pas l’effacement instantané de toutes caches GitHub inaccessibles ; si une purge physique de caches est nécessaire, contacter GitHub Support.

**Ne jamais ajouter la ROM, les dumps de ROM ou des assets propriétaires bruts dans le dépôt public ni dans l’artefact distribué.** Le paquet se limite aux exécutables/configurations nécessaires et au catalogue compact des métadonnées. Garder toute source ou ressource qui doit rester privée dans le dépôt privé.

## 5. Architecture et fichiers importants

Workspace Rust (racine Cargo.toml) :
- **crates/emerald-core** : types/logique de domaine du jeu ;
- **crates/emerald-data** : schéma, importateur et validation du catalogue de cartes ;
- **crates/emerald-game** : application Bevy et chargement facultatif de assets/map_catalog.json ;
- **crates/emerald-settings** : prototype d’interface de paramètres Dioxus.

Fichiers et documentation :
- **crates/emerald-data/src/lib.rs** : MapCatalog, MapSummary, validation et tests de l’importateur ;
- **crates/emerald-data/src/main.rs** : CLI nommée emerald-data ;
- **docs/DATA_PIPELINE.md** : pipeline du catalogue ;
- **docs/ARCHITECTURE.md** : structure de l’application ;
- **docs/ROM_SECURITY.md** : limites de distribution ;
- **docs/private-assets-template/.github/workflows/build-linux.yml** : modèle public du workflow à installer dans le dépôt privé ;
- **docs/private-assets-template/README.md** : instructions associées au dépôt privé.

Le catalogue contient les identifiants et groupes de cartes, les noms, layouts, quelques champs de métadonnées et des **comptes** d’événements. Il ne contient pas le contenu brut des événements, scripts, graphiques, tilesets ou données binaires des layouts.

## 6. Corrections de build importantes

Le dernier build réussi résout les problèmes suivants :

1. **Nom du binaire CLI :** le nom correct est **emerald-data**, pas emerald-data-import.
2. **Traçabilité des sources :** le checkout privé utilise fetch-depth: 0 pour que le workflow puisse identifier le commit qui a introduit le répertoire decomp/pokeemerald.
3. **Propriété Rust déplacée :** dans le runtime, le commit du catalogue est cloné quand il faut encore appeler starting_location().
4. **Collections null dans les map.json pret :** certains champs comme connections ou les listes d’événements peuvent être null, pas seulement absents ou des tableaux. Les champs bruts concernés sont représentés avec Option<Vec<...>>, puis les valeurs absentes/nulles sont normalisées en listes vides pour le résumé. MapSummary.connections reste un Vec<MapConnection>.
5. **Test de régression :** null_collections_are_treated_as_empty vérifie le cas où ces champs sont null.

Le correctif de typage final est dans le commit 2f78db00f3eb046a705ce1e132136b547f05cf2a. La CI verte et le build privé réussi confirment ensemble que ce correctif est compilable et que l’import du vrai snapshot fonctionne sur la version publiée.

## 7. Procédures utiles

### Vérifier le statut avant toute modification

1. Vérifier la tête de agent/project-foundation et l’état de la PR #1.
2. Consulter les workflows les plus récents dans les deux dépôts.
3. Lire les logs de l’étape qui a réellement échoué ; ne pas déduire la cause à partir du seul badge rouge.
4. Préserver la séparation dépôt public / dépôt privé.

### Lancer un nouveau build Linux

Workflow :
https://github.com/rakmahefa/PokeEmerald-rust-private-assets/actions/workflows/build-linux.yml

Choisir source_ref = agent/project-foundation (ou le ref désiré explicitement). Le workflow est déclenché depuis le dépôt privé et checkout le code public à ce ref.

**Ne pas relancer le workflow Prepare private pret source snapshot juste pour reconstruire l’application.** Il ne sert qu’à préparer/mettre à jour le snapshot privé.

### Commandes locales de validation

À la racine du dépôt public, utiliser la même séquence que la CI :

    cargo fmt --all -- --check
    cargo check --workspace --all-targets
    cargo test --workspace
    cargo build --workspace --release

Pour importer le catalogue localement avec un snapshot privé déjà préparé, le workflow exécute essentiellement :

    cargo run --release -p emerald-data --bin emerald-data --       --source ../decomp/pokeemerald       --output dist/package/assets/map_catalog.json       --pret-commit 76463dac15cad36aca5e2b3f6366abf9e53f814f

## 8. Prochaines étapes suggérées

1. **Relire la PR #1** et vérifier si le scaffold peut être fusionné ou s’il faut d’abord ajouter une checklist/description plus complète : https://github.com/rakmahefa/PokeEmerald-rust/pull/1
2. **Vérifier le livrable sous Linux** : télécharger l’artefact privé, extraire le paquet et exécuter les binaires dans un environnement Linux compatible ; le succès CI prouve le build, pas encore le bon fonctionnement interactif sur toutes les machines.
3. **Poursuivre le vrai jeu en incréments testables** : architecture de carte/tilemap, interprétation des layouts, rendu 2D/tile-based, collisions et transitions/warps. Éviter de prétendre que le projet est jouable avant une implémentation et des tests fonctionnels.
4. **Ajouter des tests d’import sur davantage de cas du snapshot** (champs absents/null, valeurs inattendues, références incohérentes) avant d’agrandir le schéma.
5. **Maintenir la pipeline privée explicite et minimale** : ajouter au manifeste les informations de provenance utiles sans incorporer le snapshot complet ou des actifs binaires dans l’artefact public.

## 9. Point de départ pour la prochaine session

Commencer par ce fichier, puis :
- vérifier la tête de branche et le statut de la PR ;
- vérifier que les deux liens de runs verts ci-dessus restent les derniers runs pertinents ;
- ouvrir docs/ARCHITECTURE.md et docs/DATA_PIPELINE.md ;
- choisir une prochaine tâche limitée avec critère de validation clair.

Ne pas reprendre le diagnostic des anciens échecs comme s’ils étaient encore ouverts : la CI publique et le build Linux privé ont tous deux terminé avec succès le 19 octobre 2026.
