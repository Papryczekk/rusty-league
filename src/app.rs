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
    show_password: bool,

    alert_message: Option<String>,
    dragged_account_idx: Option<usize>,
    drag_offset: Option<egui::Vec2>,
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
            show_password: false,
            alert_message: None,
            dragged_account_idx: None,
            drag_offset: None,
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
        ctx.set_visuals(egui::Visuals::dark());
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
                let field_width = 250.0;     
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
                            let password_response = ui.add(egui::TextEdit::singleline(&mut self.password).password(!self.show_password).desired_width(field_width));
                            
                            let eye_icon = if self.show_password { "🚫" } else { "👁" };
                            let password_rect = password_response.rect;

                            ui.scope_builder(egui::UiBuilder::new().max_rect(password_rect), |ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(8.0); 
                                    if ui.add(egui::Button::new(eye_icon).frame(false).small()).on_hover_text("Show/Hide password").clicked() {
                                        self.show_password = !self.show_password;
                                    }
                                });
                            });
                            ui.end_row();

                            ui.label("In-game name:");
                            ui.horizontal(|ui| {
                                let tag_width = 45.0;
                                let gap = ui.spacing().item_spacing.x; 
                                let hash_width_approx = 10.0;
                                
                                let safety_margin = 6.5;

                                let name_width = field_width - tag_width - (gap * 2.0) - hash_width_approx - safety_margin;

                                let name_response = ui.add(egui::TextEdit::singleline(&mut self.in_game_name).desired_width(name_width));
                                if name_response.changed() {
                                    if self.in_game_name.contains('#') {
                                        if let Some((name, tag)) = self.in_game_name.clone().split_once('#') {
                                            self.in_game_name = name.to_string();
                                            self.custom_tag = tag.to_string();
                                        }
                                    }
                                }

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
                                let (display_name, display_region) = {
                                    let parts: Vec<&str> = self.selected_account_display.split("           ").collect();
                                    let name = parts.first().map(|s| s.to_string()).unwrap_or_else(|| self.selected_account_display.clone());
                                    let region = parts.get(1).map(|s| s.to_string());
                                    (name, region)
                                };

                                let current_drag_idx = self.dragged_account_idx;
                                
                                let combo_response = egui::ComboBox::from_id_salt("account_combo")
                                    .selected_text(&display_name)
                                    .width(field_width + 8.0)
                                    .height(250.0)
                                    .show_ui(ui, |ui| {
                                        let mut swap_request = None;
                                        let mut new_drag_idx = None;
                                        let mut selected_account_idx = None;

                                        for (idx, account) in self.saved_accounts.iter().enumerate() {
                                            let label = format!("{}           {}", account.full_name(), account.region);
                                            let is_selected = self.selected_account_display == label;

                                            let font_id = egui::TextStyle::Body.resolve(ui.style());
                                            let row_height = ui.text_style_height(&egui::TextStyle::Body) + 4.0;
                                            
                                            let (rect, response) = ui.allocate_exact_size(
                                                egui::vec2(ui.available_width(), row_height), 
                                                egui::Sense::click_and_drag()
                                            );
                                            
                                            if response.drag_started() {
                                                new_drag_idx = Some(idx);
                                                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                                                     self.drag_offset = Some(pointer_pos - rect.min);
                                                }
                                            }

                                            if let Some(dragged_idx) = current_drag_idx {
                                                if dragged_idx != idx {
                                                    if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                                                        if rect.contains(pointer_pos) {
                                                            let center_y = rect.center().y;
                                                            let should_swap = if dragged_idx < idx {
                                                                pointer_pos.y > center_y
                                                            } else {
                                                                pointer_pos.y < center_y
                                                            };

                                                            if should_swap {
                                                                swap_request = Some((dragged_idx, idx));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            if response.clicked() && !response.dragged() {
                                                selected_account_idx = Some(idx);
                                            }

                                            let is_being_dragged = current_drag_idx == Some(idx);

                                            if is_being_dragged {
                                                let corner_radius = ui.visuals().widgets.inactive.corner_radius;
                                                ui.painter().rect_filled(
                                                    rect, 
                                                    corner_radius, 
                                                    ui.visuals().widgets.inactive.bg_fill.linear_multiply(0.5)
                                                );
                                                
                                                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                                                    let layer_id = egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("drag_ghost"));
                                                    let painter = ui.ctx().layer_painter(layer_id);
                                                    
                                                    let ghost_pos = if let Some(offset) = self.drag_offset {
                                                        pointer_pos - offset
                                                    } else {
                                                        pointer_pos + egui::vec2(10.0, 10.0)
                                                    };

                                                    let ghost_rect = egui::Rect::from_min_size(ghost_pos, rect.size());
                                                    
                                                    let ghost_bg = ui.visuals().widgets.active.bg_fill;
                                                    let ghost_stroke = ui.visuals().widgets.active.bg_stroke;
                                                    let ghost_text_color = ui.visuals().widgets.active.text_color();
                                                    let ghost_radius = ui.visuals().widgets.active.corner_radius;
                                                    
                                                    let shadow_rect = ghost_rect.translate(egui::vec2(2.0, 2.0));
                                                    painter.rect_filled(shadow_rect, ghost_radius, egui::Color32::from_black_alpha(100));

                                                    painter.rect_filled(ghost_rect, ghost_radius, ghost_bg);
                                                    painter.rect_stroke(ghost_rect, ghost_radius, ghost_stroke, egui::StrokeKind::Outside);
                                                    
                                                    let padding = 4.0;
                                                    painter.text(
                                                        ghost_rect.left_center() + egui::vec2(padding, 0.0),
                                                        egui::Align2::LEFT_CENTER,
                                                        account.full_name(),
                                                        font_id.clone(),
                                                        ghost_text_color,
                                                    );
                                                    
                                                    painter.text(
                                                        ghost_rect.right_center() - egui::vec2(padding, 0.0),
                                                        egui::Align2::RIGHT_CENTER,
                                                        &account.region,
                                                        font_id.clone(),
                                                        ghost_text_color,
                                                    );
                                                }
                                                ui.ctx().request_repaint();
                                            } else {
                                                let mut visuals = ui.style().interact_selectable(&response, is_selected);
                                                
                                                if  current_drag_idx.is_some() {
                                                     visuals.bg_fill = visuals.bg_fill.linear_multiply(0.8);
                                                }
                                                
                                                if let Some(d_idx) = current_drag_idx {
                                                    if d_idx != idx && response.hovered() {
                                                         ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                                                    }
                                                }

                                                if is_selected || response.hovered() || response.has_focus() {
                                                    ui.painter().rect(
                                                        rect,
                                                        visuals.corner_radius,
                                                        visuals.bg_fill,
                                                        visuals.bg_stroke,
                                                        egui::StrokeKind::Outside,
                                                    );
                                                }
                                                
                                                let text_color = visuals.text_color();
                                                let padding = 4.0;
                                                
                                                ui.painter().text(
                                                    rect.left_center() + egui::vec2(padding, 0.0),
                                                    egui::Align2::LEFT_CENTER,
                                                    account.full_name(),
                                                    font_id.clone(),
                                                    text_color,
                                                );
                                                
                                                ui.painter().text(
                                                    rect.right_center() - egui::vec2(padding, 0.0),
                                                    egui::Align2::RIGHT_CENTER,
                                                    &account.region,
                                                    font_id,
                                                    text_color,
                                                );
                                            }
                                        }
                                        (swap_request, new_drag_idx, selected_account_idx)
                                    });

                                if let Some((swap, drag_start, selection)) = combo_response.inner {
                                    if let Some(idx) = drag_start {
                                        self.dragged_account_idx = Some(idx);
                                    }

                                    if let Some((from, to)) = swap {
                                        self.saved_accounts.swap(from, to);
                                        self.dragged_account_idx = Some(to);
                                    }

                                    if let Some(idx) = selection {
                                        if let Some(account) = self.saved_accounts.get(idx) {
                                            self.selected_account_display = format!("{}           {}", account.full_name(), account.region);
                                            self.username = account.username.clone();
                                            self.password = account.password.clone();
                                            self.region = account.region.clone();
                                            self.in_game_name = account.in_game_name.clone();
                                            self.custom_tag = account.custom_tag.clone();
                                        }
                                    }
                                }
                                
                                if ui.input(|i| i.pointer.any_released()) {
                                    if self.dragged_account_idx.is_some() {
                                        self.dragged_account_idx = None;
                                        self.drag_offset = None;
                                        if let Err(e) = credentials::save_accounts(&self.saved_accounts) {
                                            self.alert_message = Some(format!("Error saving accounts: {}", e));
                                        }
                                    }
                                }

                                if let Some(region) = &display_region {
                                    let rect = combo_response.response.rect;
                                    let font_id = egui::TextStyle::Body.resolve(ui.style());
                                    let visuals = ui.style().interact_selectable(&combo_response.response, false); 
                                    let text_color = visuals.text_color();
                                    
                                    ui.painter().text(
                                        rect.right_center() - egui::vec2(25.0, 0.0),
                                        egui::Align2::RIGHT_CENTER,
                                        region,
                                        font_id,
                                        text_color,
                                    );
                                }

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
                    let kill_btn_width = 125.5; 
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
