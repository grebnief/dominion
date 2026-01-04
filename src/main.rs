use eframe::{egui, Frame};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ethers::signers::{LocalWallet, Signer};

const _ALLOWED_TLDS: [&str; 10] = [
    "p2p", "aut", "sov", "qnt", "dao", "dan", "ant", "btc", "eth", "sol"
];

const _NAME_WORDS: [&str; 50] = [
    "veil", "shadow", "ghost", "cipher", "echo", "river", "star", 
"mountain", "blue", "green",
    "quantum", "sovereign", "free", "pure", "eternal", "phoenix", "wolf", 
"lion", "eagle", "hawk",
    "forge", "hammer", "anvil", "shield", "sword", "light", "dark", 
"void", "nova", "apex",
    "zen", "flow", "wave", "storm", "fire", "ice", "thunder", "silent", 
"wild", "true",
    "bold", "brave", "wise", "calm", "strong", "deep", "high", "far", 
"near", "core"
];

const _PREMIUM_DOMAINS: [&str; 20] = [
    "free.p2p", "love.p2p", "game.p2p", "ai.p2p", "chat.p2p", "music.p2p", 
"art.p2p", "code.p2p",
    "shop.p2p", "news.p2p", "blog.p2p", "work.p2p", "play.p2p", 
"live.p2p", "home.p2p", "safe.p2p",
    "true.p2p", "pure.p2p", "bold.p2p", "wild.p2p"
];

#[allow(dead_code)]
#[derive(Clone)]
struct DominionEntry {
    owner_pks: Vec<Vec<u8>>,
    four_word: String,
    records: HashMap<String, String>,
    endorsements: Vec<(Vec<u8>, String)>,
    auction_bid: Option<u64>,
    auction_bidder: Option<Vec<u8>>,
    auction_end_time: Option<u64>,
}

type SimulatedDht = Arc<Mutex<HashMap<Vec<u8>, DominionEntry>>>;

#[derive(PartialEq, Eq)]
enum Tab {
    Dashboard,
    Browser,
    MyDomains,
    Marketplace,
    Settings,
}

struct DominionApp {
    dht: SimulatedDht,
    wallet: Option<LocalWallet>,
    current_tab: Tab,
    _command_input: String,
    output: String,
    lookup_domain: String,
    lookup_result: String,
    quick_register_domain: String,
    quick_register_fourword: String,
}

impl DominionApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            dht: Arc::new(Mutex::new(HashMap::new())),
            wallet: None,
            current_tab: Tab::Dashboard,
            _command_input: String::new(),
            output: "Welcome to Dominion — Sovereign Identity Primitive 
for Autonomi\n.p2p is the future.\nUse tabs above to 
navigate.".to_string(),
            lookup_domain: String::new(),
            lookup_result: String::new(),
            quick_register_domain: String::new(),
            quick_register_fourword: String::new(),
        }
    }

    #[allow(dead_code)]
    fn hash_domain(&self, domain: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(domain.as_bytes());
        hasher.finalize().to_vec()
    }

    #[allow(dead_code)]
    fn is_valid_tld(&self, domain: &str) -> bool {
        if domain.split('.').count() != 2 {
            return false;
        }
        let tld = domain.split('.').nth(1).unwrap_or("");
        _ALLOWED_TLDS.contains(&tld)
    }

    #[allow(dead_code)]
    fn verify_ownership(&self, domain: &str, signer_pk: &[u8]) -> bool {
        let key = self.hash_domain(domain);
        if let Some(entry) = self.dht.lock().unwrap().get(&key) {
            entry.owner_pks.iter().any(|pk| pk.as_slice() == signer_pk)
        } else {
            false
        }
    }
}

impl eframe::App for DominionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Dark Cyberpunk Theme
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(5, 5, 15);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(15, 15, 
35);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(40, 20, 
80);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 30, 
140);
        visuals.selection.bg_fill = egui::Color32::from_rgb(120, 40, 200);
        visuals.selection.stroke = egui::Stroke::new(2.0, 
egui::Color32::from_rgb(200, 100, 255));
        visuals.hyperlink_color = egui::Color32::from_rgb(140, 100, 255);
        visuals.window_stroke = egui::Stroke::new(2.0, 
egui::Color32::from_rgb(60, 30, 100));
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(12.0, 10.0);
        style.spacing.window_margin = egui::Margin::same(20.0);
        ctx.set_style(style);

        // Top Panel: Title + Tabs
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("DOMINION")
                    .strong()
                    .size(40.0)
                    .color(egui::Color32::from_rgb(180, 100, 255)));
                ui.label(egui::RichText::new("Sovereign Identity Primitive 
for Autonomi")
                    .italics()
                    .size(18.0)
                    .color(egui::Color32::from_rgb(160, 160, 240)));
                ui.add_space(15.0);
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Dashboard, 
"Dashboard");
                ui.selectable_value(&mut self.current_tab, Tab::Browser, 
"Browser");
                ui.selectable_value(&mut self.current_tab, Tab::MyDomains, 
"My Domains");
                ui.selectable_value(&mut self.current_tab, 
Tab::Marketplace, "Marketplace");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, 
"Settings");
            });
            ui.separator();
        });

        // Central Content
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Dashboard => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.label(egui::RichText::new("Quick Register")
                            .size(28.0)
                            .strong()
                            .color(egui::Color32::from_rgb(140, 80, 
255)));

                        ui.add_space(30.0);
                        ui.horizontal(|ui| {
                            ui.label("Domain:");
                            ui.text_edit_singleline(&mut 
self.quick_register_domain);
                            ui.label(".p2p");
                        });

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            ui.label("Four-word:");
                            ui.text_edit_singleline(&mut 
self.quick_register_fourword);
                        });

                        ui.add_space(40.0);
                        if ui.button(egui::RichText::new("Register 
Domain").size(20.0)).clicked() {
                            self.output = "Quick register logic coming 
soon — use CLI commands for now.".to_string();
                        }

                        ui.add_space(50.0);
                        ui.label(egui::RichText::new("Name Suggestions")
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::from_rgb(100, 180, 
255)));
                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui.button("Quantum Names").clicked() {}
                            if ui.button("Freedom Names").clicked() {}
                            if ui.button("Nature Names").clicked() {}
                        });
                    });
                }

                Tab::Browser => {
                    ui.vertical(|ui| {
                        ui.add_space(40.0);
                        ui.horizontal(|ui| {
                            ui.label("Lookup domain:");
                            ui.text_edit_singleline(&mut 
self.lookup_domain);
                            if ui.button("Lookup").clicked() && 
!self.lookup_domain.is_empty() {
                                self.lookup_result = format!(
                                    "Looking up {}...\n\nFull rich lookup 
with PQC badges coming soon.",
                                    self.lookup_domain
                                );
                            }
                        });

                        ui.separator();

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut 
self.lookup_result.as_str())
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(f32::INFINITY)
                                    .frame(false),
                            );
                        });
                    });
                }

                Tab::MyDomains => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.label(egui::RichText::new("Your Sovereign 
Domains")
                            .size(32.0)
                            .strong()
                            .color(egui::Color32::from_rgb(180, 120, 
255)));
                        ui.add_space(30.0);
                        ui.label("Connect a wallet in Settings to view and 
manage your domains.");
                    });
                }

                Tab::Marketplace => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.label(egui::RichText::new("Premium Domain 
Auctions")
                            .size(32.0)
                            .strong()
                            .color(egui::Color32::from_rgb(140, 80, 
255)));
                        ui.add_space(30.0);
                        ui.label("free.p2p • ai.p2p • love.p2p • code.p2p 
• ...");
                        ui.add_space(30.0);
                        ui.label("Connect wallet to bid — 7-day auctions 
with AUTONOMI staking.");
                    });
                }

                Tab::Settings => {
                    ui.vertical(|ui| {
                        ui.add_space(40.0);
                        ui.label(egui::RichText::new("Settings")
                            .size(28.0)
                            .strong()
                            .color(egui::Color32::from_rgb(100, 200, 
255)));

                        ui.group(|ui| {
                            ui.label("Wallet Connection");
                            if let Some(wallet) = &self.wallet {
                                ui.label(format!("Connected: {}", 
wallet.address()));
                                if ui.button("Disconnect 
Wallet").clicked() {
                                    self.wallet = None;
                                    self.output = "Wallet 
disconnected.".to_string();
                                }
                            } else {
                                ui.label("No wallet connected");
                                let mut pk_input = String::new();
                                ui.horizontal(|ui| {
                                    ui.label("Private Key (hex):");
                                    ui.text_edit_singleline(&mut 
pk_input);
                                    if ui.button("Connect").clicked() && 
!pk_input.is_empty() {
                                        self.output = format!("Simulated 
connection: 0x{}...", &pk_input[..8]);
                                    }
                                });
                            }
                        });

                        ui.add_space(40.0);
                        ui.label("Theme: Dark Cyberpunk — Sovereign 
Edition");
                        ui.label("Backend: Simulated DHT");
                        ui.label("Version: 0.3 — January 2026");
                    });
                }
            }
        });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                
ui.label(egui::RichText::new("Status:").strong().color(egui::Color32::from_rgb(100, 
200, 255)));
                if self.wallet.is_some() {
                    ui.label(egui::RichText::new("Wallet 
Connected").color(egui::Color32::LIGHT_GREEN));
                } else {
                    ui.label(egui::RichText::new("No 
Wallet").color(egui::Color32::LIGHT_RED));
                }
                ui.separator();
                ui.label(format!("Domains Registered: {}", 
self.dht.lock().unwrap().len()));
                
ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Dominion v0.3 • January 
2026")
                        .small()
                        .italics()
                        .color(egui::Color32::from_rgb(140, 140, 220)));
                });
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_resizable(true)
            .with_title("Dominion — Sovereign Identity Primitive"),
        ..Default::default()
    };

    eframe::run_native(
        "Dominion",
        options,
        Box::new(|_cc| Ok(Box::new(DominionApp::new(_cc)))),
    ).unwrap();
}

