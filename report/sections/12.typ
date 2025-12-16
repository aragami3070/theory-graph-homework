= Творческое задание.
== Цель
Написать визуализацию на нахождение максимального потока.
== Краткое описание алгоритма
Это немного модифицированный алгоритм Эдмондса-Карпа из 11 задания с
сохранением состояния между сложением потоков, чтобы пошагово алгоритм
отрисовывать.
== Код (фрагменты кода)
#set text(size: 12pt)
```rust
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
                    if **from_idx > *to_idx {
                        continue;
                    }
                    if let Some(&to_pos) = self.node_positions.get(&to_idx) {
                        let current_flow = self.residual(*from_idx, to_idx);
                        let is_path_edge = self
                            .current_path
                            .as_ref()
                            .is_some_and(|path| path.windows(2).any(|w| w == [*from_idx, to_idx]));
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
                        let arrow_len = 22.0;
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
```

