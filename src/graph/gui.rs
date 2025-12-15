use crate::graph::core::{Graph, Index};
use eframe::egui;
use std::collections::HashMap;
use std::collections::VecDeque;

struct MaxFlowVisualizer {
    json_input: String,
    json_output: String,
    graph: Option<Graph<String>>,
    node_positions: HashMap<Index, egui::Pos2>,
    show_graph: bool,
    s_input: String,
    t_input: String,

    // Состояние алгоритма
    capacity: Option<HashMap<(Index, Index), u32>>,
    flow: Option<HashMap<(Index, Index), i32>>,
    current_path: Option<Vec<Index>>,
    max_flow: u32,
    step: usize,
    s: Option<Index>,
    t: Option<Index>,
}

impl MaxFlowVisualizer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            json_input: String::new(),
            json_output: String::new(),
            s_input: String::new(),
            t_input: String::new(),
            graph: None,
            node_positions: HashMap::new(),
            show_graph: false,
            capacity: None,
            flow: None,
            current_path: None,
            max_flow: 0,
            step: 0,
            s: None,
            t: None,
        }
    }

    fn compute_layout(&self, graph: &Graph<String>) -> HashMap<Index, egui::Pos2> {
        let mut positions = HashMap::new();
        let n = graph.len().max(1) as f32;

        if n == 0.0 {
            return positions;
        }

        let center = egui::Pos2::new(400.0, 300.0);
        let radius = 200.0;

        for (&index, _) in graph.iter() {
            let angle = 2.0 * std::f32::consts::PI * (index.0 as f32 / n);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            positions.insert(index, egui::Pos2::new(x, y));
        }
        positions
    }

    fn build_capacity_and_flow(&mut self, graph: &Graph<String>) {
        let mut capacity: HashMap<(Index, Index), u32> = HashMap::new();
        let mut flow: HashMap<(Index, Index), i32> = HashMap::new();

        for (&from, adj) in graph.iter() {
            for edge in adj {
                *capacity.entry((from, edge.node.number)).or_insert(0) += edge.weight;
                flow.entry((from, edge.node.number)).or_insert(0);
                flow.entry((edge.node.number, from)).or_insert(0);
            }
        }

        self.capacity = Some(capacity);
        self.flow = Some(flow);
        self.max_flow = 0;
        self.step = 0;
        self.current_path = None;
    }

    fn residual(&self, from: Index, to: Index) -> u32 {
        if let (Some(capacity), Some(flow)) = (&self.capacity, &self.flow) {
            let c = *capacity.get(&(from, to)).unwrap_or(&0);
            let f = *flow.get(&(from, to)).unwrap_or(&0);
            if c == 0 && f > 0 {
                f as u32
            } else {
                c.saturating_sub(f.max(0) as u32)
            }
        } else {
            0
        }
    }

    fn next_step(&mut self) {
        if self.graph.is_none() || self.capacity.is_none() || self.s.is_none() || self.t.is_none() {
            return;
        }

        let graph = self.graph.as_ref().unwrap();
        let capacity = self.capacity.as_ref().unwrap();
        let flow = self.flow.as_mut().unwrap();
        let s = self.s.unwrap();
        let t = self.t.unwrap();

        // BFS поиск пути
        let mut parent: HashMap<Index, Option<Index>> = HashMap::new();
        let mut visited: HashMap<Index, bool> = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(s);
        parent.insert(s, None);
        visited.insert(s, true);

        let mut path_found = false;
        'bfs: while let Some(from_ind) = queue.pop_front() {
            if let Some(adj) = graph.get_adjacency(&from_ind) {
                for edge in adj {
                    let to_ind = edge.node.number;
                    if !visited.get(&to_ind).copied().unwrap_or(false) && {
                        let c = *capacity.get(&(from_ind, to_ind)).unwrap_or(&0);
                        let f = *flow.get(&(from_ind, to_ind)).unwrap_or(&0);
                        let r = if c == 0 && f > 0 {
                            f as u32
                        } else {
                            c.saturating_sub(f.max(0) as u32)
                        };
                        r > 0
                    } {
                        parent.insert(to_ind, Some(from_ind));
                        visited.insert(to_ind, true);
                        if to_ind == t {
                            path_found = true;
                            break 'bfs;
                        }
                        queue.push_back(to_ind);
                    }
                }
            }
        }

        if !path_found || !parent.contains_key(&t) {
            self.current_path = None;
            return;
        }

        // Восстанавливаем путь
        let mut path = vec![t];
        let mut v = t;
        while let Some(Some(u)) = parent.get(&v) {
            path.push(*u);
            v = *u;
        }
        path.reverse();

        // Восстанавливаем путь
        self.current_path = Some(path.clone());

        // Бутылочное горлышко
        let mut path_flow: i32 = i32::MAX;
        for window in path.windows(2) {
            let from = window[0];
            let to = window[1];

            let c = *capacity.get(&(from, to)).unwrap_or(&0);
            let f = *flow.get(&(from, to)).unwrap_or(&0);
            let r = if c == 0 && f > 0 {
                f as u32
            } else {
                c.saturating_sub(f.max(0) as u32)
            };

            path_flow = path_flow.min(r as i32);
        }

        // Обновляем потоки
        for window in path.windows(2) {
            *flow.entry((window[0], window[1])).or_insert(0) += path_flow;
            *flow.entry((window[1], window[0])).or_insert(0) -= path_flow;
        }

        self.max_flow += path_flow as u32;
        self.step += 1;
    }

    fn draw_graph(&self, ui: &mut egui::Ui) {
        if !self.show_graph || self.graph.is_none() {
            return;
        }

        let graph = self.graph.as_ref().unwrap();
        let (_response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), ui.available_height()),
            egui::Sense::hover(),
        );

        // Рисуем рёбра
        for (from_idx, adj) in graph.iter() {
            if let Some(&from_pos) = self.node_positions.get(from_idx) {
                for edge in adj {
                    let to_idx = edge.node.number;
                    if let Some(&to_pos) = self.node_positions.get(&to_idx) {
                        let current_flow = self.residual(*from_idx, to_idx);
                        let is_path_edge = self.current_path.as_ref().map_or(false, |path| {
                            path.windows(2).any(|w| w == [*from_idx, to_idx])
                        });

                        // Цвет и толщина в зависимости от потока и пути
                        let color = if is_path_edge {
                            egui::Color32::GREEN
                        } else if current_flow > 0 {
                            egui::Color32::from_rgb(100, 200, 100)
                        } else {
                            egui::Color32::LIGHT_GRAY
                        };

                        let stroke_width = if is_path_edge { 4.0 } else { 2.0 };

                        // Линия ребра
                        painter.line_segment(
                            [from_pos, to_pos],
                            egui::Stroke::new(stroke_width, color),
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
                            egui::Stroke::new(stroke_width, color),
                        );
                        painter.line_segment(
                            [arrow_tip, arrow_tail2],
                            egui::Stroke::new(stroke_width, color),
                        );

                        // Текст: flow/capacity
                        let mid_pos = egui::Pos2::new(
                            (from_pos.x + to_pos.x) * 0.5,
                            (from_pos.y + to_pos.y) * 0.5,
                        );

                        let flow_str = if let Some(flow_map) = &self.flow {
                            format!(
                                "{}/{}",
                                flow_map.get(&(*from_idx, to_idx)).unwrap_or(&0),
                                edge.weight
                            )
                        } else {
                            edge.weight.to_string()
                        };

                        painter.text(
                            mid_pos,
                            egui::Align2::CENTER_CENTER,
                            flow_str.as_str(),
                            egui::FontId::proportional(18.0),
                            egui::Color32::RED,
                        );
                    }
                }
            }
        }

        // Вершины (подсветка s и t)
        for (&idx, node) in &graph.get_all_nodes() {
            if let Some(&pos) = self.node_positions.get(&idx) {
                let radius = 30.0;
                let color = if Some(idx) == self.s {
                    egui::Color32::from_rgb(0, 255, 0) // зелёный для источника
                } else if Some(idx) == self.t {
                    egui::Color32::from_rgb(255, 0, 0) // красный для стока
                } else {
                    egui::Color32::from_rgb(40, 120, 200)
                };

                painter.circle_filled(pos, radius, color);
                painter.circle_stroke(pos, radius, egui::Stroke::new(2.0, egui::Color32::BLACK));

                painter.text(
                    pos,
                    egui::Align2::CENTER_CENTER,
                    node.value.as_str(),
                    egui::FontId::proportional(18.0),
                    egui::Color32::WHITE,
                );

                painter.text(
                    egui::Pos2::new(pos.x, pos.y + 18.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{}", idx.0).as_str(),
                    egui::FontId::proportional(17.0),
                    egui::Color32::BLACK,
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

                ui.vertical(|ui| {
                    ui.label(format!("Шаг: {}", self.step));
                    ui.label(format!("Поток: {}", self.max_flow));

                    // Финальный статус
                    if self.capacity.is_some() && self.current_path.is_none() {
                        ui.colored_label(
                            egui::Color32::from_rgb(0, 255, 0),
                            format!("✅ МАКСИМАЛЬНЫЙ ПОТОК: {}", self.max_flow),
                        );
                    } else if self.capacity.is_some() {
                        ui.label("🔍 Идёт поиск...");
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.show_graph && self.capacity.is_some() {
                        if ui.button("▶️ Следующий шаг").clicked() {
                            self.next_step();
                        }
                        if ui.button("🔄 Сброс").clicked() {
                            self.capacity = None;
                            self.flow = None;
                            self.current_path = None;
                            self.max_flow = 0;
                            self.step = 0;
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

                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() * 0.6)
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
                                .desired_rows(5)
                                .desired_width(ui.available_width() * 0.95),
                        );
                    });

                ui.separator();

                egui::Grid::new("controls")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        if ui.button("🔄 Создать граф").clicked() {
                            match serde_json::from_str::<Graph<String>>(&self.json_input) {
                                Ok(graph) => {
                                    self.graph = Some(graph.clone());
                                    self.node_positions = self.compute_layout(&graph);
                                    self.show_graph = true;
                                    ui.label("Граф загружен!");
                                }
                                Err(e) => {
                                    ui.colored_label(egui::Color32::RED, format!("Ошибка: {}", e));
                                }
                            }
                        }
                        if ui.button("💾 Сохранить").clicked() {
                            if let Some(graph) = &self.graph {
                                let _ = graph.write_in_file(&self.json_output);
                            }
                        };

                        ui.end_row();

                        if self.show_graph && self.graph.is_some() {
                            ui.label("Исток (s):");
                            ui.add(egui::TextEdit::singleline(&mut self.s_input));
                            if ui.button("Set s").clicked() {
                                if let Ok(val) = self.s_input.parse::<u32>() {
                                    self.s = Some(Index(val));
                                }
                            }
                            ui.end_row();

                            ui.label("Сток (t):");
                            ui.add(egui::TextEdit::singleline(&mut self.t_input));
                            if ui.button("Set t").clicked() {
                                if let Ok(val) = self.t_input.parse::<u32>() {
                                    self.t = Some(Index(val));
                                }
                            }
                            ui.end_row();

                            if ui.button("🚀 Запустить алгоритм").clicked() {
                                if let Some(graph) = &self.graph {
                                    self.build_capacity_and_flow(&graph.clone());
                                }
                            }
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if self.show_graph {
                    self.draw_graph(ui);
                } else {
                    ui.heading("Загрузите JSON графа справа");
                }
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

pub fn gui_interface() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
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
