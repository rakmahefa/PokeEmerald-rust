# Roadmap d'implémentation Rust + Bevy

La roadmap fixe des portes de validation, pas des estimations calendaires. État courant : scaffold technique, pas de gameplay.

## Phase 0 — Baseline, audit et sécurité

Geler les commits source et les hashes; inventaire par domaines; règles explicites du paquet public/privé. Ajouter une allowlist de release refusant par défaut ROM, .sav, .gba, paquets d'assets et fichiers inconnus. **Gate:** audit de paquet échoue sur fixtures interdites, le workflow public reste sans accès privé.

## Phase 1 — Modèle de domaine et architecture Bevy

Définir IDs, coordonnées/direction, session, commandes, états, RNG et erreurs dans emerald-core sans dépendance Bevy. Découper emerald-game en plugins app/data/map/player/collision/events/encounters/battle/UI/audio/input. Scène factice clairement en mode debug. **Gate:** tests de domaine sans fenêtre, cargo fmt/check/test verts.

## Phase 2 — Bundle de données versionné

Étendre au-delà de MapCatalog: map metadata → layouts/bin borders → tilesets/metatuiles/attributs/palettes → événements/warps → tables d'encounters → données Pokémon/items/moves requises. Génération déterministe depuis le snapshot privé avec manifest, provenance, hashes et validation de référence. **Gate:** aucune référence invalide ou donnée silencieusement omise; test de déterminisme et fixtures malformed.

## Phase 3 — Première carte réelle

Décoder la carte de départ explicitement choisie et construire un renderer top-down diagnostique utilisant vraie grille et attributs; charger les tilesets/bordures. La projection voxel utilise les mêmes cellules. **Gate:** screenshot/source/hash, dimensions/cellules connues conformes, plus de terrain synthétique en mode jeu normal.

## Phase 4 — Joueur, mouvement, collision, warps

Actions logiques clavier/manette; pas de grille, interpolation visuelle, collision métatuiles/objets/élévation; transitions par connexions/warps. **Gate:** avatar contrôlable, pas refusés testés, transitions cohérentes et coordonnées déterministes.

## Phase 5 — Interactions/scripts

Flags, variables, dialogues, choix, locks, coordinate events, warps et interpréteur d'opcodes. Les commandes async attendent leur achèvement sans bloquer la frame; opcodes inconnus sont rapportés. **Gate:** une interaction source se déroule de bout en bout avec tests sur conditions.

## Phase 6 — Encounters puis OWE visible

Importer les tables de rencontre canonique; implémenter méthode/terrain/heure/RNG/Repel; introduire entités OWE visibles et connecter contact au vrai BattleRequest. **Gate:** quota/timers/terrain/Surf/Repel/cleanup testés headless et mon engagé identique à l'entité visible.

## Phase 7 — Combat, Pokémon et objets

Machine d'état de combat, tour/actions/effets/statuts/PP/capture/XP/évolution, équipe et inventaire, avec RNG injecté et tests de parité. **Gate:** un encounter passe de la carte au combat, puis retour/issue de façon cohérente.

## Phase 8 — Save/progression/horloge

Format save versionné, flags/vars, dex, inventory, party, progression et horloge; migration/backups/écriture atomique. **Gate:** roundtrip après transition/combat et fichiers malformés rejetés sans écrasement.

## Phase 9 — Companion Kanto Gear

Pages Bevy UI basées sur des snapshots lecture seule et commandes validées : overview, map/Explorer, party, bag, dex, progress, notes, trainer card. Puis options discovery, day-care/calendar/field tools. **Gate:** disparition du mock « Not connected » en mode normal; pages indisponibles clairement signalées; chaque action testée.

## Phase 10 — Paramètres, input et audio

Persist settings, clavier/manette reconfigurable, volume et audio événementiel, échelle UI, plein écran, transport Dioxus/Bevy documenté. **Gate:** options appliquées et présentes après redémarrage; tests focus UI, audio et config invalide.

## Phase 11 — Couverture du monde, performance, paquet final

Porter les maps/systèmes étendus; créer matrice du niveau de support; profiler/cacher/batcher; tests install/release et manifest/sha. **Gate:** build Linux reproductible, paquet propre testé, couverture publique et licences documentées. Aucune affirmation « remake complet » sans matrice explicite.

## Definition of Done de toute phase

Fonctionnalité intégrée au runtime, tests succès/erreur, fixtures source et provenance, audit de paquet, documentation mise à jour, CI verte et artefact généré depuis le commit réel. Si une étape échoue, la phase suivante ne peut pas être qualifiée de terminée.
