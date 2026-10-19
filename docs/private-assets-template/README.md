# Template pour PokeEmerald-rust-private-assets

Ce dossier est un modèle à copier à la **racine** du nouveau dépôt GitHub privé `rakmahefa/PokeEmerald-rust-private-assets`. Ce répertoire template dans le repo public ne contient aucun dump ROM ni aucune clé.

## Fichiers à copier à la racine privée

- `.github/workflows/prepare-pret-decomp.yml` — bootstrap manuel. Requiert `rom/emerald.gba.gpg` et le secret Actions `ROM_GPG_PASSPHRASE`. Publie un snapshot ROM-free dans une release privée et un artefact Actions.
- `.github/workflows/build-linux.yml` — build de développement sans ROM, exécuté dans le contexte du dépôt privé ; utilise le snapshot privé quand il existe.
- `upstream.lock` — pins précis pour pret/pokeemerald et pret/agbcc.
- `.gitignore` — ignore le fichier ROM en clair tout en autorisant le fichier chiffré.

## Préparer les entrées

~~~bash
mkdir -p rom
gpg --symmetric --cipher-algo AES256 --output emerald.gba.gpg emerald.gba
mv emerald.gba.gpg rom/emerald.gba.gpg
~~~

La dernière commande suppose que le fichier chiffré se trouve dans le répertoire courant. Ajoute au dépôt privé **uniquement** `rom/emerald.gba.gpg`. Ajoute le passphrase comme Actions secret `ROM_GPG_PASSPHRASE`, jamais dans le repo.

## Limite actuelle

pret/pokeemerald est déjà un dépôt de décompilation. Le bootstrap contrôle la ROM fournie et reconstruit une version amont épinglée ; il ne décompile pas automatiquement une ROM en code C. Le snapshot privé n'est pas encore consommé par les systèmes de jeu Bevy : l'importeur Rust sera développé ensuite.
