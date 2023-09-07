use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//use crate::grazer::grazer::Grazer;
//use crate::plant::plant::Plant;
//use crate::rock::rock::Rock;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Default)]
#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    current_tick: u64,
    //world constants
    //plant
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
    predators: Vec<Grazer>,
}

#[wasm_bindgen]
impl Map {
    pub fn new() -> Map {
        return Map::default();
    }
    pub fn get_current_tick(&self) -> u64 {
        self.current_tick
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
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
}

trait BaseEntity {
    fn get_id(&self) -> &u32;
    fn get_x(&self) -> &i32;
    fn get_y(&self) -> &i32;
    fn set_id(&mut self, _: u32);
    fn set_x(&mut self, _: i32);
    fn set_y(&mut self, _: i32);
}

pub struct Entity {
    id: u32,
    x: i32,
    y: i32,
}

impl BaseEntity for Entity {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_x(&self) -> &i32 {
        &self.x
    }

    fn get_y(&self) -> &i32 {
        &self.y
    }

    fn set_id(&mut self, new_id: u32) {
        self.id = new_id;
    }

    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }
}

pub struct Mover {
    entity: Entity,
    state: i32, // needs to be enum of state
    velocity_x: i32,
    velocity_y: i32,
    orientation: f32,
    target_x: i32,
    target_y: i32,
    energy: i32,
}

impl Mover {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn get_state(&self) -> &i32 {
        //change to enum in future
        &self.state
    }
    fn get_velocity_x(&self) -> &i32 {
        &self.velocity_x
    }
    fn get_velocity_y(&self) -> &i32 {
        &self.velocity_y
    }
    fn get_orientation(&self) -> &f32 {
        &self.orientation
    }
    fn get_target_x(&self) -> &i32 {
        &self.target_x
    }
    fn get_target_y(&self) -> &i32 {
        &self.target_y
    }
    fn get_energy(&self) -> &i32 {
        &self.energy
    }
    fn set_state(&mut self, new_state: i32) {
        //need to be enum here once we do that
        self.state = new_state;
    }
    fn set_velocity_x(&mut self, new_velocity_x: i32) {
        self.velocity_x = new_velocity_x;
    }
    fn set_velocity_y(&mut self, new_velocity_y: i32) {
        self.velocity_y = new_velocity_y;
    }
    fn set_orientation(&mut self, new_orientation: f32) {
        self.orientation = new_orientation;
    }
    fn set_target_x(&mut self, new_target_x: i32) {
        self.target_x = new_target_x;
    }
    fn set_target_y(&mut self, new_target_y: i32) {
        self.target_y = new_target_y;
    }
    fn set_energy(&mut self, new_energy: i32) {
        self.energy = new_energy;
    }
}

pub struct Rock {
    entity: Entity,
    diameter: u32,
}

impl Rock {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn get_diameter(&self) -> &u32 {
        &self.diameter
    }
    fn set_diameter(&mut self, new_diameter: u32) {
        self.diameter = new_diameter
    }
}

pub struct Grazer {
    mover: Mover,
    min_in_loc: i32, //minutes in cur location without moving max is 10 once at 10 need to move
}

impl Grazer {
    fn get_min_in_loc(&self) -> &i32 {
        &self.min_in_loc
    }
    fn set_min_in_loc(&mut self, new_min_in_loc: i32) {
        self.min_in_loc = new_min_in_loc
    }
}

pub struct Plant {
    entity: Entity,
    diameter: u32,
}

impl Plant {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn get_diameter(&self) -> &u32 {
        &self.diameter
    }
    fn is_max_size(&mut self, map: &Map) -> bool {
        self.diameter >= map.get_max_size()
    }
    fn set_diameter(&mut self, new_diameter: u32) {
        self.diameter = new_diameter;
    }
    //need actual seeding functions
}

struct Predator {
    mover: Mover,
    gen_seq: String,
    family: Vec<i32>, //vector of family ids
    time_family: f32, // time after mating that predator cares about family
    is_pregnant: bool,
    time_til_birth: u64,  // the first tick where the gestation period is over
    mate_gen_seq: String, // mates gennetic sequence
}

impl Predator {
    fn get_mover(&self) -> &Mover {
        &self.mover
    }
    fn get_gen_seq(&self) -> &String {
        &self.gen_seq
    }
    fn get_family(&self) -> &Vec<i32> {
        &self.family
    }
    fn get_time_family(&self) -> &f32 {
        &self.time_family
    }
    fn get_is_pregnant(&self) -> &bool {
        &self.is_pregnant
    }
    fn get_time_til_birth(&self) -> &u64 {
        &self.time_til_birth
    }
    fn get_mate_seq(&self) -> &String {
        &self.mate_gen_seq
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
    fn set_time_family(&mut self, new_time_family: f32) {
        self.time_family = new_time_family;
    }
    fn set_is_pregnant(&mut self, is_pregnant: bool) {
        self.is_pregnant = is_pregnant;
    }
    fn set_time_til_birth(&mut self, map: Map, new_time_til_birth: u64) {
        self.time_til_birth = new_time_til_birth + Map::get_current_tick(&map);
    }
    fn set_mate_gen_seq(&mut self, new_mate_gen_seq: String) {
        self.mate_gen_seq = new_mate_gen_seq;
    }
}
