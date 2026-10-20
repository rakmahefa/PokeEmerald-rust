# Sécurité, provenance et droits

## Ne pas confondre les preuves

- rom_included: false signifie que le manifeste indique l'absence du dump Emerald principal du paquet considéré.
- Cela ne prouve pas que le snapshot source privé ne contient aucune donnée binaire du jeu.
- Cela ne décide pas des droits de redistribuer les données, graphismes, sons ou code amont.
- Le workflow décrit du paquet actuel livre binaires Rust et catalogue de métadonnées, sans checkout de source ni layout/sprite/script.

## Trois fichiers .gba amont suivis dans le snapshot

| Chemin | Taille | SHA de blob Git |
|---|---:|---|
| decomp/pokeemerald/data/mb_berry_fix.gba | 15 348 octets | 0afff07f5d2314e2bd35d99de518cbc114009b5e |
| decomp/pokeemerald/data/mb_colosseum.gba | 163 840 octets | fdeb854a054ba02191f6a209ec5b649f21c4e335 |
| decomp/pokeemerald/data/mb_ereader.gba | 12 512 octets | 85057cf4e2bd657da5d632ac12d1ea24aeaa4499 |

Ce sont des blobs MultiBoot amont, distincts du dump Emerald principal. Ils doivent rester hors du paquet public. Ne pas supprimer les blobs de la source de référence sans prouver que le build amont continue à fonctionner.

## Contrôles à mettre dans les workflows

- CI publique sans accès/secrets du dépôt privé; le workflow public ne clone pas le dépôt privé.
- Allowlist de fichiers distribuables dans la release; refuser par défaut ROM/dumps, .sav, .gba, paquets d'assets, fichiers extraits, secrets, assets inconnus et fichiers de test.
- Manifeste de sortie listant chaque fichier réel, taille, SHA256 et provenance.
- Test négatif qui injecte un fichier interdit dans le staging et vérifie que le packaging échoue.
- Garder les pins pret/pokeemerald et agbcc, checksums du snapshot et du bundle; distinguer données source, fichiers convertis et code original du port.
- Les réécritures d'historique ne garantissent pas la purge instantanée de chaque objet/caches de l'hébergeur.
- Les fichiers de source amont peuvent rester dans le dépôt privé; ils ne sont pas une autorisation de redistribution.

## Attribution

Le projet pret/pokeemerald ne pose pas une licence globale qui rendrait tous les contenus Pokémon librement redistribuables. Le LICENSE-PORT d'Emerald 3DS couvre uniquement certaines additions de ce port et ne relicencie pas le contenu Pokémon. Sa notice voxel crédite une logique issue de pokeemerald-multiplatform sous MIT. Kanto Gear annonce MIT pour son code, mais les fontes/atlas et ressources tierces gardent leurs notices/licences séparées. Le workspace Rust est actuellement UNLICENSED; ne pas déclarer une licence globale avant inventaire et décision explicites.

Voir aussi docs/ROM_SECURITY.md et docs/private-assets-template. Ce document est une checklist d'ingénierie et de provenance, pas un avis juridique.
