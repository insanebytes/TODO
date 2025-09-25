use eframe::{egui, glow::SAMPLE_MASK};
use jiff::{Zoned, civil::DateTime};

use crate::task::Task;
use std::{fs, io::ErrorKind};

#[derive(Default)]
pub struct ToDoApp {
    pub tasks: Vec<Task>,
    pub errorMessage: String,
}

impl ToDoApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self::default()
    }

    fn load_tasks(&mut self) {
        match fs::read_to_string("task.json") {
            Ok(content) => self.tasks = serde_json::from_str(&content).ok().unwrap_or_default(),
            Err(error) => {
                if error.kind() != ErrorKind::NotFound {
                    self.errorMessage = "Error loading task".to_string();
                }
                self.tasks = Vec::new();
            }
        }
    }

    fn save_tasks(&mut self) -> Result<(), ()> {
        match serde_json::to_string_pretty(&self.tasks) {
            Ok(content) => {
                let json_content = content;

                return match fs::write("task.json", json_content) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        self.errorMessage = "Error saving tasks".to_string();
                        Err(())
                    }
                };
            }
            Err(..) => {
                self.errorMessage = "error serializing tasks".to_string();
                Err(())
            }
        }
    }

    fn add_task(&mut self, text: String, date: Option<String>) {
        let id: u32 = (self.tasks.len() as u32) + 1;

        let date_work: DateTime;

        match date {
            Some(d) => date_work = DateTime::strptime("%F %T", d).unwrap(),
            None => date_work = Zoned::now().datetime(),
        }

        let tarea: Task = Task {
            id: id,
            text: text,
            date: date_work,
            done: false,
        };

        self.tasks.push(tarea);
        self.save_tasks();
    }

    fn mark_task_done(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.done = true;
        }
    }
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        //interfaz de usuario
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Add Task").clicked() {
                self.add_task("Test task".to_string(), None);
            }

            ui.heading("Lista de Tareas");

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for tarea in &self.tasks {
                    // Creamos un rectángulo interactivo
                    let fecha: String = tarea.date.strftime("%d/%m/%Y %H:%M:%S").to_string();

                    let response = ui.add(
                        egui::Button::new(format!("📌 {} - {}", tarea.text, fecha))
                            .fill(egui::Color32::TRANSPARENT) // Fondo transparente por defecto
                            .frame(false), // Sin marco por defecto
                    );

                    if response.hovered() {
                        let rect = response.rect;
                        ui.painter()
                            .rect_filled(rect, 5.0, egui::Color32::from_rgb(60, 60, 120));

                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("📌 {} - {}", tarea.text, fecha),
                            egui::FontId::proportional(16.0),
                            egui::Color32::WHITE,
                        );
                    }
                }
            })
        });
    }
}
