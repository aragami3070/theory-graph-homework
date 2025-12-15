use crate::graph::core::{Graph, Index};
use eframe::egui;
use std::collections::HashMap; // твой модуль

struct MaxFlowVisualizer {
    json_input: String,
    json_output: String,
    graph: Option<Graph<String>>,
    node_positions: HashMap<Index, egui::Pos2>,
    show_graph: bool,
}

impl MaxFlowVisualizer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            json_input: String::new(),
            json_output: String::new(),
            graph: None,
            node_positions: HashMap::new(),
            show_graph: false,
        }
    }

    fn compute_layout(&self, graph: &Graph<String>) -> HashMap<Index, egui::Pos2> {
        let mut positions = HashMap::new();
        let n = graph.len().max(1) as f32;

        if n == 0.0 {
            return positions;
        }

        // Центр экрана и радиус
        let center = egui::Pos2::new(400.0, 300.0);
        let radius = 200.0;

        // Простая круговой раскладкой по индексам вершин
        for (&index, _) in graph.iter() {
            let angle = 2.0 * std::f32::consts::PI * (index.0 as f32 / n);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            positions.insert(index, egui::Pos2::new(x, y));
        }

        positions
    }

    fn draw_graph(&self, ui: &mut egui::Ui) {
        if !self.show_graph || self.graph.is_none() {
            return;
        }

        let graph = self.graph.as_ref().unwrap();
        let (_, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), ui.available_height()),
            egui::Sense::hover(),
        );

        // Рисуем рёбра
        for (from_idx, adj) in graph.iter() {
            if let Some(&from_pos) = self.node_positions.get(from_idx) {
                for edge in adj {
                    let to_idx = edge.node.number;
                    if let Some(&to_pos) = self.node_positions.get(&to_idx) {
                        // Линия ребра
                        painter.line_segment(
                            [from_pos, to_pos],
                            egui::Stroke::new(2.0, egui::Color32::LIGHT_GRAY),
                        );

                        // Стрелка
                        let dir = (to_pos - from_pos).normalized();
                        let arrow_len = 12.0;
                        let arrow_tip = to_pos;
                        let arrow_tail1 =
                            arrow_tip - dir * arrow_len + egui::Vec2::new(-dir.y, dir.x) * 6.0;
                        let arrow_tail2 =
                            arrow_tip - dir * arrow_len - egui::Vec2::new(-dir.y, dir.x) * 6.0;

                        painter.line_segment(
                            [arrow_tip, arrow_tail1],
                            egui::Stroke::new(2.0, egui::Color32::LIGHT_GRAY),
                        );
                        painter.line_segment(
                            [arrow_tip, arrow_tail2],
                            egui::Stroke::new(2.0, egui::Color32::LIGHT_GRAY),
                        );

                        // Вес ребра (capacity)
                        let mid_pos = egui::Pos2::new(
                            (from_pos.x + to_pos.x) * 0.5,
                            (from_pos.y + to_pos.y) * 0.5,
                        );
                        painter.text(
                            mid_pos,
                            egui::Align2::CENTER_CENTER,
                            edge.weight.to_string().as_str(),
                            egui::FontId::proportional(22.0),
                            egui::Color32::GREEN,
                        );
                    }
                }
            }
        }

        // Рисуем вершины
        for (&idx, node) in &graph.get_all_nodes() {
            if let Some(&pos) = self.node_positions.get(&idx) {
                let radius = 30.0;

                // Круг вершины
                painter.circle_filled(pos, radius, egui::Color32::from_rgb(40, 120, 200));
                painter.circle_stroke(pos, radius, egui::Stroke::new(2.0, egui::Color32::BLACK));

                // Текст вершины
                painter.text(
                    pos,
                    egui::Align2::CENTER_CENTER,
                    node.value.as_str(),
                    egui::FontId::proportional(18.0),
                    egui::Color32::WHITE,
                );

                // Номер вершины (маленький)
                painter.text(
                    egui::Pos2::new(pos.x, pos.y + 18.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{}", idx.0).as_str(),
                    egui::FontId::proportional(17.0),
                    egui::Color32::GREEN,
                );
            }
        }
    }
}

impl eframe::App for MaxFlowVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Max Flow Visualizer");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.show_graph {
                        if ui.button("🚀 Запуск Max Flow").clicked() {
                            // TODO: запуск алгоритма
                        }
                    }
                });
            });
        });

        egui::SidePanel::right("json_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("JSON графа");

                ui.separator();

                // Текстовое поле для JSON
                let _load_graph_label = egui::ScrollArea::vertical()
                    .max_height(ui.available_height() * 0.8)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.json_input)
                                .code_editor()
                                .desired_rows(20)
                                .desired_width(ui.available_width() * 0.95),
                        );
                        ui.add(
                            egui::TextEdit::multiline(&mut self.json_output)
                                .code_editor()
                                .desired_rows(20)
                                .desired_width(ui.available_width() * 0.95),
                        )
                    });

                ui.separator();

                if ui.button("🔄 Создать граф").clicked() {
                    match serde_json::from_str::<Graph<String>>(&self.json_input) {
                        Ok(graph) => {
                            self.graph = Some(graph);
                            self.node_positions = self.compute_layout(self.graph.as_ref().unwrap());
                            self.show_graph = true;
                        }
                        Err(e) => {
                            ui.colored_label(egui::Color32::RED, format!("Ошибка JSON: {}", e));
                        }
                    }
                }
                if ui.button("💾 Сохранить").clicked() {
                    if let Some(graph) = &self.graph {
                        let _ = graph.write_in_file(&self.json_output);
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!(
                    "Граф: {}",
                    if self.show_graph {
                        self.graph.as_ref().map_or("загружается".to_string(), |g| {
                            format!("{} вершин", g.len())
                        })
                    } else {
                        "не загружен".to_string()
                    }
                ));

                if self.show_graph {
                    self.draw_graph(ui);
                } else {
                    ui.label("Загрузите JSON графа справа");
                }
            });
        });

        // Автоматический ререндер
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

pub fn gui_interface() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]) // ширина, высота
            .with_title("Max Flow Visualizer")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Max Flow Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(MaxFlowVisualizer::new(cc)))),
    )
}
