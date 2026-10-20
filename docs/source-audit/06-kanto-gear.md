# Kanto Gear — audit source et plan d'intégration

## Révision observée

[Kanto Gear](https://github.com/AverageConsumer/kanto-gear), commit c4232bea88bb32927e3ba8eb3406dd92408aeb93 : 509 entrées Git, undefined blobs, 378 fichiers Lua et 155 fichiers dans l'arborescence des tests. La source est un mod Gen1Recomp, pas une bibliothèque UI indépendante.

Les adaptateurs Gen1/2/3 dépendent du host : session, save, modèles natifs, menu layers, hooks/pointeurs et fonctions exposées par Gen1Recomp. Ces API ne sont pas celles d'un runtime Bevy. Le code Lua ne sera donc pas « intégré » en ajoutant une dépendance; on portera les comportements et les concepts.

## Modules observés

| Sous-arbre | Domaine | Choix |
|---|---|---|
| core | contrats, navigation, palette, découverte, outils de terrain, performance | Porter les idées et tests utiles vers Rust pur. |
| adapters/gen3/models | projection de bataille, Pokémon, graphismes, état et monde | Créer nos propres DTO depuis GameSession. |
| adapters/gen3/ui et menus | pages/états de menus, bag, party, moves, shop, storage | Catalogue des pages, actions et conditions préalables. |
| map, region, progress | carte, régions, badges de progression | Requêtes de lecture sur les mêmes données de jeu. |
| runtime | input/touch, frame, navigation, dialogue, inventaire, dex, party et menus | Bevy systems/resources/events, pas hooks host. |
| models/features | résumés, évolution, habitats, succès, calendrier, daycare, arbres, notes, PSS/gifts | Développer après le socle carte/partie/sauvegarde. |
| ui, i18n, phone | thèmes, typographie, langues et écran navigateur facultatif | Refaire sous Bevy UI; fenêtre téléphone différée. |
| tests | cas fonctionnels, compatibilité, régressions | Porter les scénarios pertinents, exclure les fixtures host-specific. |
| assets, fonts, credits | icônes et fontes | Vérifier provenance et licence individuellement avant copie. |

## Catalogue fonctionnel à reprendre dans l'ordre

1. Accueil/Trainer overview basé sur l'état réel.
2. Carte et Explorer local (map active, rencontres par méthode, objets/dresseurs lorsqu'ils existent).
3. Équipe et résumé : PV, statut, expérience, types, moves et objets tenus.
4. Sac : poches, quantités et actions contextuelles.
5. Pokédex, évolution et habitats.
6. Trainer Card, progression/stamps, notes et compteur de pas.
7. Day-Care, Calendar, Field Kit et outils de terrain.
8. Options discovery Vanilla/Guide/Everything/Custom, thèmes, langues et personnalisation.
9. PSS, fonctionnalités online, store d'extensions et navigateur téléphone après la stabilité solo.

## Architecture côté Rust

Pour la première version, afficher le compagnon dans **la même fenêtre Bevy**. Bevy/emerald-core reste la source de vérité; la UI lit des snapshots de jeu et émet des commandes typées avec validation. Pas de deuxième état de sauvegarde côté compagnon, pas de lecture de mémoire/pointeurs. Dioxus reste la fenêtre distincte des paramètres.

## Licences

Le code Kanto Gear annonce MIT; cette licence ne couvre pas automatiquement toutes les fontes, atlas générés, sprites et contenus Pokémon. Les fichiers ASSET_CREDITS et notices sous fonts contiennent des cas distincts. Porter la structure/UX et réécrire les widgets est le point de départ le plus sûr; conserver les notices obligatoires de tout code/asset effectivement adapté.
