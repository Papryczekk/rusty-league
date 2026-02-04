use eframe::egui;
use crate::launcher;
use crate::credentials::{self, Account};
use crate::settings::{self, Settings};

#[derive(PartialEq)]
pub enum View {
    Login,
    Settings,
}

pub struct RustyLeagueApp {
    current_view: View,
    settings: Settings,
    
    username: String,
    password: String,
    region: String,
    in_game_name: String,
    custom_tag: String,

    saved_accounts: Vec<Account>,
    
    selected_account_display: String, 
    
    show_delete_confirmation: bool,

    alert_message: Option<String>,
}

impl Default for RustyLeagueApp {
    fn default() -> Self {
        let accounts = credentials::load_accounts();
        let settings = settings::load_settings();
        
        let start_view = if settings.riot_client_path.is_empty() {
             View::Settings
        } else {
             View::Login
        };

        if accounts.is_empty() {
             let _ = credentials::save_accounts(&[]);
        }

        Self {
            current_view: start_view,
            settings: settings,
            username: String::new(),
            password: String::new(),
            region: "EUNE".to_owned(),
            in_game_name: String::new(),
            custom_tag: String::new(),
            saved_accounts: accounts,
            selected_account_display: "Select an account...".to_owned(),
            show_delete_confirmation: false,
            alert_message: None,
        }
    }
}

impl RustyLeagueApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for RustyLeagueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        let mut close_alert = false;
        if let Some(msg) = &self.alert_message {
            egui::Window::new("Info")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(msg);
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        let available_width = ui.available_width();
                        let btn_width = 30.0; 
                        ui.add_space((available_width - btn_width) / 2.0);
                        if ui.button("OK").clicked() {
                            close_alert = true;
                        }
                    });
                });
        }
        if close_alert {
            self.alert_message = None;
        }

        match self.current_view {
            View::Settings => self.render_settings_view(ctx),
            View::Login => self.render_login_view(ctx),
        }
    }
}

impl RustyLeagueApp {
    fn render_settings_view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("Rusty Settings");
                ui.add_space(30.0);
                
                ui.label("Path to Riot Client Services (EXE):");
                ui.add_space(5.0);
                
                let text_edit_width = 300.0;
                let button_width = 30.0; 
                let spacing = ui.spacing().item_spacing.x;
                let total_width = text_edit_width + spacing + button_width;
                let margin = (ui.available_width() - total_width) / 2.0;

                ui.horizontal(|ui| {
                    ui.add_space(margin.max(0.0));
                    ui.add(egui::TextEdit::singleline(&mut self.settings.riot_client_path).desired_width(text_edit_width));
                    if ui.button("📂").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Executables", &["exe"])
                            .pick_file() 
                        {
                            self.settings.riot_client_path = path.to_string_lossy().into_owned();
                        }
                    }
                });

                ui.add_space(30.0);

                if ui.button("Confirm Settings").clicked() {
                    if self.settings.riot_client_path.is_empty() {
                         self.alert_message = Some("Path cannot be empty!".into());
                    } else if !std::path::Path::new(&self.settings.riot_client_path).exists() {
                         self.alert_message = Some("File does not exist!".into());
                    } else {
                        if let Err(e) = settings::save_settings(&self.settings) {
                            self.alert_message = Some(format!("Error saving settings: {}", e));
                        } else {
                            self.current_view = View::Login;
                        }
                    }
                }
            });
        });
    }

    fn render_login_view(&mut self, ctx: &egui::Context) {
        if self.show_delete_confirmation {
            egui::Window::new("Potwierdzenie")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("Czy na pewno chcesz usunąć wybrane konto?");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Tak").clicked() {
                            if let Some(index) = self.saved_accounts.iter().position(|acc| {
                                let label = format!("{}           {}", acc.full_name(), acc.region);
                                label == self.selected_account_display
                            }) {
                                self.saved_accounts.remove(index);
                                let _ = credentials::save_accounts(&self.saved_accounts);
                                self.selected_account_display = "Select an account...".to_owned();
                                self.username.clear();
                                self.password.clear();
                                self.in_game_name.clear();
                                self.custom_tag.clear();
                            }
                            self.show_delete_confirmation = false;
                        }
                        if ui.button("Nie").clicked() {
                            self.show_delete_confirmation = false;
                        }
                    });
                });
        }

        egui::Area::new(egui::Id::new("settings_area"))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
            .show(ctx, |ui| {
                 if ui.add(egui::Button::new("⚙").frame(false).min_size(egui::vec2(30.0, 30.0))).clicked() {
                     self.current_view = View::Settings;
                 }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading("Rusty League Login");
                    ui.add_space(30.0);
                });

                let label_col_width = 115.0; 
                let field_width = 220.0;     
                let grid_spacing = [15.0, 15.0];

                let estimated_grid_width = label_col_width + field_width + grid_spacing[0];
                let margin_left = (ui.available_width() - estimated_grid_width) / 2.0;
                let final_margin = if margin_left > 0.0 { margin_left } else { 10.0 };

                ui.horizontal(|ui| {
                    ui.add_space(final_margin);
                    
                    egui::Grid::new("login_form_grid")
                        .num_columns(2)
                        .min_col_width(label_col_width) 
                        .spacing(grid_spacing) 
                        .striped(false)
                        .show(ui, |ui| {
                            
                            ui.label("Username:");
                            ui.add(egui::TextEdit::singleline(&mut self.username).desired_width(field_width));
                            ui.end_row();

                            ui.label("Password:");
                            ui.add(egui::TextEdit::singleline(&mut self.password).password(true).desired_width(field_width));
                            ui.end_row();

                            ui.label("In-game name:");
                            ui.horizontal(|ui| {
                                let tag_width = 45.0;
                                let gap = ui.spacing().item_spacing.x; 
                                let hash_width_approx = 10.0;
                                
                                let safety_margin = 6.0;

                                let name_width = field_width - tag_width - (gap * 2.0) - hash_width_approx - safety_margin;

                                ui.add(egui::TextEdit::singleline(&mut self.in_game_name).desired_width(name_width));
                                ui.label("#");
                                let tag_response = ui.add(
                                    egui::TextEdit::singleline(&mut self.custom_tag)
                                        .desired_width(tag_width)
                                ).on_hover_text("Leave this area empty if u have standard tag based on region.");

                                if tag_response.changed() {
                                    if self.custom_tag.chars().count() > 5 {
                                        let truncated: String = self.custom_tag.chars().take(5).collect();
                                        self.custom_tag = truncated;
                                    }
                                }
                            });
                            ui.end_row();

                            ui.label("Region:");
                            egui::ComboBox::from_id_salt("region_combo")
                                .selected_text(&self.region)
                                .width(field_width + 8.0) 
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.region, "EUNE".to_string(), "EUNE");
                                    ui.selectable_value(&mut self.region, "EUW".to_string(), "EUW");
                                    ui.selectable_value(&mut self.region, "NA".to_string(), "NA");
                                });
                            ui.end_row();
                        });
                });

                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(final_margin + label_col_width + grid_spacing[0]);
                    
                    if ui.button("Save Account").clicked() {
                        if self.username.is_empty() {
                            self.alert_message = Some("Username cannot be empty!".to_owned());
                            return;
                        }

                        let new_account = Account::new(
                            self.username.clone(),
                            self.password.clone(),
                            self.region.clone(),
                            self.in_game_name.clone(),
                            self.custom_tag.clone(),
                        );
                        
                        self.saved_accounts.retain(|acc| acc.username != new_account.username);
                        self.saved_accounts.push(new_account.clone());
                        
                        if let Err(e) = credentials::save_accounts(&self.saved_accounts) {
                            self.alert_message = Some(format!("Error saving accounts: {}", e));
                        } else {
                            self.selected_account_display = format!("{}           {}", new_account.full_name(), new_account.region);
                            self.alert_message = Some("Account saved successfully!".to_owned());
                        }
                    }
                });

                ui.add_space(25.0);

                ui.horizontal(|ui| {
                    ui.add_space(final_margin); 
                    egui::Grid::new("account_select_grid")
                        .num_columns(2)
                        .min_col_width(label_col_width) 
                        .spacing(grid_spacing)
                        .show(ui, |ui| {
                            ui.label("Choose account:");
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_id_salt("account_combo")
                                    .selected_text(&self.selected_account_display)
                                    .width(field_width + 8.0) 
                                    .show_ui(ui, |ui| {
                                        for account in &self.saved_accounts {
                                            let label = format!("{}           {}", account.full_name(), account.region);
                                            if ui.selectable_label(self.selected_account_display == label, &label).clicked() {
                                                self.selected_account_display = label.clone();
                                                self.username = account.username.clone();
                                                self.password = account.password.clone();
                                                self.region = account.region.clone();
                                                self.in_game_name = account.in_game_name.clone();
                                                self.custom_tag = account.custom_tag.clone();
                                            }
                                        }
                                    });

                                if ui.button("📋").on_hover_text("Skopiuj nick").clicked() {
                                    if self.selected_account_display != "Select an account..." {
                                        if let Some(acc) = self.saved_accounts.iter().find(|acc| {
                                            let label = format!("{}           {}", acc.full_name(), acc.region);
                                            label == self.selected_account_display
                                        }) {
                                            ui.ctx().copy_text(acc.full_name());
                                        }
                                    }
                                }
                            });
                            ui.end_row();
                        });
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                     ui.add_space(final_margin + label_col_width + grid_spacing[0]);
                     
                     if ui.button("Delete Account").clicked() {
                        if self.selected_account_display != "Select an account..." {
                            self.show_delete_confirmation = true;
                        }
                     }
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    let login_btn_width = 250.0;
                    let kill_btn_width = 130.0; 
                    let spacing = 20.0;
                    let total_width = login_btn_width + kill_btn_width + spacing;
                    
                    let margin = (ui.available_width() - total_width) / 2.0;
                    ui.add_space(margin.max(0.0));

                    let btn_login = egui::Button::new("Login To League")
                        .min_size(egui::vec2(login_btn_width, 50.0));
                        
                    if ui.add(btn_login).clicked() {
                        if self.username.is_empty() {
                            self.alert_message = Some("Choose an account!".to_owned());
                        } else {
                            match launcher::launch_and_login(self.username.clone(), self.password.clone(), self.settings.riot_client_path.clone()) {
                                Ok(_) => {}, 
                                Err(e) => self.alert_message = Some(format!("Error: {}", e)),
                            }
                        }
                    }

                    ui.add_space(spacing);

                    if ui.button("Kill League Process").clicked() {
                        launcher::kill_league_processes();
                        self.alert_message = Some("League processes killed.".to_owned());
                    }
                });
            });
        });
    }
}
