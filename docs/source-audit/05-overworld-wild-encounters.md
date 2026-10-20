# OWE — Overworld Wild Encounters

## Source et détail

La référence est le patch patches/pokeemerald/0014-visible-wild.patch de [emerald-3ds](https://github.com/ZallaxDev/emerald-3ds), au-dessus du pin pret/pokeemerald 76463dac15cad36aca5e2b3f6366abf9e53f814f. Il est branché à la boucle CB1_Overworld sur la cible 3DS et ajoute l'essentiel de la logique visible à wild_encounter.c. Il dépend donc des tables/règles de rencontre et des structures de sprites/combat du jeu C d'origine; ce n'est pas un plugin Rust prêt à l'emploi.

Le patch déclare six entités visibles maximum, VW_MAX=6; marqueur VW_MAGIC=0x5657; zone d'apparition environ 11 cases en X et 6 en Y; seuil d'apparition minimale de 3 cases; distance d'oubli de 15. Son réglage de densité expose les valeurs {0,2,4,6}. Ce sont des références de la variante 3DS, à vérifier comme options produit avant de les geler.

## Règles à préserver

- Chercher les cellules candidates dans le voisinage, puis tester le véritable comportement de métatuile pour distinguer terre et eau; ne pas faire apparaître une rencontre terrestre sur l'eau.
- Utiliser la table du lieu et sa méthode pour choisir l'espèce/le niveau; appliquer les conditions de niveau de Repel.
- Tenir compte du Surf, des conditions spéciales de lieu et de la désactivation des encounters.
- Garder espèce, niveau, personnalité/seed et shiny cohérents sur toute la durée de l'entité; apparence et animation restent présentation.
- Expirer les entités trop éloignées, réconcilier quota/temps de spawn et nettoyer lors des changements de map ou de désactivation.
- Le contact/interception commence une bataille pour le mon visible déjà choisi; éviter un second tirage indépendant.
- Gérer la durée de vie des ressources de présentation avec handles/assets Bevy et ECS, et non avec les compteurs de palettes/icônes GBA.

Le détail exact du timer, des préconditions et du choix de niveau doit être transcrit depuis le patch dans des tests de parité avant de marquer OWE terminé; les constantes ci-dessus ne sont pas une spécification complète.

## Cible ECS

Composants proposés : VisibleWildEncounter (espèce, niveau, personnalité/seed, source de table, terrain, statut); GridPosition; WildPresentation (handles, idle/shiny); SpawnOrigin/Distance; EncounterEligibility. Un EncounterDirector décide quota, timers, candidates et nettoyage. Le BattleRequest reçoit l'identité déjà portée par l'entité visible.

## Ordre d'implémentation et gates

1. Importer et tester les tables normales de rencontres (sans affichage).
2. Relier TileGrid aux comportements land/water et au Repel.
3. Faire apparaître les entités en mode debug, en validant quota/zone/timer/expirations.
4. Ajouter une présentation temporaire clairement marquée, puis sprites d'espèce si droits/source validés.
5. Relier contact à la machine réelle de combat, une seule fois.
6. Tester quotas, RNG, methods, eau/terre, Surf, Repel, maps, disable, timer et nettoyage en headless.

Ne pas copier CreateMonIcon, Sprite, palette APIs, MapGridGetMetatileBehaviorAt ou CB1_Overworld dans Rust : ils appartiennent au runtime GBA/3DS. Porter les décisions métier et les rendre sous Bevy.
