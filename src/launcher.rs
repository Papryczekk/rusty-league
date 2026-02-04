use std::process::Command;
use std::{thread, time};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use uiautomation::{UIAutomation, UIElement};

pub fn kill_league_processes() {
    let processes = [
        "RiotClientServices.exe",
        "RiotClientUx.exe",
        "LeagueClient.exe",
        "LeagueClientUx.exe",
        "LeagueCrashHandler64.exe",
        "League of Legends.exe",
    ];

    for process in processes {
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", process])
            .output();
    }
}

pub fn launch_and_login(username: String, password: String, riot_path: String) -> std::io::Result<()> {
    let mut path = riot_path;
    
    if path.is_empty() {
        path = r"E:\Riot Games\Riot Client\RiotClientServices.exe".to_string();
    }
    
    let child = Command::new(path)
        .arg("--launch-product=league_of_legends")
        .arg("--launch-patchline=live")
        .spawn()?;

    let pid = child.id();

    thread::spawn(move || {
        let found_element = wait_for_login_screen(pid); 

        if let Some(target_element) = found_element {
            if let Err(_e) = target_element.set_focus() {
            }
            
            thread::sleep(time::Duration::from_millis(500));
        } else {
            thread::sleep(time::Duration::from_secs(5));
        }

        if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
            let _ = enigo.text(&username);
            thread::sleep(time::Duration::from_millis(100));

            let _ = enigo.key(Key::Tab, Direction::Click);
            thread::sleep(time::Duration::from_millis(100));

            let _ = enigo.text(&password);
            thread::sleep(time::Duration::from_millis(100));

            let _ = enigo.key(Key::Return, Direction::Click);
        } else {
             eprintln!("Failed to initialize enigo");
        }
    });
    
    Ok(())
}

fn wait_for_login_screen(target_pid: u32) -> Option<UIElement> {
    let uia = match UIAutomation::new() {
        Ok(u) => u,
        Err(_e) => {
            return None;
        }
    };

    let start = time::Instant::now();
    let timeout = time::Duration::from_secs(60); 
    let mut last_heavy_search = time::Instant::now(); 
    
    while start.elapsed() < timeout {
        if let Ok(focused) = uia.get_focused_element() {
            if let Ok(auto_id) = focused.get_automation_id() {
                if auto_id == "username" {
                    return Some(focused);
                }
            }

            if let Ok(name) = focused.get_name() {
                if name == "USERNAME" {
                     return Some(focused);
                }
            }

            let mut is_edit = false;
            if let Ok(control_type) = focused.get_control_type() {
                if control_type == uiautomation::types::ControlType::Edit {
                    is_edit = true;
                }
            }

            if is_edit {
                if let Ok(pid) = focused.get_process_id() {
                    if (pid as u32) == target_pid {
                        return Some(focused);
                    }
                }

                if let Ok(walker) = uia.create_tree_walker() {
                    let mut current = focused.clone();
                    let mut found_riot_parent = false;

                    for _ in 0..6 {
                        if let Ok(parent) = walker.get_parent(&current) {
                            if let Ok(name) = parent.get_name() {
                                if name.contains("Riot Client") {
                                    found_riot_parent = true;
                                    break;
                                }
                            }
                            current = parent;
                        } else {
                            break;
                        }
                    }

                    if found_riot_parent {
                         return Some(focused);
                    }
                }
            }
        }

        if last_heavy_search.elapsed().as_millis() > 1000 {
            last_heavy_search = time::Instant::now();
            
            if let Ok(root) = uia.get_root_element() {
                let win_matcher = uia.create_matcher().from(root.clone()).name("Riot Client");
                
                if let Ok(windows) = win_matcher.find_all() {
                    for window in windows {
                        if let Ok(rect) = window.get_bounding_rectangle() {
                            if (rect.get_right() - rect.get_left()) < 200 { continue; }
                        }

                        let edit_matcher = uia.create_matcher()
                                .from(window.clone())
                                .control_type(uiautomation::types::ControlType::Edit)
                                .timeout(50);
                        
                        if let Ok(edit) = edit_matcher.find_first() {
                            return Some(edit);
                        }
                    }
                }
            }
        }
        
        thread::sleep(time::Duration::from_millis(50));
    }

    None
}
