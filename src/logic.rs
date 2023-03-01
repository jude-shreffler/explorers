use ruscii::app::{App, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::terminal::{Color};
use std::fs;
use rand::Rng;

pub struct Explorers {
    map: Map,
    round: usize,
    turn: usize,
    player: usize,
    players: [Player; 4],
}

impl Explorers {
    pub fn new() -> Explorers {
        let ret: Explorers = Explorers {map: Map::new(), round: 0, turn: 0, player: 0, players: [Player::new(), Player::new(), Player::new(), Player::new()] };
        ret
    }

    pub fn start_game(&mut self) {
        let mut ruscii = App::new();
        
        ruscii.run(|app_state: &mut State, window: &mut Window| {
            for key_event in app_state.keyboard().last_key_events() {
                match key_event {
                    KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                    KeyEvent::Pressed(Key::Q) => app_state.stop(),
                    _ => (),
                }
            }
    
            self.run_logic(app_state);

            let pencil = Pencil::new(window.canvas_mut());

            self.draw_board(pencil);
        });
    }

    fn run_logic(&mut self, app_state: &mut State) {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::Left) => self.players[self.player].cursor_position.x -= 2,
                KeyEvent::Pressed(Key::Right) => self.players[self.player].cursor_position.x += 2,
                KeyEvent::Pressed(Key::Up) => self.players[self.player].cursor_position.y -= 2,
                KeyEvent::Pressed(Key::Down) => self.players[self.player].cursor_position.y += 2,

                _ => (),
            }
        }
    }

    fn draw_board(&mut self, mut pencil: Pencil) {
        for i in 0..4 {
            for j in 0..8 {
                for k in 0..8 {
                    let b_color = match self.map.quads[i].terrain_spaces[j][k] {
                        Terrain::Desert => Color::Yellow,
                        Terrain::Mountain => Color::DarkGrey,
                        Terrain::Prarie => Color::Green,
                        Terrain::Water => Color::Blue,
                        Terrain::Village => Color::Xterm(DARK_BROWN)
                    };
                    pencil.set_background(b_color);

                    let f_color = match self.map.quads[i].terrain_spaces[j][k] {
                        Terrain::Desert => Color::Magenta,
                        Terrain::Mountain => Color::White,
                        Terrain::Prarie => Color::Red,
                        Terrain::Water => Color::White,
                        Terrain::Village => Color::Xterm(LIGHT_BROWN),
                    };
                    pencil.set_foreground(f_color);

                    let position = Vec2::xy(2*j + match i {0 => 0, 1 => 16, 2 => 0, 3 => 16, _ => panic!("how")}, 2*k + match i {0 => 0, 1 => 0, 2 => 16, 3 => 16, _ => panic!("how")});
                    let item = match self.map.quads[i].item_spaces[j][k] {
                        Item::Apple => 'a',
                        Item::Carrot => 'c',
                        Item::Fish => 'f',
                        Item::Gemstone => 'g',
                        Item::Horse => 'h',
                        Item::Key => 'k',
                        Item::Scroll => 's',
                        Item::Temple => 't',
                        _ => {
                            match self.map.quads[i].terrain_spaces[j][k] {
                                Terrain::Desert => '*',
                                Terrain::Mountain => '^',
                                Terrain::Prarie => '.',
                                Terrain::Water => '~',
                                Terrain::Village => 'A'
                            }
                        },
                    };

                    let explored = match self.map.quads[i].explored_spaces[j][k][self.player] {
                        true => 'X',
                        false => item,
                    };

                    pencil.draw_char(item, position);
                    pencil.draw_char(explored, position + Vec2::xy(0,1));
                    pencil.draw_char(explored, position + Vec2::xy(1,0));
                    pencil.draw_char(item, position + Vec2::xy(1,1));
                }
            }
        }

        pencil.set_background(Color::White);
        pencil.set_foreground(Color::Red);
        pencil.draw_char('\\', self.players[self.player].cursor_position);
        pencil.draw_char('/', self.players[self.player].cursor_position + Vec2::xy(0,1));
        pencil.draw_char('/', self.players[self.player].cursor_position + Vec2::xy(1,0));
        pencil.draw_char('\\', self.players[self.player].cursor_position + Vec2::xy(1,1));
        
    }
}

pub struct Map {
    pub quads: [MapQuad; 4],
}

impl Map {
    pub fn new() -> Map {
        let mut quads = generate_map_quads();
        let positions: [Vec2; 4] = [Vec2::xy(8,8) - quads[0].get_village_pos().unwrap(),
                                    Vec2::xy(0,8) - quads[1].get_village_pos().unwrap(),
                                    Vec2::xy(8,0) - quads[2].get_village_pos().unwrap(),
                                    Vec2::xy(0,0) - quads[2].get_village_pos().unwrap()];
        let mut distances: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            let x = positions[i].x;
            let y = positions[i].y;
            distances[i] = f32::sqrt((x*x + y*y) as f32);
        }

        let mut nearest_village = 0;
        let mut to_nearest_village = distances[0];
        for i in 1..4 {
            if distances[i] < to_nearest_village {
                nearest_village = i;
                to_nearest_village = distances[i];
            }
        }

        for player in 0..4 {
            let x = quads[nearest_village].get_village_pos().unwrap().x as usize;
            let y = quads[nearest_village].get_village_pos().unwrap().y as usize;
            quads[nearest_village].explored_spaces[x][y][player] = true;
        }

        Map {quads: [quads[0], quads[1], quads[2], quads[3]]}
    }
}

pub struct Player {
    pub cursor_position: Vec2,
}

impl Player {
    pub fn new() -> Player{
        Player { cursor_position: Vec2::xy(0, 0) }
    }
}

#[derive(Clone, Copy, Default)]
pub enum Terrain {
    #[default] Prarie,
    Water,
    Desert,
    Mountain,
    Village,
}

#[derive(Clone, Copy, Default)]
pub enum Item {
    #[default] None,
    Key,
    Scroll,
    Horse,
    Apple,
    Carrot,
    Fish,
    Gemstone,
    Temple,
    Village,
}

pub const DARK_BROWN: u8 = 94;
pub const LIGHT_BROWN: u8 = 166;

#[derive(Clone, Copy)]
pub struct MapQuad { /// one quad of the whole map
    pub terrain_spaces: [[Terrain; 8]; 8], // the 8x8 grid of Terrain denoting whether a tile is Prarie, Water, Desert, or Mountain
    pub item_spaces: [[Item; 8]; 8], /// same thing but with the Item enum
    pub explored_spaces: [[[bool; 4]; 8]; 8], // same grid, but each cell contains a vector denoting whether player 0..n has walked on that cell
}

impl MapQuad {
    fn get_village_pos(& self) -> Option<Vec2> {
        for i in 0..8 {
            for j in 0..8 {
                match self.terrain_spaces[i][j] { 
                    Terrain::Village => return Some(Vec2::xy(i, j)), 
                    _ => (),
                }
            }
        }

        None
    }
}

fn generate_map_quads() -> [MapQuad; 4] { // generates 4 quads {a, b, c, d}
    
    // open the map files and fill the terrain and item vectors from these files
    let mut terrain_pool: [[[Terrain; 8]; 8]; 32] = Default::default();
    let mut item_pool: [[[Item; 8]; 8]; 32] = Default::default(); 
    
    // read both map files in as one large string each
    let terrain_string = fs::read_to_string("quads/terrain.map").expect("Error: terrain map is missing from quads");
    let item_string = fs::read_to_string("quads/item.map").expect("Error: item map is missing from quads");

    // split each at whitespace for a vector of 8 char long strings
    let terrain_vector = terrain_string.split_whitespace().collect::<Vec<&str>>();
    let item_vector = item_string.split_whitespace().collect::<Vec<&str>>();

    // check each character using the line and char number to store them into terrain_pool and item_pool
    for current_line in 0..256 {
        for current_char in 0..8 {
            terrain_pool[current_line / 8][current_line % 8][current_char] = match terrain_vector[current_line].chars().collect::<Vec<char>>()[current_char] {
                'p' => Terrain::Prarie,
                'w' => Terrain::Water,
                'd' => Terrain::Desert,
                'm' => Terrain::Mountain,
                'v' => Terrain::Village,
                _ => panic!("Error: illegal terrain type in terrain.map"),
            };

            item_pool[current_line / 8][current_line % 8][current_char] = match item_vector[current_line].chars().collect::<Vec<char>>()[current_char] {
                'k' => Item::Key,
                's' => Item::Scroll,
                'h' => Item::Horse,
                'a' => Item::Apple,
                'c' => Item::Carrot,
                'f' => Item::Fish,
                'g' => Item::Gemstone,
                't' => Item::Temple,
                'v' => Item::Village,
                'n' => Item::None,
                _ => panic!("Error: illegal item type in item.map at line {}", current_line),
            }
        }
    }

    // now that we have our pools to draw from, generate four numbers and rotations
    let mut final_terrain: [[[Terrain; 8]; 8]; 32] = Default::default();
    let mut final_items: [[[Item; 8]; 8]; 32] = Default::default();
    let mut rotations: [usize; 4] = Default::default();
    let mut indices: [usize; 4] = Default::default();

    // for the final_terrain and final_items, we need four unique numbers 0..32
    let mut unique = false;
    while !unique {
        unique = true;
        for i in 0..4 {
            indices[i] = rand::thread_rng().gen_range(0..32);
        }

        for i in 0..4 {
            for j in 0..4 {
                if i == j {continue;}
                if indices[i] == indices[j] {unique = false;}
            }
        }
    }

    // the rotations however need not be unique. rotates by npi/2
    for i in 0..4 {
        rotations[i] = rand::thread_rng().gen_range(0..4);
    }

    // assign final_xxxxxxx based on the rotation and indices
    for i in 0..4 {
        match rotations[i] {
            // no rotation
            0 => {
                final_terrain[i] = terrain_pool[indices[i]];
                final_items[i] = item_pool[indices[i]]
            },
            // 90 degrees
            1 => {
                let mut wip_terrain = terrain_pool[indices[i]];
                let mut wip_items = item_pool[indices[i]];
                // transpose the array
                for j in 0..8 {
                    for k in 0..8 {
                        wip_terrain[j][k] = terrain_pool[indices[i]][k][j];
                        wip_items[j][k] = item_pool[indices[i]][k][j];
                    }
                }
                // reverse each row
                for j in 0..8 {
                    for k in 0..8 {
                        final_terrain[i][j][k] = wip_terrain[j][7-k];
                        final_items[i][j][k] = wip_items[j][7-k];
                    }
                }
            },
            // 180 degrees
            2 => {
                let mut wip_terrain_1 = terrain_pool[indices[i]];
                let mut wip_terrain_2 = terrain_pool[indices[i]];
                let mut wip_items_1 = item_pool[indices[i]];
                let mut wip_items_2 = item_pool[indices[i]];
                // rotate 180 by doing 90 twice
                for j in 0..8 {
                    for k in 0..8 {
                        wip_terrain_1[j][k] = terrain_pool[indices[i]][k][j];
                        wip_items_1[j][k] = item_pool[indices[i]][k][j];
                    }
                }
                for j in 0..8 {
                    for k in 0..8 {
                        wip_terrain_2[j][k] = wip_terrain_1[j][7-k];
                        wip_items_2[j][k] = wip_items_1[j][7-k];
                    }
                }
                for j in 0..8 {
                    for k in 0..8 {
                        wip_terrain_1[j][k] = terrain_pool[indices[i]][k][j];
                        wip_items_1[j][k] = item_pool[indices[i]][k][j];
                    }
                }
                for j in 0..8 {
                    for k in 0..8 {
                        wip_terrain_2[j][k] = wip_terrain_1[j][7-k];
                        wip_items_2[j][k] = wip_items_1[j][7-k];
                    }
                }
                // and then transpose one more time
                for j in 0..8 {
                    for k in 0..8 {
                        final_terrain[i][j][k] = wip_terrain_2[k][j];
                        final_items[i][j][k] = wip_items_2[k][j];
                    }
                }
            },
            // -90 degrees
            3 => {
                let mut wip_terrain = terrain_pool[indices[i]];
                let mut wip_items = item_pool[indices[i]];
                // transpose the array
                for j in 0..8 {
                    for k in 0..8 {
                        wip_terrain[j][k] = terrain_pool[indices[i]][k][j];
                        wip_items[j][k] = item_pool[indices[i]][k][j];
                    }
                }
                // reverse each column
                for j in 0..8 {
                    for k in 0..8 {
                        final_terrain[i][j][k] = wip_terrain[7-j][k];
                        final_items[i][j][k] = wip_items[7-j][k];
                    }
                }
            },
            _ => panic!("How did you do this"),
        }
    }

    // create the final blank explored maps and final mapquad structs

    let mut final_explored: [[[bool; 4]; 8]; 8] = Default::default();
    for i in 0..8 {
        for j in 0..8 {
            final_explored[i][j] = [false, false, false, false];
        }
    } 

    let final_quads = [MapQuad {terrain_spaces: final_terrain[0], item_spaces: final_items[0], explored_spaces: final_explored},
        MapQuad {terrain_spaces: final_terrain[1], item_spaces: final_items[1], explored_spaces: final_explored},
        MapQuad {terrain_spaces: final_terrain[2], item_spaces: final_items[2], explored_spaces: final_explored},
        MapQuad {terrain_spaces: final_terrain[3], item_spaces: final_items[3], explored_spaces: final_explored}];
    final_quads
}