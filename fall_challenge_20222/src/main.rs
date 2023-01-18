use core::panic;
use std::io;

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
    witcher_reached_pos: bool,
    monsterIsTaken: i32,
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
            witcher_reached_pos: false,
            monsterIsTaken : -1,
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
        entity: (i32, i32),
        other: &Vec<Entity>,
    ) -> Vec<(f64, Entity)> {
        let mut ret = Vec::new();

        other.iter().for_each(|monster| {
            let dist_to_base = Utils::distance(
                (self.init_params.base_x, self.init_params.base_y),
                (monster.x, monster.y),
            );
            if dist_to_base < 8000. && monster.id != self.monsterIsTaken {
                let distance = Utils::distance((entity.0, entity.1), (monster.x, monster.y));
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
            if dist_to_base < 7000. {
                let distance = Utils::distance((entity.x, entity.y), (enemy.x, enemy.y));
                ret.push((distance, *enemy));
            }
        });
        ret.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        ret
    }

    // pub fn IsTargetAlive(&self, target: &Entity, group: &Vec<Entity>) -> Option<&Entity> {

    // }

    pub fn GetEnemyBaseLocation(&self) -> (i32, i32) {
        match self.init_params.base_x {
            0 => (17630, 9000),
            _ => (0, 0),
        }
    }

    pub fn WitcherToEnemiesStategy(&self, hero: &Entity, enemies: &Vec<Entity>) -> Option<String> {
        let mut ret = None;
        let enemy_base_location = self.GetEnemyBaseLocation();
        let dist_hero_to_enemy_base = Utils::distance(
            (hero.x, hero.y),
            (enemy_base_location.0, enemy_base_location.1),
        );
        let mut max_control_score = 0.;
        let mut enemy_to_be_controlled_id = -1;

        for enemy in enemies {
            let dist_hero_enemy = Utils::distance((hero.x, hero.y), (enemy.x, enemy.y));
            if dist_hero_enemy <= 2200.
                && dist_hero_to_enemy_base < 7000.
                && self.players_mana[0] >= 50
            {
                if hero.shield_life <= 1  {
                    ret = Some(format!("SPELL SHIELD {}", hero.id));
                    break;
                } else if enemy.is_controlled != 1 {
                    let dist_enemy_enemy_base = Utils::distance(
                        (enemy.x, enemy.y),
                        (enemy_base_location.0, enemy_base_location.1),
                    );
                    //if (dist_enemy_enemy_base < 2000.){
                    let score = 1. / dist_enemy_enemy_base;
                    if score > max_control_score {
                        max_control_score = score;
                        enemy_to_be_controlled_id = enemy.id;
                    }
                    //}
                }
            }
        }

        if max_control_score != 0. {
            ret = Some(format!(
                "SPELL CONTROL {} {} {}",
                enemy_to_be_controlled_id, self.init_params.base_x, self.init_params.base_y
            ))
        }
        ret
    }

    fn MonsterTargetEnemy(&self, monster: &Entity,) -> bool {
        match self.init_params.base_x {
            0 => monster.vx >0 && monster.vy > 0,
            _ => monster.vx <0 && monster.vy < 0
        }
        
    }
    pub fn WitcherToMonstersStrategy(
        &self,
        hero: &Entity,
        monsters: &Vec<Entity>,
        enemies: &Vec<Entity>,
    ) -> Option<String> {
        let mut ret = None;

        let mut max_score_wind = 0.;
        let mut max_score_control = 0.;
        let mut max_score_shield = 0.;
        let mut best_score_selected_monster = std::f64::MAX;

        let enemy_base_location = self.GetEnemyBaseLocation();
        let mut pos_closed_monster_to_enemy = (-1, -1);
        let mut min_pos_to_enemy_base = std::f64::MAX;
        let mut mosnter_id_to_be_controller = -1;
        let mut mosnter_id_to_be_shielded = -1;
        for monster in monsters {

            let dist_monster_enemy_base = Utils::distance(
                (monster.x, monster.y),
                (enemy_base_location.0, enemy_base_location.1),
            );

            let dist_monster_hero = Utils::distance((monster.x, monster.y), (hero.x, hero.y));

            let mut score = dist_monster_enemy_base;

            // if monster.is_controlled == 1 {
            //     eprintln!("monster is already threat for 2");
            //     score += 500.;
            // }
            // if monster.threat_for == 2 {
            //     score += 500.;
            // }

            eprintln!(
                "monster id {} dist_monster_enemy_base {} , dist_monster_hero {}, score = {}",
                monster.id, dist_monster_enemy_base, dist_monster_hero, score
            );

            if score < best_score_selected_monster && monster.shield_life == 0 {
                best_score_selected_monster = score;
                pos_closed_monster_to_enemy = (monster.x, monster.y);
            }
         
            // if dist_monster_enemy_base < min_pos_to_enemy_base {
            //     min_pos_to_enemy_base = dist_monster_enemy_base;
            //     pos_closed_monster_to_enemy = (monster.x, monster.y);
            // }

            
            // let mut min_distance_to_enemy = std::f64::MAX;
            // for e in enemies {
            //     let d = Utils::distance((monster.x, monster.y), (e.x, e.y));
            //     if min_distance_to_enemy > d {
            //         min_distance_to_enemy = d;
            //     }
            // }

            let dist_hero_monster = Utils::distance((monster.x, monster.y), (hero.x, hero.y));
            match dist_monster_enemy_base < 8000.0 {
                true => {
                    if  dist_hero_monster < 1280.0 && self.players_mana[0] >= 50
                            && monster.health >= 5
                            && monster.shield_life == 0
                           // && min_distance_to_enemy > 2200.
                        {
                            let score_wind = 1. / dist_monster_enemy_base;
                            if max_score_wind < score_wind {
                                max_score_wind = score_wind;
                            }
                        }
                    else if dist_hero_monster < 2200. && dist_monster_enemy_base < 6000.{
                        if self.players_mana[0] >= 10
                            && monster.shield_life == 0 && self.MonsterTargetEnemy(monster) && monster.health >= 10
                        {
                            max_score_shield = 10000.;
                            mosnter_id_to_be_shielded  = monster.id;
                            // if max_score_shield < score_shield {
                            //     max_score_shield = score_shield;
                            //     mosnter_id_to_be_shielded = monster.id;
                            // }
                        }
                    }
                }
                false => {
                    if self.players_mana[0] >= 50
                        && dist_hero_monster < 2200.0
                        && monster.threat_for != 2
                        && monster.health >= 10
                        && monster.shield_life == 0
                        && monster.is_controlled != 1
                    {
                        let score_control = 1. / dist_monster_enemy_base;
                        if max_score_control < score_control {
                            max_score_control = score_control;
                            mosnter_id_to_be_controller = monster.id;
                        }
                    }
                }
            }
        }
        eprintln!("max_score_wind {} max_score_control {} max_score_shield {}", max_score_wind,max_score_control, max_score_shield);
        if max_score_wind == 0. && max_score_control == 0. && max_score_shield == 0. {
            if pos_closed_monster_to_enemy.0 == -1 {
                match self.init_params.base_x {
                    0 => {
                        ret = Some(format!("MOVE {} {}", 12000, 4200));
                    }
                    _ => {
                        ret = Some(format!("MOVE {} {}", 6000, 3200));
                    }
                }
            } else {
                let dist = Utils::distance(self.GetEnemyBaseLocation(), pos_closed_monster_to_enemy);
                if dist < 8000. && dist > 4000. {
                    ret = Some(format!(
                        "MOVE {} {} {}",
                        pos_closed_monster_to_enemy.0, pos_closed_monster_to_enemy.1, "moving to the closed monster I've found"
                    ));
                }
                else{
                    match self.init_params.base_x {
                        0 => {
                            ret = Some(format!("MOVE {} {} {}",13636, 6445, "moving to the closed monster I've found"));
                        },
                        _ => {
                            ret = Some(format!("MOVE {} {} {}",4000, 2500, "moving to the closed monster I've found"));
                        }
                    }
                    
                }
            }
        } else {
            if max_score_shield == 10000. && mosnter_id_to_be_shielded != -1 {
                ret = Some(format!("SPELL SHIELD {}", mosnter_id_to_be_shielded))
            }
            else{

                if max_score_wind >= max_score_control {
                        ret = Some(format!(
                            "SPELL WIND {} {}",
                            enemy_base_location.0, enemy_base_location.1
                        ))
                }
                else 
                {
                    if max_score_control > max_score_shield {
                        ret = Some(format!(
                            "SPELL CONTROL {} {} {}",
                            mosnter_id_to_be_controller, enemy_base_location.0, enemy_base_location.1
                        ))
                    }
                }
            }
        }
        ret
    }

    pub fn MoveWitcher(
        &mut self,
        hero: &Entity,
        monsters: &Vec<Entity>,
        enemies: &Vec<Entity>,
    ) -> String {

        let action = self.WitcherToMonstersStrategy(hero, monsters, enemies);
        if action.is_some() {
            action.unwrap()
        } else {
            let action = self.WitcherToEnemiesStategy(hero, enemies);
            if action.is_none() {
                String::from("WAIT")
            } else {
                action.unwrap()
            }
        }
    }
    fn MoveDefenseToInitPos(&self, i: i32, )->String{
        let ret;
        match self.init_params.base_x {
            0 => match i {
                1 => {
                    ret = format!("MOVE {} {}", 4294, 3454);
                }
                2 => {
                    ret = format!("MOVE {} {}", 5359, 1529);
                }
                _ => {panic!("")}
            },
            _ => match i {
                1 => {
                    ret = format!("MOVE {} {}", 14519, 3901);
                }
                2 => {
                    ret = format!("MOVE {} {}", 12709, 6584);
                }
                _ => {panic!("")}
            },
        }
        ret
    }

    pub fn MoveDefense(
        &mut self,
        i: i32,
        hero: &Entity,
        monsters: &Vec<Entity>,
        enemies: &Vec<Entity>,
    ) -> String {
        let mut ret = String::new();
        let mut targets = Vec::new();
        let distances_to_monsters = self.GetDistanceToMonsters(
            (self.init_params.base_x, self.init_params.base_y),
            &monsters,
        );
        for _ in 0..3 {
            let e = distances_to_monsters.iter().next();
            if let Some(value) = e {
                targets.push(value.1.clone());
            }
        }

        let enemies = self.GetDistanceToEnemies(hero, &enemies);
         if targets.len() > 0 {
            // deal with monsterss
            let monster1 = targets[0];
            let d1 = Utils::distance((hero.x, hero.y), (monster1.x, monster1.y));
            let dist_to_base = Utils::distance(
                (self.init_params.base_x, self.init_params.base_y),
                (monster1.x, monster1.y),
            );
            if dist_to_base < 2000.0 && self.players_mana[0] >= 10 {
                if d1 < 1280.0 && monster1.shield_life == 0 {
                    ret = format!("SPELL WIND {} {}", 17630 / 2, 9000 / 2);
                    self.monsterIsTaken = monster1.id;
                } else {
                    ret = format!("MOVE {} {}", monster1.x, monster1.y);
                }
            } else {
                ret = format!("MOVE {} {}", monster1.x, monster1.y);
            }
        } else {
                ret = self.MoveDefenseToInitPos(i);
        }
        ret
    }

    pub fn GetActions(&mut self) -> Vec<String> {
        let [me, enemies, monsters] = self.split();
        self.monsterIsTaken = -1;
        let mut ret: Vec<String> = Vec::new();
        ret.resize(3, String::new());

        me.iter().enumerate().for_each(|(i, hero)| match i {
            0 => {
                ret[i] = self.MoveWitcher(hero, &monsters, &enemies);
            }
            _ => {
                ret[i] = self.MoveDefense(i as i32, hero, &monsters, &enemies);
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
