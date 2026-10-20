# Paramètres, contrôles, audio et sauvegardes

## État actuel

emerald-core expose cinq valeurs de GameSettings (overworld voxel, visibilité compagnon, volume, échelle UI, fullscreen). La fenêtre Dioxus ne change que ses signaux locaux; elle ne persiste pas et ne synchronise pas Bevy. L'interface peut donc afficher une option sans effet réel. Une option n'est « fonctionnelle » que si elle modifie effectivement le comportement ou est signalée clairement comme future.

## Paramètres

- Conserver le contrat GameSettings partagé et versionner le fichier utilisateur séparément de la sauvegarde de jeu.
- Charger tôt, valider les plages/enum, traiter les champs absents et erreurs de lecture sans écraser silencieusement une bonne configuration.
- Écrire via fichier temporaire puis remplacement atomique.
- Ajouter bouton Enregistrer/Appliquer et distinguer les options directes des options nécessitant redémarrage.
- Choisir un seul transport Dioxus-Bevy (fichier+reload au démarrage au début; IPC versionné uniquement si nécessaire). Aucun accès direct Dioxus aux composants ECS.

## Contrôles

Définir des actions logiques configurables indépendantes des touches : directions, confirmer/interagir, annuler/menu, courir, vélo, mouvement de terrain, focus UI. Clavier et manette alimentent les mêmes commandes. Les overlays du compagnon ont priorité sur le monde sous-jacent afin d'éviter un double déclenchement.

## Audio

Découpler les événements (musique de map, entrée combat, fin combat, interaction, environnement) du lecteur. Le build d'assets convertit/organise les ressources requises; Bevy consomme des assets préparés. Préserver les boucles/transitions d'origine sans garder le mixer GBA ou ses accès matériels.

## Sauvegarde

Créer un DTO versionné contenant localisation, coordonnées/direction, équipe, inventaire, Pokédex, drapeaux/variables, progression, horloge et révision du bundle de données. Valider IDs, limites et version avant d'accepter l'état; prévoir migration, backup et écriture atomique. Ne pas sérialiser la hiérarchie ECS, les handles d'asset ni les entités.

## Critères

Un redémarrage doit restaurer options et partie. Le jeu doit refuser proprement un fichier tronqué, version future ou IDs invalides. Le volume doit agir sur la lecture réelle, et un contrôle ne doit jamais déclencher deux actions parce qu'un overlay est ouvert.
