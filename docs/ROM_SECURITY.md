# Procédure ROM privée pour GitHub Actions

Cette procédure garde le fichier clair hors du dépôt public et hors des artefacts de build. Elle ne confère aucun droit sur la ROM ou les données dérivées : utilise uniquement un dump que tu es autorisé à utiliser et respecte les règles applicables.

## Architecture

1. Le dépôt public rakmahefa/PokeEmerald-rust contient uniquement code et workflows.
2. Un dépôt distinct, par exemple rakmahefa/PokeEmerald-rust-private-assets, est configuré **Private**.
3. Le dépôt privé stocke uniquement emerald.gba.gpg, la ROM chiffrée localement.
4. Le passphrase GPG est stocké dans les Actions secrets du dépôt principal, jamais dans Git.
5. Un lancement manuel depuis main peut lire le dépôt privé, déchiffrer dans RUNNER_TEMP, vérifier l'empreinte, compiler puis supprimer le fichier clair avant l'assemblage de l'artefact.
6. Le workflow n'ajoute la ROM ni au tarball, ni au SHA256SUMS, ni à un GitHub Release. Le scaffold actuel ne consomme pas encore la ROM dans un importeur de jeu.

La ROM chiffrée reste un fichier sous contrôle privé sur GitHub. Si tu souhaites qu'aucune copie, même chiffrée, ne quitte ta machine, il faudra remplacer cette méthode par un environnement de build privé/self-hosted correctement isolé. Les runners self-hosted ne doivent pas exécuter les workflows de pull requests non fiables.

## Étape A — chiffrer localement

Sur ta machine, là où se trouve ton dump légal emerald.gba :

~~~bash
gpg --symmetric --cipher-algo AES256 --output emerald.gba.gpg emerald.gba
~~~

Choisis un passphrase long, unique et stocke-le dans un gestionnaire de mots de passe. Ne partage pas la ROM ni le passphrase dans cette conversation. Vérifie localement que le fichier chiffré peut être déchiffré avant de supprimer une copie temporaire.

**Ne place pas emerald.gba dans le dépôt public.** Ajoute les fichiers ROM et chiffrés au .gitignore local même si le dépôt privé est utilisé.

## Étape B — dépôt privé d'assets

1. Crée PokeEmerald-rust-private-assets sur GitHub et choisis explicitement la visibilité **Private**.
2. Ajoute uniquement emerald.gba.gpg à la racine du dépôt privé.
3. Vérifie qu'aucune action de ce dépôt ne déchiffre ou ne publie la ROM.
4. Évite d'ajouter des workflows qui affichent le contenu du dump ou ses données dans des logs publics.

Le workflow cherche exactement private-assets/emerald.gba.gpg.

## Étape C — configuration du dépôt principal

Dans rakmahefa/PokeEmerald-rust → **Settings → Secrets and variables → Actions** :

Créer une **variable** :
- ROM_ASSET_REPOSITORY : rakmahefa/PokeEmerald-rust-private-assets (remplacer si le nom diffère).

Créer des **secrets** :
- ROM_ASSET_READ_TOKEN : Fine-grained Personal Access Token limité au seul dépôt privé d'assets, permissions Repository contents = **Read-only**. Aucune permission d'écriture.
- ROM_GPG_PASSPHRASE : le passphrase utilisé au chiffrement GPG.

Le dépôt privé doit autoriser ce jeton à lire son contenu. Ne mets jamais le token ou le passphrase dans un fichier versionné, une variable non secrète, une commande imprimée ou un commentaire d'issue.

## Étape D — exécuter le workflow

1. Ouvre **Actions → Linux release artifact → Run workflow**.
2. Sélectionne la branche main.
3. Active use_private_rom.
4. Lance le workflow et vérifie que l'étape de validation réussit.
5. Télécharge pokeemerald-rust-linux-x86_64 puis vérifie son checksum.

L'empreinte actuellement acceptée est le SHA-1 f3ae088181bf583e55daf962a92bb46f4f1d07b7, identifiée par pret/pokeemerald et emerald-3ds comme Pokémon Emerald (USA/Europe) v1.0. Une autre empreinte sera refusée jusqu'à ajout explicite et documenté d'une version supportée.

Le workflow manuel avec ROM ne publie pas de GitHub Release. La release automatique sur tags v* est limitée au package bootstrap sans ROM.

## Limites et précautions

- Les secrets GitHub sont limités à 48 KB ; ne tente donc pas de coller une ROM encodée en base64 dans un secret. Voir [GitHub — Secrets reference](https://docs.github.com/en/actions/reference/security/secrets).
- Les personnes disposant d'un accès en écriture au dépôt principal peuvent déclencher des workflows utilisant les secrets. Limite les collaborateurs de confiance et protège main. Une branche protégée et/ou approbation d'environnement est recommandée avant de donner accès aux secrets.
- La CI des pull requests n'accède pas aux secrets ROM et n'exécute pas de workflow privilégié sur du code non fiable.
- Le fichier clair est supprimé avant l'assemblage ; l'étape de nettoyage utilise always(). Ne retire jamais cette étape.
- Les artefacts Actions sont téléchargeables par les personnes ayant accès en lecture au dépôt. Comme ce dépôt est public, tout artefact public doit être considéré comme accessible au public.
- Avant de connecter l'importeur, examine les données générées, leurs licences et la possibilité qu'elles constituent des ressources Pokémon protégées. N'inclus pas de ROM ni de ressources dérivées dans un artefact public sans avoir résolu les droits et la stratégie de distribution.
- Pour renforcer l'isolation, configure un environnement Actions rom-build avec approbation manuelle avant l'accès aux secrets. Les paramètres de protection du dépôt doivent être vérifiés dans GitHub.
