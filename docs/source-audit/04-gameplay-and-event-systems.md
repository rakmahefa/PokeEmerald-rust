# Gameplay, événements et progression

## Domaines à porter

| Source de référence | Cible |
|---|---|
| overworld.c, field_control_avatar.c, field_player_avatar.c | OverworldState, commandes de mouvement, avatar logique et interpolation visuelle. |
| fieldmap.c, metatile_behavior.c, tilesets/layouts | TileGrid, attributs de métatuile, élévation, terrain, collision et conditions de passage. |
| event_object_movement.c, event_object_lock.c | entités d'objets/NPC et états de déplacement/lock. |
| event_data.c, coord_event_weather.c, script.c, scrcmd.c | flags, variables, triggers et VM d'événements avec attentes non bloquantes. |
| wild_encounter.c et encounter tables | choix méthode/terrain/heure/conditions et résultats RNG. |
| battle_setup.c, battle_main.c, battle_script_commands.c, battle_controllers.c | machine d'état combat, actions/tours/effets, résultats et retour au terrain. |
| pokemon.c, moves, stats, party, evolution, dex | types espèce/instance, équipe, combat/capture, progression et Pokédex. |
| item.c, item_use.c, item_menu.c, shop.c, PC/storage | commandes métier vérifiées sur l'inventaire/objets. |
| save.c, load_save.c, rtc.c | SaveGame versionné, migration, vérification des références et horloge. |
| field_weather.c, region_map.c, pokenav | état météo, cartes/régions et projections pour UI companion. |

Le sous-arbre src/data et les tables ne sont pas « vieux code à supprimer » : les éléments utiles deviennent des données typées/importées. Les fonctions spécifiques Link/RFU, e-Reader, Frontier, Contest et mini-jeux sont classées REVIEW/LATER, pas supprimées sans examen.

## Contrats importants

- Position canonique : mapId + x + y + élévation + direction. Transform Bevy est visuel uniquement.
- Déplacement = tentative de pas logique, collision calculée à partir des métatuiles/objets, commit atomique puis émission d'événements. Un pas refusé ne modifie aucun état métier.
- Les warps/connexions valident la destination avant transition; aucune double map active si le chargement échoue.
- Les événements sont typés. Une VM traite dialogue, choix, délai, déplacements, combat et conditions; aucun script ne bloque la frame. Les opcodes non pris en charge sont listés dans un rapport.
- Flags/variables persistants sont séparés des locks/transitoires.
- RNG de jeu déterministe/injectable; ne jamais lancer un tirage pendant le rendu ou chaque frame.
- Combat suspend le contrôle d'overworld; la sortie résout expérience/capture/évolution/objets/events puis restaure la carte.
- Sauvegarde = DTO versionné, pas snapshot ECS ni références d'assets.

## Tranche verticale de référence

Bundle de données validé → vraie carte → spawn joueur → déplacement discret → collision → interaction/warp réel → sauvegarde/rechargement → tests headless. Une seule carte testée de bout en bout est plus utile que des dizaines de modules non intégrés. Après cette preuve, étendre à davantage de maps, de scripts, de Pokémon et de systèmes.
