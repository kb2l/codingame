use std::{
    collections::{HashMap, HashSet},
    io,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Entity {
    id: i32,
    _type: i32,
    x: i32,
    y: i32,
    shield_life: i32,
    is_controlled: i32,
    health: i32,
    vx: i32,
    vy: i32,
    near_base: i32,
    threat_for: i32,
}
#[derive(Debug, Clone, Copy, Default)]
struct InitParams {
    base_x: i32,
    base_y: i32,
    ebase_x: i32,
    ebase_y: i32,
    heroes_per_player: i32,
}

struct Game {
    init_params: InitParams,
    entities: Vec<Entity>,
    players_health: [i32; 2],
    players_mana: [i32; 2],
    targets_map: HashMap<i32, Vec<Entity>>,
    witcher_reached_pos: bool,
}

struct Utils {}
impl Utils {
    pub fn distance(p1: (i32, i32), p2: (i32, i32)) -> f64 {
        let dx = p1.0 as f64 - p2.0 as f64;
        let dy = p1.1 as f64 - p2.1 as f64;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
impl Game {
    pub fn new(init_params: InitParams) -> Self {
        Game {
            init_params,
            entities: Vec::new(),
            players_health: [0, 0],
            players_mana: [0, 0],
            targets_map: HashMap::new(),
            witcher_reached_pos: false,
        }
    }
    pub fn split(&self) -> [Vec<Entity>; 3] {
        let mut me: Vec<Entity> = Vec::new();
        let mut enemy: Vec<Entity> = Vec::new();
        let mut monsters: Vec<Entity> = Vec::new();
        self.entities.iter().for_each(|x| match x._type {
            0 => monsters.push(*x),
            1 => me.push(*x),
            2 => enemy.push(*x),
            _ => {
                panic!("this should never happen")
            }
        });
        [me, enemy, monsters]
    }

    pub fn DistanceBwtEntities(&self, entity1: &Entity, entity2: &Entity) -> f64 {
        Utils::distance((entity1.x, entity1.y), (entity2.x, entity2.y))
    }

    pub fn GetDistanceToMonsters(
        &self,
        entity: &Entity,
        other: &Vec<Entity>,
    ) -> Vec<(f64, Entity)> {
        let mut ret = Vec::new();

        other.iter().for_each(|monster| {
            let dist_to_base = Utils::distance(
                (self.init_params.base_x, self.init_params.base_y),
                (monster.x, monster.y),
            );
            if monster.near_base == 1 && dist_to_base < 5000. {
                let distance = Utils::distance((entity.x, entity.y), (monster.x, monster.y));
                ret.push((distance, *monster));
            }
        });
        ret.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        ret
    }

    pub fn GetDistanceToEnemies(&self, entity: &Entity, other: &Vec<Entity>) -> Vec<(f64, Entity)> {
        let mut ret = Vec::new();

        other.iter().for_each(|enemy| {
            let dist_to_base = Utils::distance(
                (self.init_params.base_x, self.init_params.base_y),
                (enemy.x, enemy.y),
            );
            if dist_to_base < 5000. {
                let distance = Utils::distance((entity.x, entity.y), (enemy.x, enemy.y));
                ret.push((distance, *enemy));
            }
        });
        ret.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        ret
    }

    // pub fn IsTargetAlive(&self, target: &Entity, group: &Vec<Entity>) -> Option<&Entity> {

    // }

    pub fn UpdateTargets(&mut self, myheres: &Vec<Entity>, monsters: &Vec<Entity>) {
        myheres.iter().enumerate().for_each(|(i, hero)| {
            match i {
                0 => {},
                _ => {
                    if self.targets_map.contains_key(&hero.id) {
                        let target = self.targets_map.get(&hero.id).unwrap();
        
                        match monsters.iter().find(|monster| {
                            let mut ret = false;
                            for ele in target {
                                if ele.id == monster.id {
                                    ret = true;
                                    break;
                                }
                            }
                            ret
                        }) {
                            Some(monster) => {
                                eprintln!("target {} of hero {} is still alive", monster.id, hero.id);
        
                                match monster.near_base {
                                    1 => {
                                        eprintln!("updating its position");
                                        let mut_target = self.targets_map.get_mut(&hero.id).unwrap();
                                        for ele in mut_target {
                                            if ele.id == monster.id {
                                                ele.x = monster.x;
                                                ele.y = monster.y;
                                            }
                                        }
                                    }
                                    _ => {
                                        eprintln!("monster {} is no longer a threat", monster.id);
                                        self.targets_map.insert(hero.id, Vec::new());
                                    }
                                };
                            }
                            None => {
                                eprintln!("target of hero {} is not found ! dead? ", hero.id);
                                self.targets_map.insert(hero.id, Vec::new());
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn MoveWitcher(&mut self, hero: &Entity,monsters: &Vec<Entity>, enemies: &Vec<Entity>, ret: &mut Vec<String>) {
        let (mut option1_point, mut option2_point) = ((-1, -1), (-1, -1));
        match self.init_params.base_x {
            0 => {
                option1_point = (14600, 6000);
                option2_point = (9500, 4000);
            }
            _ => {
                option1_point = (4400, 2000);
                option2_point = (8800, 4300);
            }
        }
        let d = Utils::distance((hero.x, hero.y), option1_point);
        if d < 5000. {
            self.witcher_reached_pos = true;
        }
        else {
            self.witcher_reached_pos = false;
        }
        let mut done = false;
        for ele in monsters {
            let d = Utils::distance((hero.x, hero.y), (ele.x, ele.y));
            if d < 1280.0 && self.players_mana[0] >= 50 {
                match self.init_params.base_x {
                    0 => ret.push(format!("SPELL WIND {} {}", 17630, 9000)),
                    _ => ret.push(format!("SPELL WIND {} {}", 0, 0)),
                }
                done = true;
                break;
            }
        }

        if done == false {
            match self.witcher_reached_pos {
                true => {
                    let mut min = std::f64::MAX;
                    let mut e = (-1, -1);
                    eprintln!("monsters size {}", monsters.len());                        
                    for m in monsters {
                        let d = Utils::distance((hero.x, hero.y), (m.x, m.y));
                        eprintln!("distance to monster {} {}", m.id, d);                        
                        if min > d {
                            min = d;
                            e = (m.x, m.y);
                        }
                    }
                    if e.0 != -1{
                        eprintln!("Found a monster !!!");                        
                        ret.push(format!("MOVE {} {}", e.0, e.1));
                    }
                    else{
                        eprintln!("couldn't find any mosnter !!!!!!!!!!!!!!");
                        ret.push(format!("MOVE {} {}", option1_point.0, option1_point.1));    
                    }
                },
                false => {
                    ret.push(format!("MOVE {} {}", option1_point.0, option1_point.1));
                },
            }
        }
    }
    pub fn MoveDefense(&mut self, i: i32, hero: &Entity, monsters: &Vec<Entity>, enemies: &Vec<Entity>, ret: &mut Vec<String>) {

        if !self.targets_map.contains_key(&hero.id) || self.targets_map[&hero.id].is_empty() {
            let mut targets = Vec::new();
            let distances_to_monsters = self.GetDistanceToMonsters(hero, &monsters);
            for _ in 0..3 {
                let e = distances_to_monsters.iter().next();
                if let Some(value) = e {
                    targets.push(value.1.clone());
                }
            }
            self.targets_map.insert(hero.id, targets);
        }

        let targets = self.targets_map.get(&hero.id).unwrap();
        if targets.len() > 0 {
            if targets.len() > 1 {
                let monster1 = targets[0];
                let monster2 = targets[1];
                let d1 = Utils::distance((hero.x, hero.y), (monster1.x, monster1.y));
                let d2 = Utils::distance((hero.x, hero.y), (monster2.x, monster2.y));
                let dist_to_base = Utils::distance(
                    (self.init_params.base_x, self.init_params.base_y),
                    (monster2.x, monster2.y),
                );
                if dist_to_base < 1000.0
                    && d1 < 1280.
                    && d2 < 1280.
                    && self.players_mana[0] >= 10
                {
                    ret.push(format!("SPELL WIND {} {}", 17630 / 2, 9000 / 2));
                } else {
                    ret.push(format!("MOVE {} {}", monster1.x, monster1.y));
                }
            } else {
                ret.push(format!("MOVE {} {}", targets[0].x, targets[0].y));
            }
        } else {
            let enemies = self.GetDistanceToEnemies(hero, &enemies);
            if enemies.len() > 0 {
                if Utils::distance((hero.x, hero.y), (enemies[0].1.x, enemies[0].1.y)) <= 1280.
                    && self.players_mana[0] >= 10
                {
                    let (mut new_x, mut new_y) = (0, 0);
                    match self.init_params.base_x {
                        0 => {
                            new_x = 17630;
                            new_y = 9000;
                        }
                        _ => {}
                    }

                    let s = format!("SPELL WIND {} {}", new_x, new_y);
                    ret.push(s);
                } else {
                    //ret.push(format!("MOVE {} {}", enemies[0].1.x, enemies[0].1.y));
                    match self.init_params.base_x {
                        0 => {
                            match i {
                                1 => {
                                    ret.push(format!("MOVE {} {}", 2300, 300));
                                },
                                2 => {
                                    ret.push(format!("MOVE {} {}", 1500, 2500));
                                },
                                _ => {},
                            }
                        }
                        _ => {
                            match i {
                                1 => {
                                    ret.push(format!("MOVE {} {}", 16700, 6500));
                                },
                                2 => {
                                    ret.push(format!("MOVE {} {}", 15000, 7500));
                                },
                                _ => {},
                            }
                        }
                    }
                }
            } else {
                match self.init_params.base_x {
                    0 => {
                        match i {
                            1 => {
                                ret.push(format!("MOVE {} {}", 2300, 300));
                            },
                            2 => {
                                ret.push(format!("MOVE {} {}", 1500, 2500));
                            },
                            _ => {},
                        }
                    }
                    _ => {
                        match i {
                            1 => {
                                ret.push(format!("MOVE {} {}", 16700, 6500));
                            },
                            2 => {
                                ret.push(format!("MOVE {} {}", 15000, 7500));
                            },
                            _ => {},
                        }
                    }
                }
            }
        }
    }
    pub fn GetActions(&mut self) -> Vec<String> {
        let [me, enemies, monsters] = self.split();

        ////
        self.UpdateTargets(&me, &monsters);

        let mut ret: Vec<String> = Vec::new();
        me.iter().enumerate().for_each(|(i, hero)| {
            match i {
                0 => {
                    self.MoveWitcher(hero, &monsters, &enemies, &mut ret);
                },
                _ => {
                   self.MoveDefense(i as i32, hero, &monsters, &enemies, &mut ret);
                },
            }
        });
        ret
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let base_x = parse_input!(inputs[0], i32); // The corner of the map representing your base
    let base_y = parse_input!(inputs[1], i32);
    eprintln!("I am located at base {}{}", base_x, base_y);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let heroes_per_player = parse_input!(input_line, i32); // Always 3

    let init_params = InitParams {
        base_x,
        base_y,
        ebase_x: 17630 - base_x,
        ebase_y: 9000 - base_y,
        heroes_per_player,
    };

    let mut game = Game::new(init_params);

    // game loop
    loop {
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let health = parse_input!(inputs[0], i32); // Each player's base health
            let mana = parse_input!(inputs[1], i32); // Ignore in the first league; Spend ten mana to cast a spell
            game.players_health[i] = health;
            game.players_mana[i] = mana;
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // Amount of heros and monsters you can see

        game.entities.clear();

        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // Unique identifier
            let _type = parse_input!(inputs[1], i32); // 0=monster, 1=your hero, 2=opponent hero
            let x = parse_input!(inputs[2], i32); // Position of this entity
            let y = parse_input!(inputs[3], i32);
            let shield_life = parse_input!(inputs[4], i32); // Ignore for this league; Count down until shield spell fades
            let is_controlled = parse_input!(inputs[5], i32); // Ignore for this league; Equals 1 when this entity is under a control spell
            let health = parse_input!(inputs[6], i32); // Remaining health of this monster
            let vx = parse_input!(inputs[7], i32); // Trajectory of this monster
            let vy = parse_input!(inputs[8], i32);
            let near_base = parse_input!(inputs[9], i32); // 0=monster with no target yet, 1=monster targeting a base
            let threat_for = parse_input!(inputs[10], i32); // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither
            let entity = Entity {
                id,
                _type,
                x,
                y,
                shield_life,
                is_controlled,
                health,
                vx,
                vy,
                near_base,
                threat_for,
            };

            game.entities.push(entity);
        }
        game.GetActions().iter().for_each(|s| {
            println!("{}", s);
        });
    }
}
