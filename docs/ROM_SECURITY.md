# Bootstrap pret/pokeemerald et dépôt d'assets privé

Le dépôt privé [`rakmahefa/PokeEmerald-rust-private-assets`](https://github.com/rakmahefa/PokeEmerald-rust-private-assets) existe et contient les workflows de bootstrap et de build. Le dump a été téléversé temporairement à sa racine ; il faut le traiter uniquement depuis le workflow du dépôt privé.

## Pourquoi les workflows s'exécutent dans le dépôt privé

Le dépôt principal `rakmahefa/PokeEmerald-rust` est public. Le `GITHUB_TOKEN` d'un workflow public est scoped au dépôt courant : il ne donne pas automatiquement accès au dépôt privé. La connexion GitHub de l'assistant et les permissions d'un runner Actions sont aussi deux mécanismes distincts.

Les deux workflows d'assets s'exécutent dans `PokeEmerald-rust-private-assets`. Ils utilisent le `GITHUB_TOKEN` limité à ce dépôt pour publier des releases et des artefacts privés. Il n'y a pas de PAT inter-dépôts ni de secret de passphrase dans le modèle actuel.

## Ce que fait le bootstrap

[pret/pokeemerald](https://github.com/pret/pokeemerald) est déjà un dépôt de décompilation. Le workflow ne transforme pas la ROM en nouveaux fichiers C ; il vérifie que le dump correspond à la révision acceptée, construit une version épinglée de la source puis exécute `make compare`. Il crée ensuite une archive des fichiers suivis par Git et un manifeste des commits.

Pins initiaux :

- `pret/pokeemerald`: `76463dac15cad36aca5e2b3f6366abf9e53f814f`
- `pret/agbcc`: `da598c1d918402c42c0c0d7128ba14567f3175e9`
- SHA-1 attendu, Pokémon Emerald USA/Europe v1.0 : `f3ae088181bf583e55daf962a92bb46f4f1d07b7`

Références : [README pret/pokeemerald](https://github.com/pret/pokeemerald/blob/master/README.md) et [guide d'installation](https://github.com/pret/pokeemerald/blob/master/INSTALL.md).

## Lancer le workflow de préparation

1. Ouvre [Actions dans le dépôt privé](https://github.com/rakmahefa/PokeEmerald-rust-private-assets/actions).
2. Sélectionne **Prepare private pret source snapshot**, puis **Run workflow**.
3. Sélectionne la branche `main` et lance le workflow. Ne pousse pas d'autres commits pendant son exécution : il réécrit volontairement l'historique Git de `main` pour retirer le chemin de ROM téléversé.
4. Si le SHA-1 est accepté et la compilation passe, le workflow déplace le source suivi vers `decomp/pokeemerald/`, y place `decomp/MANIFEST.json` et `decomp/SOURCE_SHA256.txt`, puis publie une release privée `pret-source-76463dac15cad36aca5e2b3f6366abf9e53f814f` ainsi qu'un artefact Actions.
5. Le fichier ROM est retiré du dernier arbre et des références Git accessibles normalement. Le workflow n'inclut jamais ce fichier dans l'archive, le manifeste, les checksums ou la release.

Comportement en cas d'erreur : un SHA-1 incorrect provoque l'échec et la purge du fichier ; si le SHA-1 est correct mais que le build échoue avant la création du snapshot, le dump est conservé pour pouvoir corriger et relancer. Si la réécriture d'historique ou le push échoue, arrête et vérifie l'état du dépôt avant toute nouvelle tentative.

### À propos de la suppression Git

La réécriture d'historique retire le dump des commits/branches ordinaires de Git, mais ne garantit pas la purge immédiate de tous les objets inaccessibles et caches internes. Si une suppression physique immédiate est nécessaire, contacte GitHub Support après la réussite du bootstrap. Le dépôt est privé, limite ses collaborateurs et évite tout partage de son historique avant la purge.

## Builds de développement sans ROM

Après un bootstrap réussi :

1. Dans le même dépôt privé, lance **Build Linux from private snapshot** depuis Actions.
2. Laisse `source_ref` sur `agent/project-foundation` tant que la pull request de fondation n'est pas fusionnée ; ensuite sélectionne `main`.
3. Le workflow vérifie le manifeste ROM-free, compile et teste le workspace Rust, puis publie une archive Linux x86_64, son SHA-256 et une release privée taggée `rust-build-N`.

Ces sorties sont privées. Le snapshot pret ne fait pas encore partie du binaire car l'importeur et son intégration dans le runtime Bevy restent à implémenter.

## Règles de sécurité et licences

- Ne téléverse jamais le dump dans le dépôt public `PokeEmerald-rust`.
- Ne téléverse jamais le dump dans un artefact, une release ou un log.
- Ne lance pas le workflow de bootstrap sur une branche non fiable ou depuis une PR.
- Le source et les données amont ont leurs propres périmètres de licence ; le fait qu'une archive soit privée ne donne aucun droit de distribution supplémentaire.
- Une future release publique doit rester distincte de ces assets privés et ne contenir que les composants dont la distribution a été évaluée.
\n## Catalogue de cartes\n\nLe workflow de build privé génère un catalogue JSON de métadonnées depuis le snapshot local. Il ne copie pas le checkout source, la ROM, les layouts binaires, les sprites, les tilesets ou les scripts dans l'archive. Le catalogue reste dans les artefacts privés tant que les licences et la politique de redistribution n'ont pas été décidées. Le code public de l'importeur ne contient aucune donnée amont.\n