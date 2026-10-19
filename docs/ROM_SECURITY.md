# Dépôt privé d'assets et bootstrap pret/pokeemerald

Cette procédure sépare le dépôt de code public, la ROM personnelle et le snapshot source utilisé pour les futurs builds. Elle ne remplace pas la vérification des droits applicables au dump et aux données dérivées.

## 1. Pourquoi le workflow doit vivre dans le dépôt privé

Le dépôt principal `rakmahefa/PokeEmerald-rust` est public. Le jeton `GITHUB_TOKEN` d'un workflow public est scoped à ce dépôt : il ne peut pas automatiquement lire `rakmahefa/PokeEmerald-rust-private-assets`. La connexion GitHub de l'assistant et les permissions du runner Actions sont deux mécanismes distincts. La documentation GitHub confirme qu'un token courant est scoped à son dépôt et qu'un autre dépôt privé exige une permission explicite : [actions/checkout — Checkout multiple repos (private)](https://github.com/actions/checkout#checkout-multiple-repos-private).

Pour éviter un PAT cross-repo, les workflows qui utilisent les assets doivent s'exécuter **dans le dépôt privé lui-même**. Le `GITHUB_TOKEN` de cette exécution pourra lire le dépôt courant, ses releases et ses assets. Le code de `PokeEmerald-rust` peut être cloné sans secret car il est public.

## 2. Ce que fait réellement pret/pokeemerald

[pret/pokeemerald](https://github.com/pret/pokeemerald) est déjà un dépôt de décompilation. Il contient le code source et les ressources versionnées qui reconstruisent une ROM. Le workflow ne peut pas produire magiquement du code C inédit à partir de la ROM : il valide la version du dump, compile le checkout amont épinglé, exécute `make compare`, puis crée une archive des fichiers source suivis par Git. La ROM générée pour la comparaison est supprimée et n'entre pas dans l'archive.

Le README upstream indique que la version reconstruite attendue a le SHA-1 `f3ae088181bf583e55daf962a92bb46f4f1d07b7` : [pret/pokeemerald README](https://github.com/pret/pokeemerald/blob/master/README.md). Le build source est décrit dans le [guide d'installation upstream](https://github.com/pret/pokeemerald/blob/master/INSTALL.md).

## 3. Créer le dépôt privé (étape manuelle nécessaire)

Le connecteur GitHub disponible dans cette session permet de créer des branches, commits, fichiers et pull requests, mais il n'expose pas d'opération de création de dépôt. Le dépôt `rakmahefa/PokeEmerald-rust-private-assets` n'existe pas encore, donc je ne peux pas y pousser les templates jusqu'à ce qu'il soit créé.

1. Ouvre [GitHub — New repository](https://github.com/new).
2. Choisis le nom exact `PokeEmerald-rust-private-assets`, propriétaire `rakmahefa`, visibilité **Private**, et initialise de préférence avec `README`.
3. Ajoute les fichiers depuis [`docs/private-assets-template`](private-assets-template/) à la racine du nouveau dépôt (notamment `.github/workflows/prepare-pret-decomp.yml`, `.github/workflows/build-linux.yml`, `upstream.lock` et `.gitignore`).
4. Crée le dossier `rom/` et ajoute seulement `rom/emerald.gba.gpg`, chiffré localement. Ne téléverse pas `emerald.gba` en clair dans Git.
5. Dans **Settings → Secrets and variables → Actions → Secrets**, ajoute `ROM_GPG_PASSPHRASE`, le passphrase utilisé pour le chiffrement. Aucun PAT n'est nécessaire pour les workflows du dépôt privé.

L'intégration à l'assistant dépend aussi du fait que ton application/installation GitHub ait accès à ce nouveau dépôt privé ; créer un dépôt ne change pas automatiquement le périmètre de connexion.

## 4. Chiffrer le dump localement

Sur ta machine, à l'emplacement où se trouve ta ROM personnelle `emerald.gba` :

~~~bash
gpg --symmetric --cipher-algo AES256 --output emerald.gba.gpg emerald.gba
~~~

Choisis un passphrase fort, unique, conservé dans un gestionnaire de mots de passe. Vérifie que le chiffrement/déchiffrement fonctionne avant de supprimer les copies de travail. Ajoute uniquement `emerald.gba.gpg` au dépôt privé, jamais la ROM claire ni le passphrase.

Le secret du passphrase est nécessaire **au bootstrap uniquement**. Une fois le snapshot source privé produit, les builds de développement quotidiens n'ont pas besoin de relire le fichier ROM. Une réimportation future nécessitera à nouveau le dump chiffré et son secret.

## 5. Workflow de préparation du snapshot

Dans le dépôt privé, ouvre **Actions → Prepare private pret source snapshot → Run workflow** et lance le workflow depuis `main`.

Il :
1. déchiffre la ROM dans `$RUNNER_TEMP` ;
2. vérifie le SHA-1 connu, puis supprime le fichier clair ;
3. checkout les commits amont épinglés de `pret/pokeemerald` et `pret/agbcc` ;
4. exécute la reconstruction et `make compare` ;
5. archive les fichiers suivis du commit source, sans le résultat ROM de build ;
6. produit manifeste et checksums SHA-256 ;
7. publie le snapshot comme asset d'une **release privée** et artefact Actions.

La release privée est le stockage durable du snapshot ; l'artefact Actions est pratique pour télécharger le résultat du run, mais reste soumis à la politique de rétention configurée.

## 6. Développement sans ROM

Le workflow privé `Build Linux from private snapshot` prend le code public de `PokeEmerald-rust`, récupère le snapshot de référence dans la release privée et compile les crates Rust. Il n'utilise pas de ROM ou de secret passphrase. Ce mécanisme est préparatoire : le runtime Bevy actuel ne consomme pas encore les données du snapshot. L'importeur et le raccordement aux systèmes de jeu restent à coder.

Les builds Rust/Bevy/Dioxus peuvent donc être fréquents sans ROM. Si une release finale choisit d'extraire des ressources depuis la ROM de chaque joueur, cette opération doit se faire localement chez le joueur et rester distincte du packaging public de l'application.

## 7. Règles de sécurité

- Ne place jamais la ROM en clair dans un dépôt, une issue, un artefact ou un log.
- Ne publie pas d'artefact de ROM, même si le build a réussi `make compare`.
- Ne déclenche pas le workflow depuis une branche non fiable ou une PR.
- Le bootstrap est manuel et limité à `main`.
- Le snapshot privé peut contenir du code et des données ayant des licences amont : ne le republie pas publiquement sans examen.
- Le futur artefact public doit contenir seulement ce qui est destiné à être distribué. Le build actuel n'inclut ni la ROM ni le snapshot source.
