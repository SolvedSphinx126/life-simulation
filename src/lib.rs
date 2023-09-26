//use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//use crate::grazer::grazer::Grazer;
//use crate::plant::plant::Plant;
//use crate::rock::rock::Rock;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use js_sys::Array;

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
    predators: Vec<Predator>,
    grazers: Vec<Grazer>,
    plants: Vec<Plant>,
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
    
    pub fn get_rocks(&self) -> js_sys::Array {
        self.rocks.clone().into_iter().map(JsValue::from).collect::<js_sys::Array>()
    }
    pub fn get_grazers(&self) -> js_sys::Array {
        self.grazers.clone().into_iter().map(JsValue::from).collect::<js_sys::Array>()
    }
    pub fn get_plants(&self) -> js_sys::Array {
        self.plants.clone().into_iter().map(JsValue::from).collect::<js_sys::Array>()
    }
    pub fn get_predators(&self) -> js_sys::Array {
        self.predators.clone().into_iter().map(JsValue::from).collect::<js_sys::Array>()
    }
    pub fn add_rock(&mut self, id: u32, x: i32, y: i32, diameter: u32, height: u32) -> Rock{
        let new_rock = Rock::new(id, x, y, diameter, height);
        self.rocks.push(new_rock);
        new_rock
    }
    pub fn add_plant(&mut self, id: u32, x: i32, y: i32, diameter: u32){
        let new_plant: Plant = Plant::new(id, x, y, diameter);
        self.plants.push(new_plant);
    }
    pub fn add_grazer(&mut self,new_id: u32, new_x:i32, new_y: i32, new_state: i32, new_velocity_x: i32, new_velocity_y: i32, new_orientation: f32, new_target_x: i32,new_target_y: i32, new_energy: i32, new_min_in_loc: i32){
        let new_grazer = Grazer::new(new_id, new_x, new_y, new_state, new_velocity_x, new_velocity_y, new_orientation, new_target_x, new_target_y, new_energy, new_min_in_loc);
        self.grazers.push(new_grazer);
    }
    pub fn add_predator(&mut self, new_id: u32, new_x:i32, new_y: i32, new_state: i32, new_velocity_x: i32, new_velocity_y: i32, new_orientation: f32, new_target_x: i32,new_target_y: i32, new_energy: i32, new_min_in_loc: i32, new_gen_seq: String, new_family: Vec<i32>, new_time_family: f32, new_is_pregnant: bool, new_time_til_birth: u64, new_mate_gen_seq: String){
        let new_predator = Predator::new(new_id, new_x, new_y, new_state, new_velocity_x, new_velocity_y, new_orientation, new_target_x, new_target_y, new_energy, new_min_in_loc, new_gen_seq, new_family, new_time_family, new_is_pregnant, new_time_til_birth, new_mate_gen_seq);
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
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Entity {
    id: u32,
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Entity {
    fn new(new_id: u32, new_x: i32, new_y: i32) -> Entity {
        Entity {
            id: new_id,
            x: new_x,
            y: new_y,
        }
    }
    fn get_id(&self) ->u32 {
        self.id
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
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

#[derive(Clone, Copy)]
#[wasm_bindgen]
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

#[wasm_bindgen]
impl Mover {
    fn new(new_id: u32, new_x:i32, new_y: i32, new_state: i32, new_velocity_x: i32, new_velocity_y: i32, new_orientation: f32, new_target_x: i32,new_target_y: i32, new_energy: i32) -> Mover {
        Mover { entity: Entity::new(new_id, new_x, new_y), state: new_state, velocity_x: new_velocity_x, velocity_y: new_velocity_y, orientation: new_orientation, target_x: new_target_x, target_y: new_target_y, energy: new_energy }
    }
    fn get_state(&self) -> i32 {
        //change to enum in future
        self.state
    }
    fn get_velocity_x(&self) -> i32 {
        self.velocity_x
    }
    fn get_velocity_y(self) -> i32 {
        self.velocity_y
    }
    fn get_orientation(&self) -> f32 {
        self.orientation
    }
    fn get_target_x(&self) -> i32 {
        self.target_x
    }
    fn get_target_y(&self) -> i32 {
        self.target_y
    }
    fn get_energy(&self) -> i32 {
        self.energy
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
fn get_length(x: f64, z: f64) -> f64
{
	return f64::sqrt((x*x) + (z*z));
}


fn normalize(mut x: f64, mut z: f64) -> (f64, f64)
{
	let distance = get_length(x, z);
	x = x / distance;
	z = z / distance;
	return (x, z);
}


fn seek (mut char: &mut Mover, target: Entity, delta_time: f64) -> &Mover
{
	let mut result_x = 0.0;
	let mut result_y = 0.0;

	result_x = target.x - char.entity.x;
	result_y = target.y - char.entity.y;

	(result_x, result_y) = normalize(result_x, result_y);
	result_x = result_x * char.max_accel;
	result_y = result_y * char.max_accel;
	
	return update(result_x, result_y, char, delta_time);
}


fn flee (mut char: &mut Mover, target: Entity, delta_time: f64) -> &Mover
{
	let mut result_x = 0.0;
	let mut result_y = 0.0;

	result_x = char.entity.x - target.x;
	result_y = char.entity.y - target.y;

	(result_x, result_y) = normalize(result_x, result_y);
	result_x = result_x * char.max_accel;
	result_y = result_y * char.max_accel;
	
	return update(result_x, result_y, char, delta_time);
}


fn arrive (mut char: &mut Mover, target: Entity, delta_time: f64) -> &Mover
{
	let mut result_x = 0.0;
	let mut result_y = 0.0;
	let mut goalSpeed = 0.0;
	let mut direction_x = 0.0;
	let mut direction_y = 0.0;

	direction_x = target.x - char.entity.x;
	direction_y = target.y - char.entity.y;

	let distance = get_length(direction_x, direction_y);

	if distance < 1.0 //This may need to be tested and fixed later.
	{
		return char;
	}

	if distance > 1.5 //This will also need to be tested. 	
	{
		goalSpeed = char.velocity;
	}

	else
	{
		goalSpeed = char.velocity * distance / 1.5 //The 1.5 is the slow radius, this needs to be tested.
	}

	let mut goal_velocity_x = direction_x;
	let mut goal_velocity_y = direction_y;
	
	(goal_velocity_x, goal_velocity_y) = normalize(goal_velocity_x, goal_velocity_y);
	goal_velocity_x = goal_velocity_x * goalSpeed;
	goal_velocity_y = goal_velocity_y * goalSpeed;

	result_x = goal_velocity_x - char.entity.x;
	result_y = goal_velocity_y - char.entity.y;
	
	/* These 2 lines might not be necessary, maybe need testing? maybe not?
	result_x = result_x / char.ttt; 
	result_y = result_y / char.ttt;	
	*/

	return update(result_x, result_y, char, delta_time);
}


fn wander(mut char: &mut Mover, delta_time: f64) ->&Mover
{
	let maxRotation = 15.0;
	let mut result_x = 0.0;
	let mut result_y = 0.0;
	let mut result_orien = 0.0;
	let num = rand::thread_rng().gen_range(-1.0..1.0);

	result_x = char.velocity * char.orientation.sin();
	result_y = char.velocity * char.orientation.cos();
	result_orien = num * maxRotation;
	
	return kinematicupdate(result_x, result_y, result_orien, char, delta_time);
}

fn update(result_x: f64, result_y: f64, char: &mut Mover, delta_time: f64) -> &Mover
{
	char.entity.x += char.velocity_x * delta_time;
	char.entity.y += char.velocity_y * delta_time;
    

	char.velocity_x += result_x * delta_time;
	char.velocity_y += result_y * delta_time;

    


	if get_length(char.velocity_x, char.velocity_y) > char.velocity
	{
		let (velocity_x, velocity_y) = normalize(char.velocity_x, char.velocity_y);
		char.velocity_x = velocity_x * char.velocity;
		char.velocity_y = velocity_y * char.velocity;
	}
    

	return char;
}

fn kinematicupdate(result_x: f64, result_y: f64, result_orien: f64, char: &mut Mover, delta_time: f64) -> &Mover
{
	char.entity.x += char.velocity_x * delta_time;
	char.entity.y += char.velocity_y * delta_time;
	char.orientation += char.rotation * delta_time;

	
	char.velocity_x += result_x * delta_time;
	char.velocity_y += result_y * delta_time;
	char.rotation += result_orien * delta_time;

	if get_length(char.velocity_x, char.velocity_y) > char.velocity
	{
		let (velocity_x, velocity_y) = normalize(char.velocity_x, char.velocity_y);
		char.velocity_x = velocity_x * char.velocity;
		char.velocity_y = velocity_y * char.velocity;
	}

	return char;
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
    fn new(new_id: u32, new_x: i32, new_y: i32, new_diameter: u32, new_height: u32) -> Rock {
        Rock { entity: Entity::new(new_id, new_x, new_y), diameter: new_diameter, height: new_height }
    }

    pub fn get_x(&self) -> i32 {
        self.entity.get_x()
    }
    pub fn get_y(&self) -> i32 {
        self.entity.get_y()
    }
    pub fn get_diameter(&self) -> u32 {
        self.diameter
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    fn set_diameter(&mut self, new_diameter: u32) {
        self.diameter = new_diameter;
    }
    fn set_height(&mut self, new_height: u32) {
        self.height = new_height;
    }
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Grazer {
    mover: Mover,
    ticks_in_loc: i32, //minutes in cur location without moving max is 10 once at 10 need to move
}

#[wasm_bindgen]
impl Grazer {
    fn new(new_id: u32, new_x:i32, new_y: i32, new_state: i32, new_velocity_x: i32, new_velocity_y: i32, new_orientation: f32, new_target_x: i32,new_target_y: i32, new_energy: i32, new_min_in_loc: i32) -> Grazer {
        Grazer { mover: Mover::new(new_id, new_x, new_y, new_state, new_velocity_x, new_velocity_y, new_orientation, new_target_x, new_target_y, new_energy), ticks_in_loc: new_min_in_loc }
    }
    pub fn get_ticks_in_loc(&self) -> i32 {
        self.ticks_in_loc
    }
    fn set_ticks_in_loc(&mut self, new_min_in_loc: i32) {
        self.ticks_in_loc = new_min_in_loc;
    }
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Plant {
    entity: Entity,
    diameter: u32,
}

#[wasm_bindgen]
impl Plant {
    fn new(new_id: u32, new_x: i32, new_y: i32, new_diameter: u32) -> Plant {
        Plant { entity: Entity::new(new_id, new_x, new_y), diameter: new_diameter}
    }
    pub fn get_diameter(&self) -> u32 {
        self.diameter
    }
    fn is_max_size(&mut self, map: &Map) -> bool {
        self.diameter >= map.get_max_size()
    }
    fn set_diameter(&mut self, new_diameter: u32) {
        self.diameter = new_diameter;
    }
    //need actual seeding functions
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Predator {
    mover: Mover,
    gen_seq: String,
    family: Vec<i32>, //vector of family ids
    time_family: f32, // time after mating that predator cares about family
    is_pregnant: bool,
    ticks_til_birth: u64,  // the first tick where the gestation period is over
    mate_gen_seq: String, // mates gennetic sequence
}

#[wasm_bindgen]
impl Predator {
    fn new(new_id: u32, new_x:i32, new_y: i32, new_state: i32, new_velocity_x: i32, new_velocity_y: i32, new_orientation: f32, new_target_x: i32,new_target_y: i32, new_energy: i32, new_min_in_loc: i32, new_gen_seq: String, new_family: Vec<i32>, new_time_family: f32, new_is_pregnant: bool, new_time_til_birth: u64, new_mate_gen_seq: String) -> Predator {
        Predator { mover: Mover::new(new_id, new_x, new_y, new_state, new_velocity_x, new_velocity_y, new_orientation, new_target_x, new_target_y, new_energy), gen_seq: new_gen_seq, family: new_family, time_family: new_time_family, is_pregnant: new_is_pregnant, ticks_til_birth: new_time_til_birth, mate_gen_seq: new_mate_gen_seq }
    }
    pub fn get_gen_seq(&self) ->  String {
        self.gen_seq.clone()
    }
    pub fn get_family(&self) -> Vec<i32> {
        self.family.clone()
    }
    pub fn get_time_family(&self) -> f32 {
        self.time_family
    }
    pub fn get_is_pregnant(&self) -> bool {
        self.is_pregnant
    }
    pub fn get_ticks_til_birth(&self) -> u64 {
        self.ticks_til_birth
    }
    pub fn get_mate_seq(&self) -> String {
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
    fn set_time_family(&mut self, new_time_family: f32) {
        self.time_family = new_time_family;
    }
    fn set_is_pregnant(&mut self, is_pregnant: bool) {
        self.is_pregnant = is_pregnant;
    }
    fn set_ticks_til_birth(&mut self, map: Map, new_time_til_birth: u64) {
        self.ticks_til_birth = new_time_til_birth + Map::get_current_tick(&map);
    }
    fn set_mate_gen_seq(&mut self, new_mate_gen_seq: String) {
        self.mate_gen_seq = new_mate_gen_seq;
    }
}
