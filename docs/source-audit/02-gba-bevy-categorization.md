# Catégorisation GBA → Rust/Bevy

## Règle d'architecture

Ne pas convertir mécaniquement chaque API C en API Rust. Garder les données, règles et contraintes de gameplay; réécrire les règles dans un domaine testable; laisser Bevy gérer les schedules, ECS, rendu, input, assets et audio desktop.

## Légende des décisions

- KEEP : garder la sémantique ou le format utile.
- PORT : réimplémenter le comportement en Rust avec tests de référence.
- REPLACE : remplacer un backend GBA par une abstraction desktop Bevy.
- REMOVE-RUNTIME : code requis éventuellement pour l'analyse/build original, mais non pour l'application.
- REVIEW : décision dépendante de la provenance, des droits ou des dépendances.

## Matrice de décision

| Code/source | Décision | Cible |
|---|---|---|
| AgbMain, CB1/CB2, tâches et mode overworld | PORT | états et plugins Bevy; ordre explicite des transitions. |
| overworld.c, field_control_avatar.c, field_player_avatar.c | PORT | commandes de déplacement, avatar logique, interpolation de présentation. |
| fieldmap.c, metatile_behavior.c, tilesets.c, layouts/connexions | KEEP + PORT | grille, métatuiles, attributs de collision, élévation, liens de maps. |
| event_object_movement.c, event_data.c, coord_event_weather.c, script.c, scrcmd.c | PORT | objets/NPC, drapeaux/variables, triggers et VM d'événements. |
| wild_encounter.c et tables | KEEP + PORT | moteur de rencontres canonique et couche d'entités OWE. |
| battle_setup.c, battle_main.c et modules battle_* | KEEP + PORT | règles de combat, RNG, tours, effets, sortie du combat. |
| pokemon.c, équipe, moves, objets, évolution et Pokédex | KEEP + PORT | types domaine, règles et progression versionnés/testables. |
| save.c, load_save.c, flags/vars, inventaire et localisation | PORT | DTO de sauvegarde; ne pas sérialiser des entités/pointeurs Bevy. |
| pokemon_icon.c, sprite.c, bg.c, graphics.c, palette.c, GPU regs, VRAM/OAM | REPLACE | textures/atlas, sprites, meshes, matériaux, UI Bevy. |
| DMA, registres I/O, appels BIOS/ABI, VBlank, interruptions matérielles | REMOVE-RUNTIME | aucun équivalent direct; ne garder les parseurs de format utiles que s'il y en a. |
| m4a.c, mixer GBA, registres audio et échantillons bas niveau | REPLACE | audio Bevy alimenté par des ressources préparées, avec événements/boucles du jeu. |
| entrées/boutons GBA | REPLACE | actions logiques configurables, clavier/manette et focus UI. |
| RTC, wallclock, clock berries, événements temporels | PORT | horloge métier indépendante, testée séparément de l'horloge machine. |
| src/data, data/maps, layouts, tilesets, tables d'espèces | KEEP AS INPUT → TRANSFORM | schema/importer déterministe; paquet de runtime versionné. |
| asm, libagbsyscall, linker, ROM headers, build makefiles | REMOVE-RUNTIME / KEEP PRIVATE TOOLING | hors exécutable Bevy, peuvent rester nécessaires au bootstrap de référence. |
| Link/RFU, e-Reader, Mystery Gift, Frontier, Contest, mini-jeux | REVIEW / PORT LATER | conserver dans le registre; prioriser après la tranche de jeu de base. |

## Cible par crate

- emerald-core : modèles métier purs, IDs, coordonnées, direction, session, état du jeu, RNG logique et commandes; aucune dépendance Bevy.
- emerald-data : formats et importeurs versionnés, validation des références, des bornes et de la provenance; catalogue actuel conservé comme index seulement.
- emerald-game : runtime Bevy, plugins app/world/map/player/collision/events/encounters/battle/save/ui/audio/input.
- emerald-settings : fenêtre Dioxus indépendante, options validées et persistées. Le paramètre est considéré fonctionnel uniquement s'il modifie réellement le runtime.

## Frontière du portage

Le renderer GBA n'est pas le modèle de collision. Les métatuiles et leur sémantique définissent la jouabilité; meshes et animations ne sont que la projection. Les scripts amont donnent les résultats de scénario à reproduire; les callbacks et pointeurs globaux sont remplacés par des états/commandes. Les assets sont traités par un pipeline, pas chargés/décodés chaque frame.

Aucun module ne passe à REMOVE sans raison écrite : non requis sur la cible, remplacé par une implémentation testée, hors périmètre explicitement convenu ou exclu pour sécurité/redistribution.
