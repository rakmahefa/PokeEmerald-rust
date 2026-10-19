use dioxus::prelude::*;
use emerald_core::GameSettings;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let defaults = GameSettings::default();
    let mut voxel_enabled = use_signal(|| defaults.voxel_overworld_enabled);
    let mut companion_enabled = use_signal(|| defaults.kanto_gear_visible);
    let mut fullscreen_enabled = use_signal(|| defaults.fullscreen);

    rsx! {
        style { {include_str!("style.css")} }
        div { class: "shell",
            aside { class: "sidebar",
                div { class: "brand",
                    div { class: "brand-mark", "PE" }
                    div {
                        div { class: "brand-name", "PokeEmerald" }
                        div { class: "brand-caption", "RUST / DESKTOP" }
                    }
                }
                nav { class: "nav",
                    div { class: "nav-item active", "◈  Général" }
                    div { class: "nav-item", "▧  Affichage" }
                    div { class: "nav-item", "♫  Audio" }
                    div { class: "nav-item", "◉  Contrôles" }
                    div { class: "nav-item", "◌  Données du jeu" }
                }
                div { class: "sidebar-foot",
                    span { class: "status-dot" }
                    span { "Runtime non connecté" }
                }
            }
            main { class: "main",
                header { class: "topbar",
                    div {
                        div { class: "eyebrow", "ESPACE DE CONFIGURATION" }
                        h1 { "Préférences" }
                        p { class: "lead", "Configurez votre expérience Hoenn." }
                    }
                    div { class: "build-chip", "EARLY ACCESS · 0.1.0" }
                }
                section { class: "hero",
                    div { class: "hero-copy",
                        div { class: "eyebrow", "VOTRE AVENTURE, VOTRE STYLE" }
                        h2 { "Un monde familier. Une nouvelle perspective." }
                        p { "Les réglages affichés sont un prototype visuel. La sauvegarde et la synchronisation avec le runtime Bevy restent à connecter." }
                    }
                    div { class: "hero-orbit",
                        div { class: "orbit orbit-outer" }
                        div { class: "orbit orbit-inner" }
                        div { class: "orbit-core", "H" }
                        div { class: "orbit-square square-one" }
                        div { class: "orbit-square square-two" }
                    }
                }
                section { class: "section-head",
                    div {
                        h2 { "Affichage & interface" }
                        p { "Choisissez comment explorer le monde." }
                    }
                    span { class: "section-number", "01 / 03" }
                }
                section { class: "settings-grid",
                    article { class: "setting-card",
                        div { class: "setting-icon green", "▦" }
                        div { class: "setting-content",
                            h3 { "Overworld voxel" }
                            p { "Activer le rendu 3D du terrain et des environnements." }
                        }
                        label { class: "toggle",
                            input {
                                r#type: "checkbox",
                                checked: voxel_enabled(),
                                onchange: move |event| voxel_enabled.set(event.checked()),
                            }
                            span { class: "slider" }
                        }
                    }
                    article { class: "setting-card",
                        div { class: "setting-icon violet", "▤" }
                        div { class: "setting-content",
                            h3 { "Compagnon Kanto Gear" }
                            p { "Afficher le panneau de navigation et de progression." }
                        }
                        label { class: "toggle",
                            input {
                                r#type: "checkbox",
                                checked: companion_enabled(),
                                onchange: move |event| companion_enabled.set(event.checked()),
                            }
                            span { class: "slider" }
                        }
                    }
                    article { class: "setting-card",
                        div { class: "setting-icon blue", "⛶" }
                        div { class: "setting-content",
                            h3 { "Plein écran" }
                            p { "Prévoir une fenêtre immersive sans bordure." }
                        }
                        label { class: "toggle",
                            input {
                                r#type: "checkbox",
                                checked: fullscreen_enabled(),
                                onchange: move |event| fullscreen_enabled.set(event.checked()),
                            }
                            span { class: "slider" }
                        }
                    }
                }
                section { class: "bottom-grid",
                    article { class: "info-card",
                        div { class: "eyebrow", "ÉTAT DU PROJET" }
                        h3 { "Les fondations sont en place." }
                        p { "Le prototype sépare le moteur Bevy, les contrats partagés et l'interface Dioxus. Les réglages ne sont pas encore persistés ni appliqués au jeu." }
                        div { class: "progress-track", div { class: "progress-value" } }
                        div { class: "progress-label",
                            span { "Architecture initiale" }
                            span { "Prototype" }
                        }
                    }
                    article { class: "info-card data-card",
                        div { class: "eyebrow", "RÉSUMÉ DES OPTIONS" }
                        div { class: "summary-row",
                            span { "Overworld voxel" }
                            span { class: if voxel_enabled() { "summary-value on" } else { "summary-value off" },
                                if voxel_enabled() { "Activé" } else { "Désactivé" }
                            }
                        }
                        div { class: "summary-row",
                            span { "Compagnon" }
                            span { class: if companion_enabled() { "summary-value on" } else { "summary-value off" },
                                if companion_enabled() { "Visible" } else { "Masqué" }
                            }
                        }
                        div { class: "summary-row",
                            span { "Plein écran" }
                            span { class: if fullscreen_enabled() { "summary-value on" } else { "summary-value off" },
                                if fullscreen_enabled() { "Activé" } else { "Désactivé" }
                            }
                        }
                    }
                }
                footer { class: "footer",
                    span { "POKEEMERALD-RUST" }
                    span { "Rust · Bevy · Dioxus" }
                    span { "Prototype local — options non persistées" }
                }
            }
        }
    }
}
