#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


use rand::Rng;
use uuid::Uuid;
mod utils;
use wasm_bindgen::{prelude::*, JsValue};

#[derive(Default)]
#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    current_tick: u64,
    //world constants
    //plant
    //counts arent needed
    init_plant_count: u32,
    growth_rate: f32,
    max_size: u32,
    max_seed_cast_distance: u32,
    max_seed_number: u32,
    seed_viability: f32,
    //grazer
    init_grazer_count: u32,
    grazer_energy_input: u32,
    grazer_energy_output: u32,
    grazer_energy_to_reproduce: u32,
    grazer_maintain_speed: f32,
    grazer_max_speed: f32,
    //predator
    init_predator_count: u32,
    max_speed_hod: f32,
    max_speed_hed: f32,
    max_speed_hor: f32,
    predator_maintain_speed: f32,
    predator_energy_output: u32,
    predator_energy_to_reproduce: u32,
    predator_max_offspring: u32,
    predator_gestation: u64,
    predator_offspring_energy: u32,

    rocks: Vec<Rock>,
    predators: Vec<Predator>,
    grazers: Vec<Grazer>,
    plants: Vec<Plant>,
}

#[wasm_bindgen]
impl Map {
    pub fn new() -> Map {
        Map::default()
    }
    fn get_current_tick(&self) -> u64 {
        self.current_tick
    }
    pub fn tick(&mut self) {
        let mut new_grazers = Vec::new();
        // let mut new_predators = Vec::new();
        let mut new_plants = Vec::new();
        let mut plants_to_remove = Vec::new();
        let max_size = self.get_max_size() as f32; // Calculate it once
        let maintain_speed_ticks = (self.grazer_maintain_speed * 60.0) as i32;

        for (index, plant) in self.plants.iter().enumerate() {
            let mut seeds = plant.tick(
                self.get_width(),
                self.get_height(),
                self.get_growth_rate(),
                self.get_max_size(),
                self.get_max_seed_cast_distance(),
                self.get_max_seed_number(),
                self.get_seed_viability(),
                self.get_current_tick(),
            );
            new_plants.append(&mut seeds);

            if plant.get_diameter() == 0.0 {
                // Check if there are other plants too close
                let is_too_close = self.plants.iter().enumerate().any(|(i, plant2)| {
                    i != index && // Exclude the current plant

                    ((plant.entity.x - plant2.entity.x).powi(2) + (plant.entity.y - plant2.entity.y).powi(2)).sqrt() < max_size
                });

                if is_too_close {
                    // Mark the current plant for removal
                    plants_to_remove.push(index);
                }
            }
        }

        for grazer in self.grazers.iter() {
            new_grazers.append(&mut grazer.clone().tick(self.get_grazer_energy_input(),
            self.get_grazer_energy_output(),
            self.get_grazer_energy_to_reproduce(),
            self.get_grazer_max_speed(),
            maintain_speed_ticks,
            self.get_plants_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 5.0),
            self.get_plants_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 150.0),
            self.get_predators_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 25.0),
            self.get_rocks_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 50.0),
            self.get_current_tick()
            ));
        }
        let mut preds = vec![];

        for pred in self.predators.iter() {
            preds.append(&mut pred.clone().tick(
                self.predator_energy_to_reproduce,
                self.current_tick,
                self.predator_energy_output,
                self.get_predators_within_vicinity(
                    pred.mover.get_entity().get_x(),
                    pred.mover.get_entity().get_y(),
                    5.0,
                ),
                self.predator_max_offspring,
                self.predator_offspring_energy,
                self.get_predator_gestation(),
                self.get_predator_by_id(pred.family.get(0)),
            ));
        }
        self.predators = preds;
        self.grazers = new_grazers;

        self.current_tick += 1;
        self.plants = new_plants;
        // Remove plants marked for removal
        // plants_to_remove.sort_by(|a, b| b.cmp(a)); // Sort in reverse order
        // for index in plants_to_remove {
        //     self.plants.remove(index);
        // }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_rocks(&self) -> js_sys::Array {
        self.rocks
            .clone()
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>()
    }
    pub fn get_grazers(&self) -> js_sys::Array {
        self.grazers
            .clone()
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>()
    }
    pub fn get_plants(&self) -> js_sys::Array {
        self.plants
            .clone()
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>()
    }
    pub fn get_predators(&self) -> js_sys::Array {
        self.predators
            .clone()
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>()
    }

    fn get_plants_within_vicinity(&self, x: f32, y: f32, max_dist: f32) -> Vec<Plant> {
        
        self.plants
            .iter()
            .filter(|plant| get_length(plant.entity.x - x, plant.entity.y - y) < max_dist)
            //.inspect(|pred| log(format!("{}", pred.mover.entity.x - x).as_str()))
            .map(|plant: &Plant| plant.clone())
            .collect::<Vec<Plant>>()
    }

    fn get_grazers_within_vicinity(&self, x: f32, y: f32, max_dist: f32) -> Vec<Grazer> {
        
        self.grazers
            .iter()
            .filter(|graz| get_length(graz.mover.entity.x - x, graz.mover.entity.y - y) < max_dist)
            //.inspect(|pred| log(format!("{}", pred.mover.entity.x - x).as_str()))
            .map(|graz: &Grazer| graz.clone())
            .collect::<Vec<Grazer>>()
    }

    fn get_rocks_within_vicinity(&self, x: f32, y: f32, max_dist: f32) -> Vec<Rock> {
        
        self.rocks
            .iter()
            .filter(|rock| get_length(rock.entity.x - x, rock.entity.y - y) < max_dist)
            //.inspect(|pred| log(format!("{}", pred.mover.entity.x - x).as_str()))
            .map(|rock: &Rock| rock.clone())
            .collect::<Vec<Rock>>()
    }

    fn get_predators_within_vicinity(&self, x: f32, y: f32, max_dist: f32) -> Vec<Predator> {
        self.predators
            .iter()
            .filter(|pred| get_length(pred.mover.entity.x - x, pred.mover.entity.y - y) < max_dist)
            //.inspect(|pred| log(format!("{}", pred.mover.entity.x - x).as_str()))
            .map(|pred: &Predator| pred.clone())
            .collect::<Vec<Predator>>()
    }

    pub fn get_rocks_size(&self) -> u32 {
        self.rocks.len() as u32
    }
    pub fn get_plants_size(&self) -> u32 {
        self.plants.len() as u32
    }
    pub fn get_grazers_size(&self) -> u32 {
        self.grazers.len() as u32
    }
    pub fn get_predators_size(&self) -> u32 {
        self.predators.len() as u32
    }

    pub fn add_rock(&mut self, x: f32, y: f32, diameter: u32, height: u32) {
        let new_rock = Rock::new(x, y, diameter, height);
        self.rocks.push(new_rock);
    }
    pub fn add_plant(&mut self, x: f32, y: f32, diameter: f32) {
        let new_plant = Plant::new(x, y, diameter);
        self.plants.push(new_plant);
    }
    pub fn add_grazer(&mut self, new_x: f32, new_y: f32, new_energy: u32) {
        let new_grazer = Grazer::new(new_x, new_y, new_energy);
        self.grazers.push(new_grazer);
    }
    pub fn add_predator(&mut self, new_x: f32, new_y: f32, new_energy: u32, new_gen_seq: String) {
        let new_predator = Predator::new(new_x, new_y, new_energy, new_gen_seq);
        self.predators.push(new_predator)
    }

    pub fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
    pub fn set_height(&mut self, new_height: u32) {
        self.height = new_height;
    }

    //plants
    pub fn get_init_plant_count(&self) -> u32 {
        self.init_plant_count
    }
    pub fn get_growth_rate(&self) -> f32 {
        self.growth_rate
    }
    pub fn get_max_size(&self) -> u32 {
        self.max_size
    }
    pub fn get_max_seed_cast_distance(&self) -> u32 {
        self.max_seed_cast_distance
    }
    pub fn get_max_seed_number(&self) -> u32 {
        self.max_seed_number
    }
    pub fn get_seed_viability(&self) -> f32 {
        self.seed_viability
    }
    pub fn set_init_plant_count(&mut self, new_init_plant_count: u32) {
        self.init_plant_count = new_init_plant_count;
    }
    pub fn set_growth_rate(&mut self, new_growth_rate: f32) {
        self.growth_rate = new_growth_rate;
    }
    pub fn set_max_size(&mut self, new_max_size: u32) {
        self.max_size = new_max_size;
    }
    pub fn set_max_seed_cast_distance(&mut self, new_max_seed_cast_distance: u32) {
        self.max_seed_cast_distance = new_max_seed_cast_distance;
    }
    pub fn set_max_seed_number(&mut self, new_max_seed_number: u32) {
        self.max_seed_number = new_max_seed_number;
    }
    pub fn set_seed_viability(&mut self, new_seed_viability: f32) {
        self.seed_viability = new_seed_viability;
    }

    //Grazers
    pub fn get_init_grazer_count(&self) -> u32 {
        self.init_grazer_count
    }
    pub fn get_grazer_energy_input(&self) -> u32 {
        self.grazer_energy_input
    }
    pub fn get_grazer_energy_output(&self) -> u32 {
        self.grazer_energy_output
    }
    pub fn get_grazer_energy_to_reproduce(&self) -> u32 {
        self.grazer_energy_to_reproduce
    }
    pub fn get_grazer_maintain_speed(&self) -> f32 {
        self.grazer_maintain_speed
    }
    pub fn get_grazer_max_speed(&self) -> f32 {
        self.grazer_max_speed
    }
    pub fn set_init_grazer_count(&mut self, new_init_grazer_count: u32) {
        self.init_grazer_count = new_init_grazer_count;
    }
    pub fn set_grazer_energy_input(&mut self, new_grazer_energy_input: u32) {
        self.grazer_energy_input = new_grazer_energy_input;
    }
    pub fn set_grazer_energy_output(&mut self, new_grazer_energy_output: u32) {
        self.grazer_energy_output = new_grazer_energy_output;
    }
    pub fn set_grazer_energy_to_reproduce(&mut self, new_grazer_energy_to_reproduce: u32) {
        self.grazer_energy_to_reproduce = new_grazer_energy_to_reproduce;
    }
    pub fn set_grazer_maintain_speed(&mut self, new_maintain_speed: f32) {
        self.grazer_maintain_speed = new_maintain_speed
    }
    pub fn set_grazer_max_speed(&mut self, new_max_speed: f32) {
        self.grazer_max_speed = new_max_speed
    }

    //predators
    fn get_predator_by_id(&self, id: Option<&Uuid>) -> Option<&Predator> {
        match id {
            Some(id) => self
                .predators
                .iter()
                .filter(|pred| pred.get_entity().get_id() == *id)
                .next(),
            None => None,
        }
    }
    pub fn get_init_predator_count(&self) -> u32 {
        self.init_predator_count
    }
    pub fn get_max_speed_hod(&self) -> f32 {
        self.max_speed_hod
    }
    pub fn get_max_speed_hed(&self) -> f32 {
        self.max_speed_hed
    }
    pub fn get_max_speed_hor(&self) -> f32 {
        self.max_speed_hor
    }
    pub fn get_predator_maintain_speed(&self) -> f32 {
        self.predator_maintain_speed
    }
    pub fn get_predator_energy_output(&self) -> u32 {
        self.predator_energy_output
    }
    pub fn get_predator_energy_to_reproduce(&self) -> u32 {
        self.predator_energy_to_reproduce
    }
    pub fn get_predator_max_offspring(&self) -> u32 {
        self.predator_max_offspring
    }
    pub fn get_predator_gestation(&self) -> u64 {
        self.predator_gestation
    }
    pub fn get_predator_offspring_energy(&self) -> u32 {
        self.predator_offspring_energy
    }
    pub fn set_init_predator_count(&mut self, new_init_predator_count: u32) {
        self.init_predator_count = new_init_predator_count;
    }
    pub fn set_max_speed_hod(&mut self, new_max_speed_hod: f32) {
        self.max_speed_hod = new_max_speed_hod;
    }
    pub fn set_max_speed_hed(&mut self, new_max_speed_hed: f32) {
        self.max_speed_hed = new_max_speed_hed;
    }
    pub fn set_max_speed_hor(&mut self, new_max_speed_hor: f32) {
        self.max_speed_hor = new_max_speed_hor;
    }
    pub fn set_predator_maintain_speed(&mut self, new_predator_maintain_speed: f32) {
        self.predator_maintain_speed = new_predator_maintain_speed;
    }
    pub fn set_predator_energy_output(&mut self, new_predator_energy_output: u32) {
        self.predator_energy_output = new_predator_energy_output;
    }
    pub fn set_predator_energy_to_reproduce(&mut self, new_predator_energy_to_reproduce: u32) {
        self.predator_energy_to_reproduce = new_predator_energy_to_reproduce;
    }
    pub fn set_predator_gestation(&mut self, new_predator_gestation: u32) {
        self.predator_gestation = new_predator_gestation as u64;
    }
    pub fn set_predator_offspring_energy(&mut self, new_predator_offspring_energy: u32) {
        self.predator_offspring_energy = new_predator_offspring_energy;
    }
    pub fn set_predator_max_offspring(&mut self, new_predator_max_offspring: u32) {
        self.predator_max_offspring = new_predator_max_offspring;
    }
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Entity {
    id: Uuid,
    x: f32,
    y: f32,
    generation: u32,
}

#[wasm_bindgen]
impl Entity {
    fn new(new_x: f32, new_y: f32) -> Entity {
        Entity {
            id: Uuid::new_v4(),
            x: new_x,
            y: new_y,
            ..Default::default()
        }
    }
    fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
    fn get_gen(&self) -> u32 {
        self.generation
    }

    fn set_id(&mut self, new_id: Uuid) {
        self.id = new_id;
    }

    fn set_x(&mut self, new_x: f32) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: f32) {
        self.y = new_y;
    }
    fn set_gen(&mut self, new_gen: u32) {
        self.generation = new_gen;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            id: Uuid::new_v4(),
            x: 0.0,
            y: 0.0,
            generation: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Mover {
    pub entity: Entity,
    state: i32, //0 = stay 1 = Arrive 2 = wander
    velocity_x: f32,
    velocity_y: f32,
    orientation: f32,
    target_x: f32,
    target_y: f32,
    energy: u32,
    du: f32,
    max_speed: f32,
}

#[wasm_bindgen]
impl Mover {
    //Mover { entity: Entity::default(), state: new_state, velocity_x: new_velocity_x, velocity_y: new_velocity_y, orientation: new_orientation, target_x: new_target_x, target_y: new_target_y, energy: new_energy }
    //}
    fn new(new_x: f32, new_y: f32, new_energy: u32) -> Mover {
        Mover {
            energy: new_energy,
            entity: Entity {
                x: new_x,
                y: new_y,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    fn tick(&mut self, max_speed: f32, energy: u32, target: Entity){

        if self.energy > 5 {
            // move here
            if self.state == 0{
                //stay
                //return self back
                // dont need to set anything to zero as will return a new copy of mover up to
            }
            else if self.state == 1 {
                Mover::arrive(self, target, 1 as f32);
            }
            else if self.state == 2 {
                Mover::wander(self, 1 as f32);
            }
            else if self.state == 3 {
                Mover::flee(self, target, 1 as f32);
            }

            self.du += f32::sqrt(self.entity.x.powi(2) + self.entity.y.powi(2));
            if self.du > 5.0 {
                self.du -= 5.0;
                self.energy -= energy;
            }
        }
        // a grazer can only move 10 du when energy is below 25
        //  add a death here for 0 energy
    }
    fn get_state(&self) -> i32 {
        //change to enum in future
        self.state
    }
    fn get_velocity_x(&self) -> f32 {
        self.velocity_x
    }
    fn get_velocity_y(self) -> f32 {
        self.velocity_y
    }
    pub fn get_orientation(&self) -> f32 {
        self.orientation
    }
    fn get_target_x(&self) -> f32 {
        self.target_x
    }
    fn get_target_y(&self) -> f32 {
        self.target_y
    }
    fn get_energy(&self) -> u32 {
        self.energy
    }
    fn get_entity(&self) -> Entity {
        self.entity
    }
    fn set_state(&mut self, new_state: i32) {
        //need to be enum here once we do that
        self.state = new_state;
    }
    fn set_velocity_x(&mut self, new_velocity_x: f32) {
        self.velocity_x = new_velocity_x;
    }
    fn set_velocity_y(&mut self, new_velocity_y: f32) {
        self.velocity_y = new_velocity_y;
    }
    fn set_orientation(&mut self, new_orientation: f32) {
        self.orientation = new_orientation;
    }
    fn set_target_x(&mut self, new_target_x: f32) {
        self.target_x = new_target_x;
    }
    fn set_target_y(&mut self, new_target_y: f32) {
        self.target_y = new_target_y;
    }
    fn set_energy(&mut self, new_energy: u32) {
        self.energy = new_energy;
    }
    fn get_length(x: f64, z: f64) -> f64 {
        return f64::sqrt((x * x) + (z * z));
    }

    fn normalize(mut x: f32, mut z: f32) -> (f32, f32) {
        let distance = get_length(x, z);
        x = x / distance;
        z = z / distance;
        return (x, z);
    }

    fn seek(mut char: &mut Mover, target: Entity, delta_time: f32) -> &Mover {
        let mut result_x = 0.0;
        let mut result_y = 0.0;

        result_x = target.x - char.entity.x;
        result_y = target.y - char.entity.y;

        (result_x, result_y) = Mover::normalize(result_x, result_y);
        result_x = result_x * char.max_speed;
        result_y = result_y * char.max_speed;

        return Mover::update(result_x, result_y, char, delta_time);
    }

    fn flee(mut char: &mut Mover, target: Entity, delta_time: f32) -> &Mover {
        let mut result_x = 0.0;
        let mut result_y = 0.0;

        result_x = char.entity.x - target.x;
        result_y = char.entity.y - target.y;

        (result_x, result_y) = Mover::normalize(result_x, result_y);
        result_x = result_x * char.max_speed;
        result_y = result_y * char.max_speed;

        return Mover::update(result_x, result_y, char, delta_time);
    }

    fn arrive(mut char: &mut Mover, target: Entity, delta_time: f32) -> &Mover {
        log("arrive");
        let mut result_x = 0.0;
        let mut result_y = 0.0;
        let mut goalSpeed = 0.0;
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        direction_x = target.x - char.entity.x;
        direction_y = target.y - char.entity.y;

        let distance = get_length(direction_x, direction_y);

        if distance < 1.0
        //This may need to be tested and fixed later.
        {
            return char;
        }

        if distance > char.max_speed + 5.0
        //This will also need to be tested.
        {
            goalSpeed = char.max_speed;

        } else {
            goalSpeed = char.max_speed * distance / (char.max_speed + 5.0) //The 1.5 is the slow radius, this needs to be tested.
        }

        let mut goal_velocity_x = direction_x;
        let mut goal_velocity_y = direction_y;

        (goal_velocity_x, goal_velocity_y) = Mover::normalize(goal_velocity_x, goal_velocity_y);
        goal_velocity_x = goal_velocity_x * goalSpeed;
        goal_velocity_y = goal_velocity_y * goalSpeed;

        result_x = goal_velocity_x - char.entity.x;
        result_y = goal_velocity_y - char.entity.y;

        /* These 2 lines might not be necessary, maybe need testing? maybe not?
        result_x = result_x / char.ttt;
        result_y = result_y / char.ttt;
        */

        return Mover::update(result_x, result_y, char, delta_time);
    }

    fn wander(mut char: &mut Mover, delta_time: f32) -> &Mover {
        let max_rotation = 15.0;
        let mut result_x = 0.0;
        let mut result_y = 0.0;
        let mut result_orien = 0.0;
        let num = rand::thread_rng().gen_range(-1.0..1.0);

       // log("max speed is ", char.max_speed);

        result_x = char.max_speed * char.orientation.sin() * 0.75;
        result_y = char.max_speed * char.orientation.cos() * 0.75;
        result_orien = num * max_rotation;

        return Mover::kinematicupdate(result_x, result_y, result_orien, char, delta_time);
    }

    fn update(result_x: f32, result_y: f32, char: &mut Mover, delta_time: f32) -> &Mover {
        char.entity.x += char.velocity_x * delta_time;
        char.entity.y += char.velocity_y * delta_time;

        char.velocity_x += result_x * delta_time;
        char.velocity_y += result_y * delta_time;

        if get_length(char.velocity_x, char.velocity_y) > char.max_speed {
            let (velocity_x, velocity_y) = Mover::normalize(char.velocity_x, char.velocity_y);
            char.velocity_x = velocity_x * char.max_speed;
            char.velocity_y = velocity_y * char.max_speed;
        }

        return char;
    }

    fn kinematicupdate(
        result_x: f32,
        result_y: f32,
        result_orien: f32,
        char: &mut Mover,
        delta_time: f32,
    ) -> &Mover {
        char.entity.x += char.velocity_x * delta_time;
        char.entity.y += char.velocity_y * delta_time;
        char.orientation += char.orientation * delta_time;

        char.velocity_x += result_x * delta_time;
        char.velocity_y += result_y * delta_time;
        char.orientation += result_orien * delta_time;

        if get_length(char.velocity_x, char.velocity_y) > char.max_speed {
            let (velocity_x, velocity_y) = Mover::normalize(char.velocity_x, char.velocity_y);
            char.velocity_x = velocity_x * char.max_speed;
            char.velocity_y = velocity_y * char.max_speed;
        }

        return char;
    }
}

impl Default for Mover {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Mover {
            entity: Entity::default(),
            state: 0,
            velocity_x: 1.0,
            velocity_y: 1.0,
            orientation: rng.gen_range(0.0..6.28),
            target_x: 0.0,
            target_y: 0.0,
            energy: 0,
            du: 0.0,
            max_speed: 10.0
        }
    }
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Rock {
    entity: Entity,
    diameter: u32,
    height: u32,
}

#[wasm_bindgen]
impl Rock {
    fn new(new_x: f32, new_y: f32, new_diameter: u32, new_height: u32) -> Rock {
        Rock {
            entity: Entity {
                x: new_x,
                y: new_y,
                ..Default::default()
            },
            diameter: new_diameter,
            height: new_height,
        }
    }

    fn get_x(&self) -> f32 {
        self.entity.get_x()
    }
    fn get_y(&self) -> f32 {
        self.entity.get_y()
    }
    pub fn get_diameter(&self) -> u32 {
        self.diameter
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn set_diameter(&mut self, new_diameter: u32) {
        self.diameter = new_diameter;
    }
    fn set_height(&mut self, new_height: u32) {
        self.height = new_height;
    }
    pub fn get_entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Clone, Copy, Default)]
#[wasm_bindgen]
pub struct Grazer {
    mover: Mover,
    ticks_in_loc: i32, //minutes in cur location without moving max is 10 once at 10 need to move
    ticks_at_speed: i32,
}

#[wasm_bindgen]
impl Grazer {
    fn new(new_x: f32, new_y: f32, new_energy: u32) -> Grazer {
        Grazer {
            mover: Mover {
                entity: Entity {
                    x: new_x,
                    y: new_y,
                    ..Default::default()
                },
                energy: new_energy,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    fn tick( &mut self, 
        energy_in: u32,
        energy_out: u32,
        energy_reproduce: u32,
        max_speed: f32,
        maintain_speed: i32,
        at_plants: Vec<Plant>, //in 5du
        plants: Vec<Plant>, //150 du
        predators: Vec<Predator>, //25 du
        rocks: Vec<Rock>, // 50 du
        cur_tick: u64,
    ) -> Vec<Grazer> {
        //plants and predator is a vector of creatures with distance sight is not yet implemented 
        //to integrate sight just change the function called when tick is called in map.
        let mut new_graz = Vec::new();
        self.mover.max_speed = max_speed;
        log(format!("length of plants {}", plants.len()).as_str());
        //first check for predators to run from
        if !predators.is_empty(){
            log("state is flee");
            //seek rock away from closest pred
            //set movers target
            self.mover.state = 1; //set state to arrive
            //log(plants.len());
            if !rocks.is_empty(){
            let mut min_dist = 0.0 as f32;
            let mut closest_rock = &rocks[0];
            for rock in rocks.iter(){
                let distance = ((rock.entity.x - self.mover.entity.x).powi(2) + (rock.entity.y - self.mover.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_rock = rock;
                    }
                }
            if self.ticks_at_speed < maintain_speed {
                self.mover.tick(max_speed, energy_out, closest_rock.entity);
                self.ticks_at_speed += 1;
            }
            else if min_dist == 0.0 {
                self.mover.tick(max_speed * 0.75, energy_out, closest_rock.entity);
            }
            
            }
            else {
                let mut min_dist = 0.0 as f32;
                let mut closest_pred = &predators[0];
                for pred in predators.iter(){
                    let distance = ((pred.mover.entity.x - self.mover.entity.x).powi(2) + (pred.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                    if distance < min_dist {
                        min_dist = distance;
                        closest_pred = pred;
                        }
                    }
                    self.mover.state = 3;
                    self.mover.tick(max_speed * 0.75, energy_out, closest_pred.mover.entity);
             }
        }
        else if self.mover.energy >= energy_reproduce {
            log("state is reproduce");
            new_graz.push(self.reproduce());
        }
        // here means no predators
        // check if at food for plant in 5 du
        // been at plant
        else if !at_plants.is_empty() && self.ticks_in_loc != 0{
            log("at plant");
            //now check if tick at loc is at max
            self.mover.state = 0;
            if self.ticks_in_loc == 600{
                // kill plant somehow
                self.ticks_in_loc = 0;
                //seek next plant   
            }
            else {
                //then if not max stay
                //gain energy on 100 increments
                if self.ticks_in_loc % 100 == 0 {
                    self.mover.energy += energy_in;
                }
            }

            
        }
        // first tick at plant
        else if !at_plants.is_empty() && self.ticks_in_loc == 0{
            log("arrived at plant");
            // just arrived at plant
            self.mover.state = 0;
            self.ticks_in_loc += 1;
            self.mover.tick(max_speed, energy_out, self.mover.entity);
        }

    
        else if at_plants.is_empty() && !plants.is_empty(){
            log("seek plant");
            //find closest plant and set arrive target
            self.mover.state = 1;
            let mut min_dist = 0.0 as f32;
            let mut closest_plant = &plants[0];
            for plant in plants.iter(){
                let distance = ((plant.entity.x - self.mover.entity.x).powi(2) + (plant.entity.y - self.mover.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_plant = plant;
                    }
                }
            self.mover.tick(max_speed, energy_out, closest_plant.entity);
        }
        else{
            log("wander");
            //start wandering
            self.mover.state = 2;
            self.mover.tick(max_speed, energy_out, self.mover.entity);
        }
        
        // only add grazers worthy of life
        if self.mover.energy != 0{
            new_graz.push(self.clone());
        }
        

        return new_graz;
        // self.mover.tick(5.0, energy);

    }

    fn get_ticks_in_loc(&self) -> i32 {
        self.ticks_in_loc
    }
    pub fn get_mover(&self) -> Mover {
        self.mover
    }
    pub fn get_entity(&self) -> Entity {
        self.mover.entity
    }
    fn set_ticks_in_loc(&mut self, new_min_in_loc: i32) {
        self.ticks_in_loc = new_min_in_loc;
    }
    fn reproduce(&mut self) -> Grazer {
        let new_energy = self.mover.energy / 2;
        let mut new_graz1 = Grazer::new(self.mover.entity.x + 5.0, self.mover.entity.y, new_energy);
        new_graz1.mover.entity.generation = self.mover.entity.generation + 1;
        self.mover.energy = new_energy;
        return new_graz1;
        // keep an eye on original grazer to make sure energy is set to new energy after new grazer exixsts
    }
}

#[derive(Clone, Copy, Default)]
#[wasm_bindgen]
pub struct Plant {
    entity: Entity,
    diameter: f32,
    next_seed_tick: u64,
    grow_tick: u64,
}

#[wasm_bindgen]
impl Plant {
    fn new(new_x: f32, new_y: f32, new_diameter: f32) -> Plant {
        Plant {
            entity: Entity::new(new_x, new_y),
            diameter: new_diameter,
            ..Default::default()
        }
    }
    pub fn get_diameter(&self) -> f32 {
        self.diameter
    }
    fn tick(
        &self,
        width: u32,
        height: u32,
        growth_rate: f32,
        max_size: u32,
        seed_distance: u32,
        seed_number: u32,
        viability: f32,
        cur_tick: u64,
    ) -> Vec<Plant> {
        let mut new_plants = Vec::new();

        if self.get_diameter() == 0.0 && self.grow_tick == cur_tick {
            //first growth
            let growth_rate = growth_rate * max_size as f32;
            let mut fake_plant = self.clone();
            fake_plant.grow(growth_rate);
            let new_plant = fake_plant.clone();
            new_plants.push(new_plant);
        } else if self.is_max_size(max_size) && self.get_next_seed_tick() == cur_tick {
            //any seed event
            let mut copy_thingy = self.seed(
                width,
                height,
                max_size,
                seed_distance,
                seed_number,
                viability,
                cur_tick,
            );
            new_plants.append(&mut copy_thingy);
            let new_plant = self.clone();
            new_plants.push(new_plant)
        } else if self.is_max_size(max_size) && self.get_next_seed_tick() == 0 {
            //first check of max size that sets next seed tick
            let mut fake_plant = self.clone();
            fake_plant.set_next_seed_tick(cur_tick + 3600); //change back to 3600 after testing
            let new_plant = fake_plant.clone();
            new_plants.push(new_plant);
        } else if !self.is_max_size(max_size) {
            //all growth other than first after seed
            let mut fake_plant = self.clone();
            let growth_rate = growth_rate * max_size as f32;
            fake_plant.grow(growth_rate);
            let new_plant = fake_plant.clone();
            new_plants.push(new_plant)
        } else {
            let plant = self.clone();
            new_plants.push(plant);
        }
        return new_plants;
        // an example of a mutable borrow of map is in map.tick
        //at the end where the tick is incremented
    }
    fn is_max_size(&self, max_size: u32) -> bool {
        return self.diameter >= (max_size as f32);
    }
    fn get_next_seed_tick(&self) -> u64 {
        self.next_seed_tick
    }
    fn get_grow_tick(&self) -> u64 {
        self.grow_tick
    }
    fn set_diameter(&mut self, new_diameter: f32) {
        self.diameter = new_diameter;
    }
    fn set_next_seed_tick(&mut self, new_tick: u64) {
        self.next_seed_tick = new_tick
    }
    fn set_grow_tick(&mut self, new_grow_tick: u64) {
        self.grow_tick = new_grow_tick;
    }
    fn set_generation(&mut self, new_gen: u32) {
        self.entity.set_gen(new_gen);
    }
    // make function that just increments by 1hrs worth of ticks
    // not includeing set grow tick as this only matters to new plant and never needs to be used again

    pub fn get_entity(&self) -> Entity {
        self.entity
    }
    //need actual seeding functions
    fn grow(&mut self, growth_add: f32) {
        // where growth add is growth rate * max size (this should be calculated in map and passed to this function)
        if self.diameter == 0.0 {
            self.diameter = 0.01
        }
        self.diameter += growth_add;
    }
    fn seed(
        &self,
        width: u32,
        height: u32,
        max_size: u32,
        seed_distance: u32,
        seed_number: u32,
        viability: f32,
        cur_tick: u64,
    ) -> Vec<Plant> {
        // need tick to second ratio 1:1
        // seeds start growing after 10 seconds so should add delay_growth: till specific tick to plant
        // need to add next_seed_tick as well 1 hour between seed events
        // need rng for seed count 0-Max seed count
        let mut new_plants = Vec::new();
        let mut rng = rand::thread_rng();
        let seed_num = rng.gen_range(0..seed_number);
        let mut i = 1;
        while i <= seed_num {
            let good_seed = rng.gen_range(0.0..100.0);
            if good_seed > viability {
                //if seed is viable make plant
                //generate coords
                let new_angle = rng.gen_range(0.0..360.0) as f32;
                let new_distance = rng.gen_range(max_size..seed_distance) as f32;
                let mut new_x = self.entity.get_x() + (new_distance * new_angle.cos());
                let mut new_y = self.entity.get_y() + (new_distance * new_angle.sin());
                let new_grow_tick = cur_tick + 10;
                let new_gen = self.entity.get_gen() + 1;

                //bound checking
                if new_x < 0.0 {
                    new_x = 0.0;
                }
                if new_y < 0.0 {
                    new_y = 0.0;
                }
                if new_x > width as f32 {
                    new_x = width as f32;
                }
                if new_y > height as f32 {
                    new_y = height as f32;
                }

                let new_x = new_x;
                let new_y = new_y;

                let mut new_plant = Plant::new(new_x, new_y, 0.0);
                new_plant.set_next_seed_tick(0);
                new_plant.set_grow_tick(new_grow_tick);
                new_plant.set_generation(new_gen);

                new_plants.push(new_plant);
            }
            i += 1;
        }
        return new_plants;
    }
}

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Predator {
    pub mover: Mover,
    family: Vec<Uuid>, //vector of family ids
    time_family: u64,  // time after mating that predator cares about family
    is_pregnant: bool,
    ticks_til_birth: u64, // the first tick where the gestation period is over
    agression: Gene,
    strength: Gene,
    speed: Gene,
}

#[wasm_bindgen]
impl Predator {
    pub fn new(new_x: f32, new_y: f32, new_energy: u32, new_gen_seq: String) -> Predator {
        let mut new = Predator {
            mover: Mover::new(new_x, new_y, new_energy),
            ..Default::default()
        };
        new.parse_gen_seq(new_gen_seq);
        new
    }
    pub fn get_mover(&self) -> Mover {
        self.mover
    }
    pub fn get_entity(&self) -> Entity {
        self.mover.entity
    }
    fn tick(
        &mut self,
        energy_to_reproduce: u32,
        cur_tick: u64,
        energy: u32,
        preds: Vec<Predator>,
        max_offspring: u32,
        offspring_energy: u32,
        gestation: u64,
        partner: Option<&Predator>,
    ) -> Vec<Predator> {
        let mut ret = vec![];
        // if energy and not pregnant
        // has a mate
        // mate
        // perform birth() for both parents
        // add birthed predators to ret
        // set is_pregnant
        // set gestation
        // add mate to avoid list

        // need to filter for avoid list

        let pred = preds
            .iter()
            .filter(|p| p.willing_to_mate(energy_to_reproduce))
            .filter(|p| p.get_entity().get_id() != self.get_entity().get_id())
            //.inspect(|pred| log(pred.get_entity().get_id().to_string().as_str()))
            .next();

        if self.willing_to_mate(energy_to_reproduce) {
            // if vaible candidate is found
            if let Some(pred) = pred {
                self.mate(&mut pred.clone(), cur_tick, gestation);
                log("viable mate found");
            }
        } else if self.is_pregnant {
            if self.get_ticks_til_birth() < cur_tick {
                if let Some(partner) = partner {
                    ret.append(&mut self.birth(
                        max_offspring,
                        offspring_energy,
                        partner.clone(),
                        energy_to_reproduce,
                        self.get_entity().get_x(),
                        self.get_entity().get_y(),
                    ));
                }
            }
        }

        self.mover.tick(5.0, energy, self.mover.entity);
        ret.push(self.clone());
        ret
    }
    fn willing_to_mate(&self, rep_energy: u32) -> bool {
        (self.mover.energy >= rep_energy) && !self.is_pregnant
    }
    pub fn get_gen_seq(&self) -> String {
        let ag = match self.agression {
            Gene::Hetero => "Hetero agression, ",
            Gene::HomoDominant => "Homo Dom agression, ",
            Gene::HomoRecessive => "Homo Rec agression, ",
        }
        .to_owned();

        let strength = match self.strength {
            Gene::Hetero => "Hetero strength, ",
            Gene::HomoDominant => "Homo Dom strength, ",
            Gene::HomoRecessive => "Homo Rec strength, ",
        }
        .to_owned();

        let speed = match self.speed {
            Gene::Hetero => "Hetero speed",
            Gene::HomoDominant => "Homo Dom speed",
            Gene::HomoRecessive => "Homo Rec speed",
        }
        .to_owned();
        format!("{}{}{}", ag, strength, speed)
    }
    fn get_family(&self) -> Vec<Uuid> {
        self.family.clone()
    }
    fn get_time_family(&self) -> u64 {
        self.time_family
    }
    fn get_is_pregnant(&self) -> bool {
        self.is_pregnant
    }
    fn get_ticks_til_birth(&self) -> u64 {
        self.ticks_til_birth
    }
    fn get_mate_seq(&self) -> (Gene, Gene, Gene) {
        (self.agression, self.strength, self.speed)
    }
    fn add_family(&mut self, new_fam_id: Uuid) {
        self.family.push(new_fam_id);
    }
    fn set_time_family(&mut self, new_time_family: u64) {
        self.time_family = new_time_family;
    }
    fn set_is_pregnant(&mut self, is_pregnant: bool) {
        self.is_pregnant = is_pregnant;
    }
    fn set_ticks_til_birth(&mut self, new_time_til_birth: u64) {
        self.ticks_til_birth = new_time_til_birth;
    }
    fn parse_gen_seq(&mut self, new_mate_gen_seq: String) {
        // need genetic code verification and error handling
        if new_mate_gen_seq.contains("aa") {
            self.agression = Gene::HomoRecessive;
        } else if new_mate_gen_seq.contains("Aa") {
            self.agression = Gene::Hetero;
        } else if new_mate_gen_seq.contains("AA") {
            self.agression = Gene::HomoDominant;
        }
        if new_mate_gen_seq.contains("ss") {
            self.strength = Gene::HomoRecessive;
        } else if new_mate_gen_seq.contains("Ss") {
            self.strength = Gene::Hetero;
        } else if new_mate_gen_seq.contains("SS") {
            self.strength = Gene::HomoDominant;
        }
        if new_mate_gen_seq.contains("ff") {
            self.speed = Gene::HomoRecessive;
        } else if new_mate_gen_seq.contains("Ff") {
            self.speed = Gene::Hetero;
        } else if new_mate_gen_seq.contains("FF") {
            self.speed = Gene::HomoDominant;
        }
    }

    fn mate_genes(parent1: &Predator, parent2: &Predator) -> (Gene, Gene, Gene) {
        (
            parent1.agression.mate(parent2.agression),
            parent1.strength.mate(parent2.strength),
            parent1.speed.mate(parent2.speed),
        )
    }
    fn mate(&mut self, other: &mut Predator, cur_tick: u64, gestation: u64) {
        self.set_is_pregnant(true);
        other.set_is_pregnant(true);
        self.set_ticks_til_birth(cur_tick + gestation);
        other.set_ticks_til_birth(cur_tick + gestation);
        self.add_family(other.get_entity().get_id());
        other.add_family(self.get_entity().get_id());
    }
    fn birth(
        &mut self,
        max_offspring: u32,
        new_energy: u32,
        other: Predator,
        energy_to_reproduce: u32,
        new_x: f32,
        new_y: f32,
    ) -> Vec<Predator> {
        let mut preds = vec![];

        let children = rand::thread_rng().gen_range(0..=max_offspring);

        // loop through each child
        for _ in 0..children {
            let new_genes = Predator::mate_genes(self, &other);
            let new_pred = Predator {
                agression: new_genes.0,
                strength: new_genes.1,
                speed: new_genes.2,
                mover: Mover {
                    energy: new_energy,
                    entity: Entity {
                        x: new_x,
                        y: new_y,
                        ..Default::default() //TODO add generation??
                    },
                    ..Default::default()
                },
                ..Default::default()
            };
            //TODO need to add family logic
            preds.push(new_pred);
        }
        self.mover.energy -= energy_to_reproduce;
        self.set_is_pregnant(false);
        preds
    }
}

#[derive(Clone, Copy, Default)]
#[wasm_bindgen]
pub enum Gene {
    HomoDominant,
    #[default]
    Hetero,
    HomoRecessive,
}

impl Gene {
    fn mate(self, other: Gene) -> Gene {
        let rand: u8 = rand::thread_rng().gen_range(0..4);
        match (self, other) {
            (Gene::HomoDominant, Gene::HomoDominant) => Gene::HomoDominant,
            (Gene::HomoRecessive, Gene::HomoRecessive) => Gene::HomoRecessive,
            (Gene::HomoDominant, Gene::HomoRecessive) => Gene::Hetero,
            (Gene::HomoRecessive, Gene::HomoDominant) => Gene::Hetero,
            (Gene::HomoDominant, Gene::Hetero) => match rand {
                0..=1 => Gene::Hetero,
                2..=3 => Gene::HomoDominant,
                _ => {
                    panic!()
                }
            },
            (Gene::Hetero, Gene::HomoDominant) => match rand {
                0..=1 => Gene::Hetero,
                2..=3 => Gene::HomoDominant,
                _ => {
                    panic!()
                }
            },
            (Gene::HomoRecessive, Gene::Hetero) => match rand {
                0..=1 => Gene::Hetero,
                2..=3 => Gene::HomoRecessive,
                _ => {
                    panic!()
                }
            },
            (Gene::Hetero, Gene::HomoRecessive) => match rand {
                0..=1 => Gene::Hetero,
                2..=3 => Gene::HomoRecessive,
                _ => {
                    panic!()
                }
            },
            (Gene::Hetero, Gene::Hetero) => match rand {
                0 => Gene::HomoDominant,
                1..=2 => Gene::Hetero,
                3 => Gene::HomoRecessive,
                _ => {
                    panic!()
                }
            },
        }
    }
}

fn get_length(x: f32, z: f32) -> f32 {
    return f32::sqrt((x * x) + (z * z));
}
