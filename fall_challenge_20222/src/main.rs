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
    heroes_per_player: i32,
}

struct Game {
    init_params: InitParams,
    entities: Vec<Entity>,
    players_health: [i32; 2],
    players_mana: [i32; 2],
    targets_map: HashMap<i32, Option<Entity>>,
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

    pub fn GetDistance(&self, entity: &Entity, other: &Vec<Entity>) -> Vec<(Entity, f64)> {
        let mut ret = Vec::new();
        other.iter().for_each(|o| {
            if o.near_base == 1 {
                let distance = Utils::distance((entity.x, entity.y), (o.x, o.y));
                ret.push((*o, distance));
            }
        });
        ret
    }

    // pub fn IsTargetAlive(&self, target: &Entity, group: &Vec<Entity>) -> Option<&Entity> {
        
    // }

    pub fn UpdateTargets(&mut self, myheres: &Vec<Entity>, monsters: &Vec<Entity>) {
        myheres.iter().for_each(|hero| {
            if self.targets_map.contains_key(&hero.id) {
                if let Some(target) = self.targets_map[&hero.id] {
                    match monsters.iter().find(|e| target.id == e.id) {
                        Some(monster) => {
                            eprintln!("target {} of hero {} is still alive", monster.id, hero.id);
                            eprintln!("updating its position");
                            self.targets_map.insert(hero.id, Some(*monster));
                        }
                        None => {
                            eprintln!("target of hero {} is not found ! dead? ", hero.id);
                            self.targets_map.insert(hero.id, None);
                        }
                    }
                }
            }
        });
    }

    pub fn GetActions(&mut self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let [me, enemies, monsters] = self.split();

        ////
        self.UpdateTargets(&me, &monsters);
        
        me.iter().for_each(|hero| {
            if !self.targets_map.contains_key(&hero.id) || self.targets_map[&hero.id] == None {
                let distances_to_monsters = self.GetDistance(hero, &monsters);
                let mut min = std::f64::MAX;
                let mut target = None;
                distances_to_monsters.iter().for_each(|(e, dist)| {
                    if min > *dist {
                        min = *dist;
                        target = Some(e);
                    }
                });
                self.targets_map.insert(hero.id, target.copied());
            }
        });

        self.targets_map.iter().for_each(|(k, v)|{
            if let Some(target) = self.targets_map[&k] {
                ret.push(format!("MOVE {} {}", target.x, target.y));
            } else {
                ret.push("WAIT".to_owned());
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
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let heroes_per_player = parse_input!(input_line, i32); // Always 3

    let initParams = InitParams {
        base_x,
        base_y,
        heroes_per_player,
    };

    let mut game = Game::new(initParams);

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
