# Pipeline de données — première tranche

## Ce qui est implémenté

Le crate Rust emerald-data importe data/maps/map_groups.json et les fichiers data/maps/<MapName>/map.json depuis une copie précise de pret/pokeemerald. Il produit un catalogue JSON versionné contenant les identifiants des cartes, leurs groupes, layouts symboliques, connexions, options de déplacement et nombres d'évènements par catégorie.

Le programme refuse les noms de chemin inattendus, les fichiers JSON surdimensionnés, les doublons d'identifiants et les catalogues incohérents. Le hash complet du commit amont est enregistré pour rendre chaque artefact traçable. Des tests couvrent l'import, les comptages d'évènements, la validation du hash, les fichiers manquants et la sérialisation.

## Exécution locale

Depuis la racine du workspace, avec une copie locale de la source décompilée :

~~~bash
cargo run -p emerald-data --bin emerald-data-import -- \
  --source /chemin/vers/pokeemerald \
  --output ./dist/assets/map_catalog.json \
  --pret-commit 76463dac15cad36aca5e2b3f6366abf9e53f814f
~~~

## Livraison privée

Le workflow privé construit l'application puis génère ce catalogue depuis decomp/pokeemerald. L'archive Linux privée contient les exécutables, le catalogue de métadonnées et les documents de lancement. Elle n'intègre ni ROM, ni checkout source, ni layout binaire, script, tileset ou sprite.

Le runtime valide le catalogue au démarrage et affiche son nombre de cartes, son commit amont et la salle de départ cible dans le panneau compagnon. Le terrain Bevy reste une scène procédurale de test : la lecture des layouts binaires, les collisions, le rendu des tilesets, les objets/interactions et les transitions entre cartes constituent la prochaine tranche.

Le catalogue est dérivé de ressources du projet amont. Il demeure dans le dépôt et les artefacts privés jusqu'à l'établissement d'une politique de redistribution adaptée.
