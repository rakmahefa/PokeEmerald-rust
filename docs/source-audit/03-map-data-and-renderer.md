# Maps, layouts et renderer réel

## État actuel

emerald-data lit data/maps/map_groups.json et les map.json associés. Le catalogue schéma 1 enregistre IDs, groupes, layout symbolique, métadonnées, connexions et comptages d'événements. Il n'inclut pas le layout binaire, les pixels de tilesets, palettes, bordures, scripts ou contenu d'événements. 518 cartes indexées ne signifient pas 518 cartes rendues/jouables.

## Pipeline à construire

1. Lire l'index des groupes et toutes les map.json; vérifier IDs, groupe, connexion, limites et champs requis.
2. Décoder layouts.json/layouts.inc et data/layouts/*/map.bin + border.bin : dimensions, cellules, ordre des octets, IDs et tailles maximales.
3. Résoudre la paire de tilesets primaire/secondaire et importer tiles.png, metatiles.bin, metatile_attributes.bin, palettes et animations.
4. Importer la définition des événements, des objets/NPCs, des warps, coordinate/background events et références de scripts.
5. Importer les tables d'encounters et leurs conditions dans un schéma de domaine distinct.
6. Produire un bundle versionné avec manifest/provenance, hash du contenu et erreurs de validation par fichier.
7. Charger le bundle dans Bevy sans dépendance au checkout pret; conserver le snapshot source privé.

Les layouts source doivent être lus selon la définition upstream à la révision épinglée; ne pas deviner les structures à partir de noms de fichiers. Définir des fixtures et tests pour dimensions, bounds, métatile connu, terrain/eau, warp et résolution d'une connexion.

## Modèle logique et projection 3D

- Vérité gameplay : coordonnées entières map/x/y/élévation et métatuiles.
- Projection visuelle : meshes/atlas/materials Bevy, joueur, objets et caméra.
- Collision et OWE consultent exactement les mêmes attributs de cellules.
- Préparer les assets hors schedule de rendu et mettre en cache layouts/tilesets.
- Regrouper géométrie par chunk/couche plutôt qu'une entité par face; mettre à jour seulement quand la carte/chunk/animation change.
- Premier affichage diagnostique top-down avec overlay de collision; après validation, projection 3D voxel sur les mêmes données.
- En mode jeu, échec du chargement = diagnostic visible et arrêt de la scène jouable, jamais substitution silencieuse d'un terrain synthétique.

## Référence Emerald 3DS

emerald-3ds/src/voxel/voxel_world.c illustre la résolution de payloads, les instances de maps, les connexions, les attributs de métatuiles et le chargement borné. scripts/gen_map_data.py externalise map.bin et border.bin sous forme de descripteurs/payloads. Cette logique est un guide algorithmique, pas une implémentation directement liée : le backend cible Citro3D/3DS, tandis que Bevy utilise sa gestion de ressources/meshes. L'état amont du voxel est partiel et doit être mesuré map par map.

## Schéma de bundle recommandé

Manifest avec schema version, pins upstream, catégories de fichiers, hash et comptages. MapDefinition avec id, dimensions, layout, tilesets, règles de déplacement, connexions et map events. LayoutGrid avec cellules et border. MetatileDefinition avec collision/élévation/couches/animation. TilesetBundle avec atlas/palettes/attributs. ScriptTable avec déclencheurs et opcodes référencés. EncounterTable avec méthodes, zones et entrées. Toute référence non résolue ou opcode inconnu est reporté explicitement.

Ne pas publier le bundle du jeu ou les données brutes issues de ROM dans le dépôt public avant une décision de droits indépendante.
