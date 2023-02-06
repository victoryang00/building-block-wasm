use std::collections::HashMap;

use cell::Cellule;
use gloo::timers::callback::Interval;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

mod cell;
type Point = (i8, i8);
fn is_point_in_polygon(t1: i8, t2: i8, t3: i8, t4: i8, p: &Point) -> bool {
    p.0 <= t1 && p.0 >= t2 && p.1 >= t3 && p.1 <= t4
}

pub enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule(usize),
    Tick,
}

pub struct App {
    active: bool,
    chess_map: HashMap<Vec<(i8, i8, i8, i8)>, (String, String)>,
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    _interval: Interval,
}

impl App {
    pub fn random_mutate(&mut self) {
        for cellule in self.cellules.iter_mut() {
            if rand::thread_rng().gen() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].is_alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        // get the current location of the cellule
        let row = idx / self.cellules_width;
        let col = idx % self.cellules_width;
        let (cellule_status, href) = {
            let mut res = "cellue-live".to_owned();
            let mut res1 = "".to_owned();
            if cellule.is_alive() {
                for (key, value) in self.chess_map.iter() {
                    for k in key {
                        if is_point_in_polygon(k.0, k.1, k.2, k.3, &(row as i8, col as i8)) {
                            // log::info!("{},{}",row, col);
                            res = value.0.clone();
                            res1 = value.1.clone();
                        }
                    }
                }
                // res = "cellue-live".to_owned();
            } else {
                for (key, value) in self.chess_map.iter() {
                    for k in key {
                        if is_point_in_polygon(k.0, k.1, k.2, k.3, &(row as i8, col as i8)) {
                            // log::info!("{},{}",row, col);
                            res1 = value.1.clone();
                        }
                    }
                }
                res = "cellue-dead".to_owned();
            }
            (res, res1)
        };
        // log::info!("{}",href);

        html! {
            <a href={href}>
            <div key={idx} class={classes!("game-cellule", cellule_status)}
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
            </a>
        }
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        let (cellules_width, cellules_height) = (53, 29);
        let mut chess_map = HashMap::new();
        chess_map.insert(
            vec![(4, 0, 0, 13), (7, 4, 0, 17)],
            (
                "cellule-live-math".to_string(),
                "https://dblp.org/db/conf/stoc/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(4, 0, 14, 30), (8, 4, 18, 30)],
            (
                "cellule-live-ai".to_string(),
                "https://dblp.org/db/conf/aaai/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(3, 0, 31, 53), (7, 4, 31, 42)],
            (
                "cellule-live-distributed".to_string(),
                "https://dblp.org/db/conf/sosp/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(13, 8, 37, 47), (8, 4, 42, 47)],
            (
                "cellule-live-ds".to_string(),
                "https://dblp.org/db/conf/soda/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(20, 8, 0, 4), (20, 17, 4, 32)],
            (
                "cellule-live-os".to_string(),
                "https://dblp.org/db/conf/osdi/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(12, 8, 5, 36)],
            (
                "cellule-live-app".to_string(),
                "https://dblp.org/db/conf/nips/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(16, 13, 5, 15)],
            (
                "cellule-live-framework".to_string(),
                "https://dblp.org/db/conf/asplos/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(16, 13, 16, 25)],
            (
                "cellule-live-data".to_string(),
                "https://dblp.org/db/conf/vldb/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(16, 13, 26, 36), (20, 17, 33, 36)],
            (
                "cellule-live-compiler".to_string(),
                "https://dblp.org/db/conf/pldi/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(40, 4, 48, 53)],
            (
                "cellule-live-pc".to_string(),
                "https://dblp.org/db/conf/ppopp/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(20, 14, 37, 47)],
            (
                "cellule-live-network".to_string(),
                "https://dblp.org/db/conf/nsdi/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(40, 21, 0, 10), (24, 21, 11, 26)],
            (
                "cellule-live-ca".to_string(),
                "https://dblp.org/db/conf/micro/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(40, 21, 40, 47), (24, 21, 27, 39)],
            (
                "cellule-live-reconfigurable".to_string(),
                "https://dblp.org/db/conf/fpga/index.html".to_string(),
            ),
        );
        chess_map.insert(
            vec![(40, 25, 11, 39)],
            (
                "cellule-live-circuit".to_string(),
                "https://dblp.org/db/conf/cav/index.html".to_string(),
            ),
        );
        // chess_map.insert(vec![(4,8,0,17)], "math".to_string());
        Self {
            active: false,
            chess_map,
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if !self.active {
            self.random_mutate();
            log::info!("Random");
            self.active = true;
            log::info!("Start");

            return true;
        }
        match msg {
            Msg::Step => {
                self.step();
                true
            }
            Msg::ToggleCellule(idx) => {
                self.cellules[idx].toggle();
                true
            }
            Msg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows =
            self.cellules
                .chunks(self.cellules_width)
                .enumerate()
                .map(|(y, cellules)| {
                    let idx_offset = y * self.cellules_width;

                    let cells = cellules
                        .iter()
                        .enumerate()
                        .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link()));
                    html! {
                        <div key={y} class="game-row">
                            { for cells }
                        </div>
                    }
                });
        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
