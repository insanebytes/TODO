use eframe::egui::{self, RichText};
use jiff::{Zoned, civil::DateTime};

use crate::task::Task;
use std::{fs, io::ErrorKind};

#[derive(Default)]
pub struct ToDoApp {
    pub tasks: Vec<Task>,
    pub error_message: String,
}

impl ToDoApp {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.load_tasks();
        app
    }

    fn load_tasks(&mut self) {
        match fs::read_to_string("task.json") {
            Ok(content) => self.tasks = serde_json::from_str(&content).ok().unwrap_or_default(),
            Err(error) => {
                if error.kind() != ErrorKind::NotFound {
                    self.error_message = "Error loading task".to_string();
                }
                self.tasks = Vec::new();
            }
        }
    }

    fn save_tasks(&mut self) -> Result<(), &str> {
        match serde_json::to_string_pretty(&self.tasks) {
            Ok(content) => {
                let json_content = content;

                match fs::write("task.json", json_content) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        self.error_message = format!("{} {}", "Error saving tasks", error);
                        Err(&self.error_message)
                    }
                }
            }
            Err(error) => {
                self.error_message = format!("{} {}", "Error serializing tasks", error);
                Err(&self.error_message)
            }
        }
    }

    fn add_task<T: Into<String>>(&mut self, name: T, description: T, date: Option<DateTime>) {
        let id: u32 = (self.tasks.len() as u32) + 1;

        let date_work: DateTime = match date {
            Some(d) => d,
            None => Zoned::now().datetime(),
        };

        let tarea: Task = Task {
            id,
            name: name.into(),
            description: description.into(),
            date: date_work,
            done: false,
        };

        self.tasks.push(tarea);
        if let Err(error) = self.save_tasks() {
            self.error_message = format!("{} {}", "Error serializing tasks", error);
        }
    }
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        //interfaz de usuario
        eframe::egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("Lista de Tareas");
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let mut guardar: bool = false;
                    let available_width = ui.available_width();
                    let width = available_width * 0.85;

                    for tarea in self.tasks.iter_mut() {
                        ui.add_space(20.0);
                        if let Some(()) = render_task(ui, tarea, width) {
                            guardar = true;
                        }
                    }

                    if guardar {
                        let _ = self.save_tasks();
                    }

                    egui::Frame::NONE.show(ui, |ui| {
                        ui.set_width(width);

                        let desired_size = egui::vec2(
                            ui.available_width(),
                            ui.text_style_height(&egui::TextStyle::Body)
                                + ui.spacing().item_spacing.y * 2.0,
                        );
                        let (rect, response) =
                            ui.allocate_exact_size(desired_size, egui::Sense::click());
                        let painter = ui.painter_at(rect);

                        // Posiciones para dibujar el "+"
                        let plus_pos =
                            rect.left_center() + egui::vec2(ui.spacing().icon_width / 2.0, 0.0);

                        // Texto "Añadir tarea"
                        let text_pos = plus_pos
                            + egui::vec2(
                                ui.spacing().item_spacing.x + ui.spacing().icon_spacing,
                                -ui.text_style_height(&egui::TextStyle::Body) * 0.5,
                            );

                        // Si está hovered -> círculo alrededor del "+"
                        if response.hovered() {
                            painter.circle_filled(
                                plus_pos,
                                ui.spacing().icon_width / 2.0,
                                egui::Color32::LIGHT_RED,
                            );
                            // Dibujar "+"
                            painter.text(
                                plus_pos,
                                egui::Align2::CENTER_CENTER,
                                "+",
                                egui::TextStyle::Body.resolve(ui.style()),
                                egui::Color32::WHITE,
                            );

                            // Dibujar texto
                            painter.text(
                                text_pos,
                                egui::Align2::LEFT_TOP,
                                "Añadir tarea",
                                egui::TextStyle::Body.resolve(ui.style()),
                                egui::Color32::RED,
                            );
                        } else {
                            // Dibujar "+"
                            painter.text(
                                plus_pos,
                                egui::Align2::CENTER_CENTER,
                                "+",
                                egui::TextStyle::Body.resolve(ui.style()),
                                egui::Color32::RED,
                            );

                            // Dibujar texto
                            painter.text(
                                text_pos,
                                egui::Align2::LEFT_TOP,
                                "Añadir tarea",
                                egui::TextStyle::Body.resolve(ui.style()),
                                ui.style().visuals.text_color(),
                            );
                        }

                        // Acción al hacer click en cualquier parte de la fila
                        if response.clicked() {
                            self.add_task(
                                "Task",
                                "Task description",
                                Some(Zoned::now().datetime()),
                            );
                        }
                    });
                });
            })
        });
    }
}

fn render_task(ui: &mut egui::Ui, tarea: &mut Task, width: f32) -> Option<()> {
    let fecha: String = tarea.date.strftime("%d/%m/%Y %H:%M:%S").to_string();

    let mut opcion: Option<()> = None;
    egui::Frame::NONE.show(ui, |ui| {
        ui.set_width(width);

        //nombre y check de hecho
        ui.horizontal(|ui| {
            if ui.radio(tarea.done, "").clicked() {
                tarea.done = !tarea.done;
                opcion = Some(())
            };
            if tarea.done {
                ui.label(RichText::new(&tarea.name).strong().strikethrough().size(16.0));
            } else {
                ui.label(RichText::new(&tarea.name).strong().size(16.0));
            }
        });

        //descripcion
        // ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
        ui.horizontal(|ui| {
            // let radio_width = ui.spacing().interact_size.x - ui.spacing().icon_width;
            let radio_width = ui.spacing().interact_size.x - ui.spacing().icon_width;
            ui.add_space(radio_width);
            ui.label(RichText::new(&tarea.description).size(12.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(&fecha).size(12.0));
            });
        });
        ui.add_space(5.0);
        ui.separator()
    });

    opcion
}
