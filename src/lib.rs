//use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
use std::{cell::RefCell, rc::Rc};

use rand::Rng;
use uuid::Uuid;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};


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
    predator_gestation: f32,
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
    pub fn get_current_tick(&self) -> u64 {
        self.current_tick
    }
    pub fn tick(&mut self) {

        // let mut new_grazers = Vec::new();
        // let mut new_predators = Vec::new();
        let mut new_plants = Vec::new();
        let mut plants_to_remove = Vec::new();
        let max_size = self.get_max_size() as f32; // Calculate it once

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
            //grazer.tick(&map);
        }
        for pred in self.predators.iter() {
            //pred.tick(&map);
        }
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
    pub fn add_grazer(&mut self, new_x: f32, new_y: f32, new_energy: i32) {
        let new_grazer = Grazer::new(new_x, new_y, new_energy);
        self.grazers.push(new_grazer);
    }
    pub fn add_predator(&mut self, new_x: f32, new_y: f32, new_energy: i32, new_gen_seq: String) {
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
    pub fn get_predator_gestation(&self) -> f32 {
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
    pub fn set_predator_gestation(&mut self, new_predator_gestation: f32) {
        self.predator_gestation = new_predator_gestation;
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
    fn set_gen(&mut self, new_gen: u32){
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
    state: i32, // needs to be enum of state
    velocity_x: f32,
    velocity_y: f32,
    orientation: f32,
    target_x: f32,
    target_y: f32,
    energy: i32,
}

#[wasm_bindgen]
impl Mover {
    //fn new(new_id: u32, new_x:i32, new_y: i32, new_state: i32, new_velocity_x: i32, new_velocity_y: i32, new_orientation: f32, new_target_x: i32,new_target_y: i32, new_energy: i32) -> Mover {
    //Mover { entity: Entity::default(), state: new_state, velocity_x: new_velocity_x, velocity_y: new_velocity_y, orientation: new_orientation, target_x: new_target_x, target_y: new_target_y, energy: new_energy }
    //}
    fn new(new_x: f32, new_y: f32, new_energy: i32) -> Mover {
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
    fn tick(&mut self) {
        self.entity.x += self.velocity_x;
        self.entity.y += self.velocity_y;
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
    fn get_energy(&self) -> i32 {
        self.energy
    }
    pub fn get_entity(&self) -> Entity {
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
    fn set_energy(&mut self, new_energy: i32) {
        self.energy = new_energy;
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
}

#[wasm_bindgen]
impl Grazer {
    fn new(new_x: f32, new_y: f32, new_energy: i32) -> Grazer {
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
    fn tick(&mut self, _map: &RefCell<&mut Map>) {
        self.mover.tick();
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
    fn reproduce(&mut self, map: &mut Map) {
        let new_energy = self.mover.energy / 2;
        map.add_grazer(self.mover.entity.x + 0.5, self.mover.entity.y, new_energy);
        self.mover.energy = new_energy;
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
    fn tick(&self,width:u32,height:u32, growth_rate: f32, max_size: u32, seed_distance: u32, seed_number: u32, viability: f32, cur_tick: u64) -> Vec<Plant>{
        
        let mut new_plants = Vec::new();

        if self.get_diameter() == 0.0 && self.grow_tick == cur_tick{
            //first growth
            let growth_rate =   growth_rate * max_size as f32;
            let mut fake_plant = self.clone();
            fake_plant.grow(growth_rate);
            let new_plant = fake_plant.clone();
            new_plants.push(new_plant);
        }
        else if self.is_max_size(max_size) && self.get_next_seed_tick() == cur_tick{
            //any seed event
            let mut copy_thingy = self.seed(width, height, max_size,seed_distance, seed_number, viability, cur_tick);
            new_plants.append(&mut copy_thingy);
            let new_plant = self.clone();
            new_plants.push(new_plant)


        }
        else if self.is_max_size(max_size) && self.get_next_seed_tick() == 0{
            //first check of max size that sets next seed tick
            let mut fake_plant = self.clone();
            fake_plant.set_next_seed_tick(cur_tick + 36); //change back to 3600 after testing
            let new_plant = fake_plant.clone();
            new_plants.push(new_plant);
        }
        else if !self.is_max_size(max_size){
            //all growth other than first after seed
            let mut fake_plant = self.clone();
            let growth_rate =   growth_rate * max_size as f32;
            fake_plant.grow(growth_rate);
            let new_plant = fake_plant.clone();
            new_plants.push(new_plant)
            
        }
        else{
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
    fn seed(&self, width: u32, height: u32, max_size: u32, seed_distance: u32, seed_number: u32, viability: f32, cur_tick: u64) -> Vec<Plant>{
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
                if new_x < 0.0{
                    new_x = 0.0;
                }
                if new_y < 0.0{
                    new_y = 0.0;
                }
                if new_x > width as f32{
                    new_x = width as f32;
                }
                if new_y > height as f32{
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
    mover: Mover,
    gen_seq: String,
    family: Vec<i32>, //vector of family ids
    time_family: u64, // time after mating that predator cares about family
    is_pregnant: bool,
    ticks_til_birth: u64, // the first tick where the gestation period is over
    mate_gen_seq: String, // mates gennetic sequence
}

#[wasm_bindgen]
impl Predator {
    fn new(new_x: f32, new_y: f32, new_energy: i32, new_gen_seq: String) -> Predator {
        Predator {
            mover: Mover::new(new_x, new_y, new_energy),
            gen_seq: new_gen_seq,
            ..Default::default()
        }
    }
    pub fn get_mover(&self) -> Mover {
        self.mover
    }
    pub fn get_entity(&self) -> Entity {
        self.mover.entity
    }
    fn tick(&mut self, _map: &RefCell<&mut Map>) {
        // an example of a mutable borrow of map is in map.tick 
		//at the end where the tick is incremented
        self.mover.tick();
    }
    fn get_gen_seq(&self) -> String {
        self.gen_seq.clone()
    }
    fn get_family(&self) -> Vec<i32> {
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
    fn get_mate_seq(&self) -> String {
        self.mate_gen_seq.clone()
    }
    fn set_gen_seq(&mut self, new_gen_seq: String) {
        self.gen_seq = new_gen_seq;
    }
    fn set_familiy(&mut self, new_family: Vec<i32>) {
        self.family = new_family;
    }
    fn add_family(&mut self, new_fam_id: i32) {
        self.family.push(new_fam_id);
    }
    fn set_time_family(&mut self, new_time_family: u64) {
        self.time_family = new_time_family;
    }
    fn set_is_pregnant(&mut self, is_pregnant: bool) {
        self.is_pregnant = is_pregnant;
    }
    fn set_ticks_til_birth(&mut self, map: Map, new_time_til_birth: u64) {
        self.ticks_til_birth = new_time_til_birth + map.get_current_tick();
    }
    fn set_mate_gen_seq(&mut self, new_mate_gen_seq: String) {
        self.mate_gen_seq = new_mate_gen_seq;
    }
}
