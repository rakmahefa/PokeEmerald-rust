# Workflows privés — template

Ces workflows appartiennent au dépôt privé rakmahefa/PokeEmerald-rust-private-assets ; le dossier public ne les exécute pas.

- Prepare private pret source snapshot : vérifie la ROM temporaire, reconstruit la source pret épinglée et retire la ROM du chemin et de l'historique Git normal avant de conserver la source sans ROM.
- Build Linux from private snapshot : compile le workspace Rust, importe les métadonnées des cartes en catalogue JSON, puis publie un paquet Linux privé qui ne contient ni ROM ni arbre source. Les layouts binaires, scripts, tilesets et sprites ne sont pas encore importés.

Le dossier du template sert de référence documentaire uniquement. Le workflow public Linux ne lit jamais le dépôt privé.
