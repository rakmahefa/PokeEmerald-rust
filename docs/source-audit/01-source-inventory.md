# Inventaire du code et des données amont

Révision du snapshot privé : a89f73dbb2d97f76700e66cb66ccaf56261c3643.

## Périmètre mesuré

- Arbre Git complet : 13 405 entrées (fichiers et répertoires, configuration incluse).
- Fichiers suivis sous decomp/pokeemerald : 11 551.
- src : 468 blobs/fichiers; include : 327; data : 3551; graphics : 5728; sound : 1303; tools : 117; asm : 24.
- data/layouts : 884 fichiers suivis.

## Extensions les plus fréquentes

| Extension | Fichiers |
|---|---:|
| .png | 4 076 |
| .pal | 2 641 |
| .bin | 1 457 |
| .inc | 794 |
| .wav | 544 |
| .json | 523 |
| .h | 514 |
| .mid | 420 |
| .c | 324 |
| .s | 131 |
| .pcm | 25 |
| .cpp | 25 |
| .sans extension | 23 |
| .gitignore | 17 |
| .txt | 9 |

## Catalogue de fichiers du sous-arbre src

Les 468 fichiers sont listés une seule fois chacun. Les groupes sont une première classification basée sur les noms/chemins pour organiser la migration, pas une garantie que chaque fichier ne touche qu'un seul domaine. Les appels et dépendances seront revus lors du port de chaque sous-système.

### Connectivité et communications — 28 fichiers

src/AgbRfu_LinkManager.c, src/cable_car.c, src/cable_club.c, src/libgcnmultiboot.s, src/libisagbprn.c, src/librfu_intr.c, src/librfu_rfu.c, src/librfu_sio32id.c, src/librfu_stwi.c, src/link.c, src/link_rfu_2.c, src/link_rfu_3.c, src/multiboot.c, src/mystery_event_menu.c, src/mystery_event_msg.c, src/mystery_event_script.c, src/mystery_gift.c, src/mystery_gift_client.c, src/mystery_gift_link.c, src/mystery_gift_menu.c, src/mystery_gift_scripts.c, src/mystery_gift_server.c, src/mystery_gift_view.c, src/union_room.c, src/union_room_battle.c, src/union_room_chat.c, src/union_room_player_avatar.c, src/wireless_communication_status_screen.c

### Couche basse, vidéo, audio et utilitaires — 32 fichiers

src/agb_flash.c, src/agb_flash_1m.c, src/agb_flash_le.c, src/agb_flash_mx.c, src/bg.c, src/blit.c, src/crt0.s, src/data.c, src/decompress.c, src/dma3_manager.c, src/gpu_regs.c, src/graphics.c, src/io_reg.c, src/m4a.c, src/m4a_1.s, src/m4a_tables.c, src/main.c, src/main_menu.c, src/malloc.c, src/math_util.c, src/mini_printf.c, src/palette.c, src/palette_util.c, src/rom_header.s, src/rom_header_gf.c, src/scanline_effect.c, src/sound.c, src/sprite.c, src/task.c, src/trig.c, src/util.c, src/window.c

### Autres systèmes de jeu et utilitaires — 57 fichiers

src/anim_mon_front_pics.c, src/apprentice.c, src/bard_music.c, src/berry.c, src/berry_blender.c, src/berry_crush.c, src/berry_fix_graphics.c, src/berry_fix_program.c, src/berry_powder.c, src/birch_pc.c, src/braille.c, src/braille_puzzles.c, src/confetti_util.c, src/digit_obj_util.c, src/dodrio_berry_picking.c, src/dynamic_placeholder_text_util.c, src/easy_chat.c, src/ereader_helpers.c, src/faraway_island.c, src/hof_pc.c, src/image_processing_effects.c, src/international_string_util.c, src/lilycove_lady.c, src/mail.c, src/mail_data.c, src/mauville_old_man.c, src/minigame_countdown.c, src/mon_markings.c, src/move_relearner.c, src/new_game.c, src/player_pc.c, src/pokeball.c, src/pokenav.c, src/pokenav_conditions.c, src/pokenav_conditions_gfx.c, src/pokenav_conditions_search_results.c, src/pokenav_list.c, src/pokenav_match_call_data.c, src/pokenav_match_call_gfx.c, src/pokenav_match_call_list.c, src/pokenav_region_map.c, src/pokenav_ribbons_list.c, src/pokenav_ribbons_summary.c, src/post_battle_event_funcs.c, src/rayquaza_scene.c, src/recorded_battle.c, src/rotating_gate.c, src/rotating_tile_puzzle.c, src/roulette.c, src/siirtc.c, src/slot_machine.c, src/string_util.c, src/strings.c, src/tv.c, src/use_pokeblock.c, src/walda_phrase.c, src/wonder_news.c

### Combats, IA et animations de combat — 62 fichiers

src/battle_ai_script_commands.c, src/battle_ai_switch_items.c, src/battle_anim.c, src/battle_anim_bug.c, src/battle_anim_dark.c, src/battle_anim_dragon.c, src/battle_anim_effects_1.c, src/battle_anim_effects_2.c, src/battle_anim_effects_3.c, src/battle_anim_electric.c, src/battle_anim_fight.c, src/battle_anim_fire.c, src/battle_anim_flying.c, src/battle_anim_ghost.c, src/battle_anim_ground.c, src/battle_anim_ice.c, src/battle_anim_mon_movement.c, src/battle_anim_mons.c, src/battle_anim_normal.c, src/battle_anim_poison.c, src/battle_anim_psychic.c, src/battle_anim_rock.c, src/battle_anim_smokescreen.c, src/battle_anim_sound_tasks.c, src/battle_anim_status_effects.c, src/battle_anim_throw.c, src/battle_anim_utility_funcs.c, src/battle_anim_water.c, src/battle_arena.c, src/battle_bg.c, src/battle_controller_link_opponent.c, src/battle_controller_link_partner.c, src/battle_controller_opponent.c, src/battle_controller_player.c, src/battle_controller_player_partner.c, src/battle_controller_recorded_opponent.c, src/battle_controller_recorded_player.c, src/battle_controller_safari.c, src/battle_controller_wally.c, src/battle_controllers.c, src/battle_dome.c, src/battle_factory.c, src/battle_factory_screen.c, src/battle_gfx_sfx_util.c, src/battle_interface.c, src/battle_intro.c, src/battle_main.c, src/battle_message.c, src/battle_palace.c, src/battle_pike.c, src/battle_pyramid.c, src/battle_pyramid_bag.c, src/battle_records.c, src/battle_script_commands.c, src/battle_setup.c, src/battle_tent.c, src/battle_tower.c, src/battle_transition.c, src/battle_transition_frontier.c, src/battle_tv.c, src/battle_util.c, src/battle_util2.c

### Écrans, menus, dialogue et présentation — 23 fichiers

src/berry_tag_screen.c, src/clear_save_data_screen.c, src/credits.c, src/ereader_screen.c, src/fonts.c, src/intro.c, src/intro_credits_graphics.c, src/list_menu.c, src/menu.c, src/menu_helpers.c, src/menu_specialized.c, src/naming_screen.c, src/option_menu.c, src/pokenav_main_menu.c, src/pokenav_menu_handler.c, src/pokenav_menu_handler_gfx.c, src/reset_rtc_screen.c, src/reshow_battle_screen.c, src/start_menu.c, src/text.c, src/text_input_strings.c, src/text_window.c, src/title_screen.c

### Carte, overworld, collisions et événements — 49 fichiers

src/bike.c, src/coord_event_weather.c, src/daycare.c, src/event_data.c, src/event_object_lock.c, src/event_object_movement.c, src/field_camera.c, src/field_control_avatar.c, src/field_door.c, src/field_effect.c, src/field_effect_helpers.c, src/field_message_box.c, src/field_player_avatar.c, src/field_poison.c, src/field_region_map.c, src/field_screen_effect.c, src/field_special_scene.c, src/field_specials.c, src/field_tasks.c, src/field_weather.c, src/field_weather_effect.c, src/fieldmap.c, src/fldeff_cut.c, src/fldeff_dig.c, src/fldeff_escalator.c, src/fldeff_flash.c, src/fldeff_misc.c, src/fldeff_rocksmash.c, src/fldeff_softboiled.c, src/fldeff_strength.c, src/fldeff_sweetscent.c, src/fldeff_teleport.c, src/heal_location.c, src/landmark.c, src/map_name_popup.c, src/metatile_behavior.c, src/mirage_tower.c, src/overworld.c, src/region_map.c, src/roamer.c, src/safari_zone.c, src/scrcmd.c, src/script.c, src/script_menu.c, src/script_movement.c, src/script_pokemon_util.c, src/tileset_anims.c, src/tilesets.c, src/wild_encounter.c

### Sauvegarde, temps et déterminisme — 11 fichiers

src/clock.c, src/load_save.c, src/play_time.c, src/random.c, src/reload_save.c, src/rtc.c, src/save.c, src/save_failed_screen.c, src/save_location.c, src/time_events.c, src/wallclock.c

### Objets, économie et activités — 30 fichiers

src/coins.c, src/contest.c, src/contest_ai.c, src/contest_effect.c, src/contest_link.c, src/contest_link_util.c, src/contest_painting.c, src/contest_util.c, src/decoration.c, src/decoration_inventory.c, src/dewford_trend.c, src/diploma.c, src/frontier_pass.c, src/frontier_util.c, src/gym_leader_rematch.c, src/hall_of_fame.c, src/item.c, src/item_icon.c, src/item_menu.c, src/item_menu_icons.c, src/item_use.c, src/lottery_corner.c, src/match_call.c, src/money.c, src/pokeblock.c, src/pokeblock_feed.c, src/record_mixing.c, src/secret_base.c, src/shop.c, src/trainer_hill.c

### Données et tables de jeu — 155 fichiers

src/data/.gitignore, src/data/bard_music/actions.h, src/data/bard_music/adjectives.h, src/data/bard_music/battle.h, src/data/bard_music/conditions.h, src/data/bard_music/endings.h, src/data/bard_music/events.h, src/data/bard_music/feelings.h, src/data/bard_music/greetings.h, src/data/bard_music/hobbies.h, src/data/bard_music/lifestyle.h, src/data/bard_music/misc.h, src/data/bard_music/moves.h, src/data/bard_music/people.h, src/data/bard_music/pokemon.h, src/data/bard_music/speech.h, src/data/bard_music/status.h, src/data/bard_music/time.h, src/data/bard_music/trainer.h, src/data/bard_music/trendysaying.h, src/data/bard_music/voices.h, src/data/battle_anim.h, src/data/battle_frontier/apprentice.h, src/data/battle_frontier/battle_frontier_exchange_corner.h, src/data/battle_frontier/battle_frontier_mons.h, src/data/battle_frontier/battle_frontier_trainer_mons.h, src/data/battle_frontier/battle_frontier_trainers.h, src/data/battle_frontier/battle_pyramid_level_50_wild_mons.h, src/data/battle_frontier/battle_pyramid_open_level_wild_mons.h, src/data/battle_frontier/battle_tent.h, src/data/battle_frontier/trainer_hill.h, src/data/battle_moves.h, src/data/contest_moves.h, src/data/contest_opponents.h, src/data/contest_text_tables.h, src/data/credits.h, src/data/decoration/description.h, src/data/decoration/header.h, src/data/decoration/icon.h, src/data/decoration/tilemaps.h, src/data/decoration/tiles.h, src/data/easy_chat/easy_chat_group_actions.h, src/data/easy_chat/easy_chat_group_adjectives.h, src/data/easy_chat/easy_chat_group_battle.h, src/data/easy_chat/easy_chat_group_conditions.h, src/data/easy_chat/easy_chat_group_endings.h, src/data/easy_chat/easy_chat_group_events.h, src/data/easy_chat/easy_chat_group_feelings.h, src/data/easy_chat/easy_chat_group_greetings.h, src/data/easy_chat/easy_chat_group_hobbies.h, src/data/easy_chat/easy_chat_group_lifestyle.h, src/data/easy_chat/easy_chat_group_misc.h, src/data/easy_chat/easy_chat_group_move_1.h, src/data/easy_chat/easy_chat_group_move_2.h, src/data/easy_chat/easy_chat_group_people.h, src/data/easy_chat/easy_chat_group_pokemon.h, src/data/easy_chat/easy_chat_group_pokemon2.h, src/data/easy_chat/easy_chat_group_speech.h, src/data/easy_chat/easy_chat_group_status.h, src/data/easy_chat/easy_chat_group_time.h, src/data/easy_chat/easy_chat_group_trainer.h, src/data/easy_chat/easy_chat_group_trendy_saying.h, src/data/easy_chat/easy_chat_group_voices.h, src/data/easy_chat/easy_chat_groups.h, src/data/easy_chat/easy_chat_words_by_letter.h, src/data/field_effects/field_effect_object_template_pointers.h, src/data/field_effects/field_effect_objects.h, src/data/graphics/battle_environment.h, src/data/graphics/berries.h, src/data/graphics/berry_fix.h, src/data/graphics/decorations.h, src/data/graphics/intro_scene.h, src/data/graphics/items.h, src/data/graphics/mail.h, src/data/graphics/pokeballs.h, src/data/graphics/pokemon.h, src/data/graphics/rayquaza_scene.h, src/data/graphics/slot_machine.h, src/data/graphics/trainers.h, src/data/heal_locations.constants.json.txt, src/data/heal_locations.json, src/data/heal_locations.json.txt, src/data/item_icon_table.h, src/data/items.h, src/data/lilycove_lady.h, src/data/object_events/base_oam.h, src/data/object_events/berry_tree_graphics_tables.h, src/data/object_events/movement_action_func_tables.h, src/data/object_events/movement_type_func_tables.h, src/data/object_events/object_event_anims.h, src/data/object_events/object_event_graphics.h, src/data/object_events/object_event_graphics_info.h, src/data/object_events/object_event_graphics_info_pointers.h, src/data/object_events/object_event_pic_tables.h, src/data/object_events/object_event_subsprites.h, src/data/party_menu.h, src/data/pointillism_points.h, src/data/pokedex_area_glow.h, src/data/pokemon/cry_ids.h, src/data/pokemon/egg_moves.h, src/data/pokemon/evolution.h, src/data/pokemon/experience_tables.h, src/data/pokemon/item_effects.h, src/data/pokemon/level_up_learnset_pointers.h, src/data/pokemon/level_up_learnsets.h, src/data/pokemon/pokedex_entries.h, src/data/pokemon/pokedex_orders.h, src/data/pokemon/pokedex_text.h, src/data/pokemon/species_info.h, src/data/pokemon/tmhm_learnsets.h, src/data/pokemon/trainer_class_lookups.h, src/data/pokemon/tutor_learnsets.h, src/data/pokemon_graphics/back_pic_coordinates.h, src/data/pokemon_graphics/back_pic_table.h, src/data/pokemon_graphics/enemy_mon_elevation.h, src/data/pokemon_graphics/footprint_table.h, src/data/pokemon_graphics/front_pic_anims.h, src/data/pokemon_graphics/front_pic_coordinates.h, src/data/pokemon_graphics/front_pic_table.h, src/data/pokemon_graphics/palette_table.h, src/data/pokemon_graphics/shiny_palette_table.h, src/data/pokemon_graphics/still_front_pic_table.h, src/data/pokemon_graphics/unknown_table.h, src/data/pokemon_graphics/unused_anims.h, src/data/region_map/city_map_entries.h, src/data/region_map/city_map_tilemaps.h, src/data/region_map/region_map_layout.h, src/data/region_map/region_map_sections.constants.json.txt, src/data/region_map/region_map_sections.json, src/data/region_map/region_map_sections.json.txt, src/data/script_menu.h, src/data/text/abilities.h, src/data/text/gift_ribbon_descriptions.h, src/data/text/item_descriptions.h, src/data/text/match_call_messages.h, src/data/text/move_descriptions.h, src/data/text/move_names.h, src/data/text/nature_names.h, src/data/text/ribbon_descriptions.h, src/data/text/species_names.h, src/data/text/trainer_class_names.h, src/data/tilesets/graphics.h, src/data/tilesets/headers.h, src/data/tilesets/metatiles.h, src/data/trade.h, src/data/trainer_graphics/back_pic_anims.h, src/data/trainer_graphics/back_pic_tables.h, src/data/trainer_graphics/front_pic_anims.h, src/data/trainer_graphics/front_pic_tables.h, src/data/trainer_parties.h, src/data/trainers.h, src/data/union_room.h, src/data/wallpapers.h, src/data/wild_encounters.json, src/data/wild_encounters.json.txt

### Pokémon, équipe, dex et progression — 21 fichiers

src/egg_hatch.c, src/evolution_graphics.c, src/evolution_scene.c, src/party_menu.c, src/pokedex.c, src/pokedex_area_region_map.c, src/pokedex_area_screen.c, src/pokedex_cry_screen.c, src/pokemon.c, src/pokemon_animation.c, src/pokemon_icon.c, src/pokemon_jump.c, src/pokemon_size_record.c, src/pokemon_storage_system.c, src/pokemon_summary_screen.c, src/starter_choose.c, src/trade.c, src/trader.c, src/trainer_card.c, src/trainer_pokemon_sprites.c, src/trainer_see.c


## Rôle des autres arbres

| Arbre | Contenu et action cible |
|---|---|
| include | Interfaces, structures, constantes et macros C. Extraire les contrats utiles, ne pas porter les macros matérielles automatiquement. |
| data/maps | Metadata de map, groupes, événements déclarés, connexions; importer/valider les identifiants. |
| data/layouts | Layouts binaires, bordures et index; décoder dimensions, IDs de métatuile, endianness et bornes. |
| data/tilesets | Tilesets primaires/secondaires, définitions et attributs de métatuiles, palettes, animations; convertir au pipeline d'assets. |
| data hors maps/layouts | Tables de jeu et définitions de domaines; importer de façon typée selon l'ordre d'implémentation. |
| graphics | Sprites, portraits, images d'interface et animations; données privées, droits à auditer. |
| sound | Musiques, bruitages, séquences et échantillons; convertir sans conserver le backend audio GBA. |
| asm, libagbsyscall, symbol maps, linker | Démarrage/ABI et appels matériels GBA; hors runtime Bevy. Conserver dans le snapshot privé si nécessaires au build de référence. |
| Makefiles, tools et scripts amont | Build du jeu original et génération de données; isoler les outils effectivement nécessaires au pipeline privé. |
| docs, manifestes et hashes | Source de provenance, pins et contraintes; conserver et faire évoluer avec le code. |

## Point de sécurité : fichiers .gba suivis

L'arbre contient 3 blobs .gba suivis :
- decomp/pokeemerald/data/mb_berry_fix.gba — 15 348 octets — blob 0afff07f5d2314e2bd35d99de518cbc114009b5e
- decomp/pokeemerald/data/mb_colosseum.gba — 163 840 octets — blob fdeb854a054ba02191f6a209ec5b649f21c4e335
- decomp/pokeemerald/data/mb_ereader.gba — 12 512 octets — blob 85057cf4e2bd657da5d632ac12d1ea24aeaa4499

Ce sont des blobs MultiBoot amont, distincts du dump Emerald principal du bootstrap. Le champ de manifeste rom_included: false ne veut pas dire « aucun fichier .gba ou contenu de jeu binaire dans le snapshot ». Exclure explicitement ces fichiers de tout paquet public.

## Limite du relevé

L'arbre entier a été recensé; les fichiers de code sont inventoriés et catégorisés. Les milliers de fichiers binaires de graphiques, sons, palettes/layouts ne sont pas tous décodés individuellement dans ce rapport. Les importeurs/validateurs et tests doivent fournir la revue au niveau de chaque format; ce document n'affirme pas qu'un contrôle visuel individuel de tous ces assets a déjà été fait.
