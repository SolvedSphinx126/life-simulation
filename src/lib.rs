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
        //log("in map.tick");
        log(format!("current tick is {}", self.current_tick).as_str());
        let mut new_grazers = Vec::new();
        // let mut new_predators = Vec::new();
        let mut new_plants = Vec::new();
        let mut plants_to_remove = Vec::new();
        let maintain_speed_ticks = (self.grazer_maintain_speed * 60.0) as i32;
        let p_maintain_speed = (self.predator_maintain_speed * 60.0) as i32;
        let mut preds = vec![];
        let mut ded_preds = vec![];
        let mut ded_grazs = vec![];

        for  plant in self.plants.iter() {
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


        }
        //log("ticked all plants");
        for grazer in self.grazers.iter() {
            //new_grazers.append
            log(format!("current pos {}, {}", grazer.mover.entity.x, grazer.mover.entity.y).as_str());
            let mut weird = grazer.clone().tick(self.get_grazer_energy_input(),
            self.get_grazer_energy_output(),
            self.get_grazer_energy_to_reproduce(),
            self.get_grazer_max_speed()/60.0,
            maintain_speed_ticks,
            self.get_plants_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 1.0),
            self.get_plants_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 150.0),
            self.get_predators_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 25.0),
            self.get_rocks_within_vicinity(grazer.mover.entity.x, grazer.mover.entity.y, 150.0),
            self.get_current_tick(),
            self.width,
            self.height
            );
             new_grazers.append(&mut weird.0); 
            plants_to_remove.append(&mut weird.1);
        }
        //log("ticked all grazers");

        for pred in self.predators.iter() {
            let mut weird = pred.clone().tick(
                self.predator_energy_to_reproduce,
                self.current_tick,
                self.predator_energy_output,
                self.get_predators_within_vicinity(
                    pred.mover.get_entity().get_x(),
                    pred.mover.get_entity().get_y(),
                    5.0,
                ),
                self.get_predators_within_vicinity(
                    pred.mover.get_entity().get_x(),
                    pred.mover.get_entity().get_y(),
                    5.0,
                ),
                self.get_rocks_within_vicinity(pred.mover.entity.x,pred.mover.entity.y, 150.0),
                self.get_grazers_within_vicinity(pred.mover.entity.x, pred.mover.entity.y, 150.0),
                self.predator_max_offspring,
                self.predator_offspring_energy,
                self.get_predator_gestation(),
                self.get_predator_by_id(pred.family.get(0)),
                self.max_speed_hod /60.0,
                self.max_speed_hed /60.0,
                self.max_speed_hor /60.0,
                p_maintain_speed,
                self.width,
                self.height
            );
            preds.append(&mut weird.0);
            ded_preds.append(&mut weird.1);
            ded_grazs.append(&mut weird.2);

        }
        //log("ticked all predators");

        //set all alive creatures
        self.predators = preds;
        self.grazers = new_grazers;
        self.plants = new_plants;

        //increment tick counter
        self.current_tick += 1;
        
        
        // remove dead
        self.plants.retain(|obj| {
            !plants_to_remove.iter().any(|r| r.entity.id == obj.entity.id) // Change the condition based on your specific criteria
        });
        self.grazers.retain(|obj| {
            !ded_grazs.iter().any(|r| r.get_entity().id == obj.get_entity().id) // Change the condition based on your specific criteria
        });
        self.predators.retain(|obj| {
            !ded_preds.iter().any(|r| r.get_entity().id == obj.get_entity().id) // Change the condition based on your specific criteria
        });

        log(format!("length of plants{}", self.plants.len()).as_str());
        log(format!("length of grazers{}", self.grazers.len()).as_str());
        log(format!("length of predators{}", self.predators.len()).as_str());

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
    fn tick(&mut self, max_speed: f32, energy: u32, target: Entity, rocks: Vec<Rock>, width: u32, height: u32){

        
            // move here
            if self.state == 0{
                //stay
                //return self back
                // dont need to set anything to zero as will return a new copy of mover up to
            }
            else if self.state == 1 {
                Mover::arrive(self, target, rocks, 1 as f32, width, height);
            }
            else if self.state == 2 {
                Mover::wander(self, 1 as f32, rocks, width, height);
            }
            else if self.state == 3 {
                Mover::flee(self, target, 1 as f32, rocks, width, height);
            }

            //does this acurately measure distance traveled? the grazers distance able to travel changes after a speed change
            self.du += f32::sqrt(self.velocity_x.powi(2) + self.velocity_y.powi(2));
            if self.du > 6.0 {
                self.du -= 5.0;
                self.energy -= energy;
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
        let distance = get_length(x, z); //282 -200
        x = x / distance;
        z = z / distance;
        return (x, z); // -0.709
    }

    fn evade (mut char: &mut Mover, rocks: Vec<Rock>, delta_time: f32, width: u32, height: u32) -> &Mover{
        let mut min_dist = 150.0;
        let mut closest_rock = Rock::default();
        for rock in rocks.iter(){
            
                let distance = ((rock.entity.x - char.entity.x).powi(2) + (rock.entity.y - char.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_rock = rock.clone();
                    }
                }

    let mut new_target_x = char.entity.x - closest_rock.entity.x;
    let mut new_target_y = char.entity.y - closest_rock.entity.y;

    (new_target_x, new_target_y) = Mover::normalize(new_target_x, new_target_y);

    if new_target_x < 0.0
    {
        new_target_x = closest_rock.entity.x + new_target_x + ((closest_rock.diameter as f32 / 2.0) * -1.0);
        new_target_y = closest_rock.entity.y + new_target_y + (closest_rock.diameter as f32 / 2.0);
    }

    else if new_target_y < 0.0
    {
        new_target_y = closest_rock.entity.y + new_target_y + ((closest_rock.diameter as f32 / 2.0) * -1.0);
        new_target_x = closest_rock.entity.x + new_target_x + (closest_rock.diameter as f32 / 2.0);
    }

    else if new_target_y < 0.0 && new_target_x < 0.0
    {
        new_target_y = closest_rock.entity.y + new_target_y + ((closest_rock.diameter as f32 / 2.0) * -1.0);
        new_target_x = closest_rock.entity.x + new_target_x + ((closest_rock.diameter as f32 / 2.0) * -1.0);
    }

    else
    {
        new_target_x = closest_rock.entity.x + new_target_x + (closest_rock.diameter as f32 / 2.0);
        new_target_y = closest_rock.entity.y + new_target_y + (closest_rock.diameter as f32 / 2.0);
    }

    let temp = Entity::new(new_target_x, new_target_y);
    return Mover::arrive(char, temp, rocks, delta_time, width, height);

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

    fn flee(mut char: &mut Mover, target: Entity, delta_time: f32, rocks: Vec<Rock>, width: u32, height: u32) -> &Mover {
        let dx = target.x - char.entity.x;
        let dy = target.y - char.entity.y;
        char.orientation = dy.atan2(dx);

        let mut result_x = 0.0;
        let mut result_y = 0.0;

        result_x = char.entity.x - target.x;
        result_y = char.entity.y - target.y;

        if !rocks.is_empty(){
            (result_x, result_y) = Mover::avoid(result_x, result_y, char, rocks, delta_time, width, height);
        }
        (result_x, result_y) = Mover::normalize(result_x, result_y);
        result_x = result_x * char.max_speed;
        result_y = result_y * char.max_speed;

        return Mover::update(result_x, result_y, char, delta_time);
    }

    fn arrive(mut char: &mut Mover, target: Entity,  rocks: Vec<Rock>, delta_time: f32, width: u32, height: u32 ) -> &Mover {
        
        let dx = target.x - char.entity.x;
        let dy = target.y - char.entity.y;
        char.orientation = dy.atan2(dx);

        let mut result_x = 0.0;
        let mut result_y = 0.0;
        let mut goalSpeed = 0.0;
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        direction_x = target.x - char.entity.x; // -200
        direction_y = target.y - char.entity.y; // -200

        let distance = get_length(direction_x, direction_y); //282
        if distance < 1.0
        //This may need to be tested and fixed later.
        {
            return char;
        }

        else if distance > char.max_speed / 2.0
        //This will also need to be tested. // max sped
        {
            //log("arrive at max");
            goalSpeed = char.max_speed; // 20

        } else { //slow
            //log("arrive at slow");
            goalSpeed = char.max_speed * distance / (char.max_speed + 5.0) //The 1.5 is the slow radius, this needs to be tested.
        }

        let mut goal_velocity_x = direction_x; // -200
        let mut goal_velocity_y = direction_y; // -200

        
        

        (goal_velocity_x, goal_velocity_y) = Mover::normalize(goal_velocity_x, goal_velocity_y); // -0.709
        goal_velocity_x = goal_velocity_x * goalSpeed; // -14
        goal_velocity_y = goal_velocity_y * goalSpeed; // -14

        log(format!("velocity ({},{})", goal_velocity_x, goal_velocity_y).as_str());

        result_x = goal_velocity_x;
        result_y = goal_velocity_y;

        if !rocks.is_empty(){
            log("trying to avoid rock");
            (result_x, result_y) = Mover::avoid(result_x, result_y, char, rocks, delta_time, width, height);

            let tester = get_length(result_x, result_y);
            if tester > char.max_speed{
                (result_x, result_y) = Mover::normalize(result_x, result_y);
                result_x = result_x * char.max_speed;
                result_y = result_y * char.max_speed;
            }
  
        }



        /* These 2 lines might not be necessary, maybe need testing? maybe not?
        result_x = result_x / char.ttt;
        result_y = result_y / char.ttt;
        */

        return Mover::update(result_x, result_y, char, delta_time );
    }

    fn wander(mut char: &mut Mover, delta_time: f32, rocks: Vec<Rock>, width: u32, height: u32) -> &Mover {
        let max_rotation = 0.25;
        let mut result_x = 0.0;
        let mut result_y = 0.0;
        let mut result_orien = 0.0;
        let num = rand::thread_rng().gen_range(-1.0..1.0);

       // log("max speed is ", char.max_speed);
       log(format!("WANDER max_speed {}, orientation {}, num {}", char.max_speed, char.orientation, num).as_str());

        result_y = char.max_speed * char.orientation.sin();
        result_x = char.max_speed * char.orientation.cos();
        result_orien = num * max_rotation;

        log(format!("WANDER result x {}, result y {}, delta time{}", result_x, result_y, delta_time).as_str());

        if !rocks.is_empty(){
            log("trying to avoid rock");
            (result_x, result_y) = Mover::avoid(result_x, result_y, char, rocks, delta_time, width, height);
            
        }

        return Mover::kinematicupdate(result_x, result_y, result_orien, char, delta_time);
    }

    fn avoid(mut result_x: f32, mut result_y: f32, char: &mut Mover, rocks: Vec<Rock>, delta_time: f32, width: u32, height: u32) -> (f32, f32){
        
        let mut interim_dis_x = 0.0 as f32;
        let mut interim_dis_y = 0.0 as f32;

        let mut min_dist = 150 as f32;
        let mut closest_rock = &rocks[0];
        let mut radius = 0.0;

        for rock in rocks.iter(){
                let distance = ((rock.entity.x - char.entity.x).powi(2) + (rock.entity.y - char.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_rock = rock;
                    radius = rock.diameter as f32 / 2.0;
                    }
                }

        if min_dist <= radius + 8.0 && min_dist > radius + 2.0
        {
            log("close but not too close");
            interim_dis_x = char.entity.x - closest_rock.entity.x;
            interim_dis_y = char.entity.y - closest_rock.entity.y;

            let test1 = result_x * 1.2;
            let test2 = result_y * 1.2;

            let test3 = interim_dis_x * 0.85 ;
            let test4 = interim_dis_y * 0.85;

            result_x = test1 + test3;
            result_y = test2 + test4;

            (result_x, result_y) = Mover::normalize(result_x, result_y);
            result_x = result_x * char.max_speed;
            result_y = result_y * char.max_speed;

            return (result_x, result_y);

        }
    else if min_dist <= radius + 2.0
        {
            log("way too close");
            interim_dis_x = char.entity.x - closest_rock.entity.x;
            interim_dis_y = char.entity.y - closest_rock.entity.y;

            let test1 = result_x * 0.85;
            let test2 = result_y * 0.85;

            let test3 = interim_dis_x * 2.0;
            let test4 = interim_dis_y * 2.0;

            result_x = test1 + test3;
            result_y = test2 + test4;

            (result_x, result_y) = Mover::normalize(result_x, result_y);
            result_x = result_x * char.max_speed;
            result_y = result_y * char.max_speed;

            return (result_x, result_y);
        }

        if (char.entity.x + char.velocity_x) >= width as f32 && (char.entity.x + char.velocity_x) <= 0.0 {
            log("close to wall side");
            // the result line might not be necessary
            char.velocity_x = char.velocity_x * -1.0;
            result_x = result_x * -1.0;
        }

        if (char.entity.y + char.velocity_y) >= height as f32  && (char.entity.y + char.velocity_y) <= 0.0 {
            log("close to wall top");
            // the result line might not be necessary
            char.velocity_y = char.velocity_y * -1.0;
            result_y = result_y * -1.0;
        }

        return (result_x, result_y);
    }

    fn update(result_x: f32, result_y: f32, char: &mut Mover, delta_time: f32) -> &Mover {
        char.velocity_x += result_x * delta_time; //-264
        char.velocity_y += result_y * delta_time; // -264

        if get_length(char.velocity_x, char.velocity_y) > char.max_speed {
            let (velocity_x, velocity_y) = Mover::normalize(char.velocity_x, char.velocity_y);
            char.velocity_x = velocity_x * char.max_speed; // -14
            char.velocity_y = velocity_y * char.max_speed; // -14
        }
        
        char.entity.x += char.velocity_x * delta_time; 
        char.entity.y += char.velocity_y * delta_time;



        return char;
    }

    fn kinematicupdate(
        result_x: f32,
        result_y: f32,
        result_orien: f32,
        char: &mut Mover,
        delta_time: f32,
    ) -> &Mover {
        log(format!("result x {}, result y {}, delta time{}", result_x, result_y, delta_time).as_str());
        char.entity.x += char.velocity_x * delta_time;
        char.entity.y += char.velocity_y * delta_time;
        //char.orientation += char.orientation * delta_time;

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
            max_speed: 15.0
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

impl Default for Rock {
    fn default() -> Self {
        Rock {
            entity: Entity::default(),
            diameter: 0,
            height: 0,
        }
    }
}

#[derive(Clone, Copy, Default)]
#[wasm_bindgen]
pub struct Grazer {
    mover: Mover,
    ticks_in_loc: i32, //minutes in cur location without moving max is 10 once at 10 need to move
    ticks_at_speed: i32,
    speed_cooldown: i32,
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
        width: u32,
        height: u32
    ) -> (Vec<Grazer>, Vec<Plant>) {
        //plants and predator is a vector of creatures with distance sight is not yet implemented 
        //to integrate sight just change the function called when tick is called in map.
        let mut new_graz = Vec::new();
        let mut ded_plants = Vec::new();
        log(format!("energy {}", self.mover.energy).as_str());
        if self.mover.energy > 25 && self.ticks_at_speed < maintain_speed{
            self.mover.max_speed = max_speed;
        }
        else if self.mover.energy <= 25 {
            self.mover.max_speed = 10.0;
        }
        else if self.ticks_at_speed > maintain_speed && self.speed_cooldown == 0 
        {
            self.mover.max_speed = max_speed * 0.75;
            self.speed_cooldown += 1;
        }
        else {
            self.speed_cooldown += 1;
        }
        if self.speed_cooldown == 10{
            self.ticks_at_speed = 0;
            self.speed_cooldown = 0;
        }
        
        
        //first check for predators to run from
        if !predators.is_empty(){
            //log("pred not emp");
            //seek rock away from closest pred
            //set movers target
            self.mover.state = 1; //set state to arrive
            //log(plants.len());
            if !rocks.is_empty(){
            let mut min_dist = 150 as f32;
            let mut closest_rock = &rocks[0];
            for rock in rocks.iter(){
                let distance = ((rock.entity.x - self.mover.entity.x).powi(2) + (rock.entity.y - self.mover.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_rock = rock;
                    }
                }
                
                if min_dist > 0.0 {
                    self.mover.tick(max_speed, energy_out, closest_rock.entity, rocks, width, height);
                    self.ticks_at_speed += 1;
                }
            
            }
            else {
                
                let mut min_dist = 150 as f32;
                let mut closest_pred = &predators[0];
                for pred in predators.iter(){
                    let distance = ((pred.mover.entity.x - self.mover.entity.x).powi(2) + (pred.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                    if distance < min_dist {
                        min_dist = distance;
                        closest_pred = pred;
                        }
                    }
                    self.mover.state = 3;
                    self.mover.tick(max_speed, energy_out, closest_pred.mover.entity, rocks, width, height);
                    self.ticks_at_speed += 1;
             }
        }
        else if self.mover.energy >= energy_reproduce {
            //log("reproduce");
            new_graz.push(self.reproduce());
        }
        // here means no predators
        // check if at food for plant in 5 du
        // been at plant
        else if !at_plants.is_empty() && self.ticks_in_loc != 0{

            //now check if tick at loc is at max
            self.mover.state = 0;
            log(format!("tick in loc {}", self.ticks_in_loc).as_str());
            log(format!("energy {}", self.mover.energy).as_str());
            if self.ticks_in_loc == 600{
                for plant in at_plants.iter(){
                    ded_plants.push(plant.clone());
                }
                
                self.ticks_in_loc = 0;
                //seek next plant   
            }
            else {
                
                //then if not max stay
                //gain energy on 60 increments
                self.ticks_in_loc += 1;
                if self.ticks_in_loc % 60 == 0 {
                    self.mover.energy += energy_in;
                }
            }
        }
        // first tick at plant
        else if !at_plants.is_empty() && self.ticks_in_loc == 0{
            // just arrived at plant
            self.mover.state = 0;
            self.ticks_in_loc += 1;
            self.mover.tick(max_speed, energy_out, self.mover.entity, rocks,  width, height);
        }

    
        else if at_plants.is_empty() && !plants.is_empty(){
            //find closest plant and set arrive target
            self.mover.state = 1;
            let mut min_dist = 150 as f32;
            let mut closest_plant = &plants[0];
            for plant in plants.iter(){
                let distance = ((plant.entity.x - self.mover.entity.x).powi(2) + (plant.entity.y - self.mover.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_plant = plant;
                    }
                }
            self.mover.tick(max_speed, energy_out, closest_plant.entity, rocks, width, height);
        }
        else{
            //start wandering
            log("wander");
            self.mover.state = 2;
            self.mover.tick(max_speed, energy_out, self.mover.entity, rocks,  width, height);
        }
        
        //log(format!("energy {}", self.mover.energy).as_str());
        // only add grazers worthy of life
        if self.mover.energy >= 0 as u32{
            new_graz.push(self.clone());
        }
        
        
        return (new_graz, ded_plants);
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
    ticks_at_speed: i32,
    cooldown_speed: i32,
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
        mates: Vec<Predator>,
        preds: Vec<Predator>,
        rocks: Vec<Rock>,
        grazers: Vec<Grazer>,
        max_offspring: u32,
        offspring_energy: u32,
        gestation: u64,
        partner: Option<&Predator>,
        max_speed_hod: f32,
        max_speed_hed: f32,
        max_speed_hor: f32,
        predator_maintain_speed: i32,
        width: u32,
        height: u32
    ) -> (Vec<Predator>, Vec<Predator>, Vec<Grazer>) {
        let mut ret = vec![];
        let mut max_speed = 0.0;
        let mut ded_grazs = Vec::new();
        let mut ded_preds = Vec::new();
        let mut predators = preds.clone();
        predators.retain(|obj|  self.get_entity().get_id() != obj.get_entity().get_id());
        //(|pred| pred.get_entity().get_id() != self.get_entity().get_id());
        // if energy and not pregnant
        // has a mate
        // mate
        // perform birth() for both parents
        // add birthed predators to ret
        // set is_pregnant
        // set gestation
        // add mate to avoid list

        // need to filter for avoid list

        // if can mate seek mate
        // else if seek food (within check about genetcs and type of prey)
        // with in this check if at prey then using genetcs calulate chance fo catch and kill and gain energy
        // else wander
        match self.speed {
            Gene::HomoDominant => {
                self.mover.max_speed = max_speed_hod;
            }
            Gene::Hetero =>
            {
                self.mover.max_speed = max_speed_hed;
            }
            Gene::HomoRecessive => {
                self.mover.max_speed = max_speed_hor;
            }
        }
        if self.ticks_at_speed > predator_maintain_speed {
            if self.ticks_at_speed % 15 == 0 {
                self.mover.max_speed = self.mover.max_speed - 1.0;
            }
            if self.mover.max_speed <= 0.0 {
                self.cooldown_speed += 1;
            }
        }
        if self.cooldown_speed == 10 {
            self.ticks_at_speed = 0;
            self.cooldown_speed = 0;
        }

        if self.is_pregnant {
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
        

        else if self.willing_to_mate(energy_to_reproduce) {
            // if vaible candidate is found
            let pred = mates
            .iter()
            .filter(|p| p.willing_to_mate(energy_to_reproduce))
            .filter(|p| p.get_entity().get_id() != self.get_entity().get_id())
            //.inspect(|pred| log(pred.get_entity().get_id().to_string().as_str()))
            .next();

            if let Some(pred) = pred {
                self.mate(&mut pred.clone(), cur_tick, gestation);
                log("viable mate found");
            }
        } 
        
        if !predators.is_empty(){
            self.ticks_at_speed += 1;
            log("predators not empty");
            // if preds not empty 
            //check if at a valid prey before seeking
            //AA seek closest prey source wether pred or graz
            //Aa seek pred only IF graz.is_empty()
            //aa runaway from pred unless mating
            // push dead preds to ded_preds
            match self.agression {

                Gene::HomoDominant => {

                    let mut p_min_dist = 150 as f32;
                    let mut closest_pred: Predator = Predator::default();
                    for pred in predators.iter().filter(|pred| !self.family.contains(&pred.get_entity().get_id())){
                        let distance = ((pred.mover.entity.x - self.mover.entity.x).powi(2) + (pred.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                        if distance < p_min_dist {
                            p_min_dist = distance;
                            closest_pred = pred.clone();
                            }
                    }
                    if !grazers.is_empty() {
                        let mut g_min_dist = 150 as f32;
                        let mut closest_graz = Grazer::default();
                        for graz in grazers.iter(){
                            let distance = ((graz.mover.entity.x - self.mover.entity.x).powi(2) + (graz.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                            if distance < g_min_dist {
                                g_min_dist = distance;
                                closest_graz = graz.clone();
                                }
                        }
                        //if graz closer go for graz
                        if g_min_dist < p_min_dist {
                            if g_min_dist <= 5.0 {
                                //hunt math
                                let hunt_chance :u8 = rand::thread_rng().gen_range(0..100);
                                match self.strength {
                                    Gene::HomoDominant => {
                                        if hunt_chance <= 95 as u8{
                                            ded_grazs.push(closest_graz);
                                            self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                                        }                                           
                                    }
                                    Gene::Hetero => {
                                        if hunt_chance <= 75 as u8{
                                            //kill grazer gain energy
                                            ded_grazs.push(closest_graz);
                                            self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                                        }
                                    }
                                    Gene::HomoRecessive => {
                                        if hunt_chance <= 50 as u8{
                                            //kill grazer gain energy
                                            ded_grazs.push(closest_graz);
                                            self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                                        }
                                    }
                                }
                            }
                            else {
                                self.mover.state = 1;
                                self.mover.tick(max_speed, energy, closest_graz.mover.entity, rocks,  width, height);
                            }

                        }
                    
                    }
                    else {
                        // hunt the closest pred
                        if p_min_dist <= 5.0 {
                            //hunt math //need to update for pred v pred 
                            let hunt_chance :u8 = rand::thread_rng().gen_range(0..100);
                            match self.strength {
                                Gene::HomoDominant => {
                                    match closest_pred.strength {
                                        Gene::HomoDominant => {
                                            if hunt_chance <= 50 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::Hetero => {
                                            if hunt_chance <= 75 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::HomoRecessive => {
                                            if hunt_chance <= 95 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                    }
                                                                               
                                }
                                Gene::Hetero => {
                                    match closest_pred.strength {
                                        Gene::HomoDominant => {
                                            if hunt_chance <= 25 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::Hetero => {
                                            if hunt_chance <= 50 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::HomoRecessive => {
                                            if hunt_chance <= 75 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                    }
                                    
                                }
                                Gene::HomoRecessive => {
                                    match closest_pred.strength {
                                        Gene::HomoDominant => {
                                            if hunt_chance <= 5 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                        }
                                        Gene::Hetero => {
                                            if hunt_chance <= 25 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                        }
                                        Gene::HomoRecessive => {
                                            if hunt_chance <= 50 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            //seek closest prey
                            self.mover.state = 1;
                            self.mover.tick(max_speed, energy, closest_pred.mover.entity, rocks, width, height);
                        }
                    }
                    

                }
                Gene::Hetero => {
                    let mut p_min_dist = 150 as f32;
                    let mut closest_pred: Predator = Predator::default();
                    for pred in predators.iter().filter(|pred| !self.family.contains(&pred.get_entity().get_id()) ){
                        let distance = ((pred.mover.entity.x - self.mover.entity.x).powi(2) + (pred.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                        if distance < p_min_dist {
                            p_min_dist = distance;
                            closest_pred = pred.clone();
                            }
                    }
                    if !grazers.is_empty() {
                        log("grazers not empty");
                        let mut g_min_dist = 150 as f32;
                        let mut closest_graz = Grazer::default();
                        for graz in grazers.iter(){
                            let distance = ((graz.mover.entity.x - self.mover.entity.x).powi(2) + (graz.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                            if distance < g_min_dist {
                                g_min_dist = distance;
                                closest_graz = graz.clone();
                            }
                        }
                        if g_min_dist <= 5.0 {
                            //hunt math
                            let hunt_chance :u8 = rand::thread_rng().gen_range(0..100);
                            match self.strength {
                                Gene::HomoDominant => {
                                    if hunt_chance <= 95 as u8{
                                        ded_grazs.push(closest_graz);
                                        self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                                    }                                           
                                }
                                Gene::Hetero => {
                                    if hunt_chance <= 75 as u8{
                                        //kill grazer gain energy
                                        ded_grazs.push(closest_graz);
                                        self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                                    }
                                }
                                Gene::HomoRecessive => {
                                    if hunt_chance <= 50 as u8{
                                        //kill grazer gain energy
                                        ded_grazs.push(closest_graz);
                                        self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                                    }
                                }
                            }
                        }
                        else {
                            self.mover.state = 1;
                            self.mover.tick(max_speed, energy, closest_graz.mover.entity, rocks,  width, height);
                        }
                    }
                    else {
                        // hunt the closest pred
                        if p_min_dist <= 5.0 {
                            //hunt math //need to update for pred v pred 
                            let hunt_chance :u8 = rand::thread_rng().gen_range(0..100);
                            match self.strength {
                                Gene::HomoDominant => {
                                    match closest_pred.strength {
                                        Gene::HomoDominant => {
                                            if hunt_chance <= 50 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::Hetero => {
                                            if hunt_chance <= 75 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::HomoRecessive => {
                                            if hunt_chance <= 95 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                    }
                                                                               
                                }
                                Gene::Hetero => {
                                    match closest_pred.strength {
                                        Gene::HomoDominant => {
                                            if hunt_chance <= 25 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::Hetero => {
                                            if hunt_chance <= 50 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                        Gene::HomoRecessive => {
                                            if hunt_chance <= 75 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                            else {
                                                self.add_family(closest_pred.get_entity().get_id());
                                                closest_pred.add_family(self.get_entity().get_id());
                                            }
                                        }
                                    }
                                    
                                }
                                Gene::HomoRecessive => {
                                    match closest_pred.strength {
                                        Gene::HomoDominant => {
                                            if hunt_chance <= 5 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                        }
                                        Gene::Hetero => {
                                            if hunt_chance <= 25 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                        }
                                        Gene::HomoRecessive => {
                                            if hunt_chance <= 50 as u8{
                                                self.mover.energy += (f64::from(closest_pred.mover.energy) * 0.9) as u32;
                                                ded_preds.push(closest_pred);
                                                
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            //seek closest prey
                            self.mover.state = 1;
                            self.mover.tick(max_speed, energy, closest_pred.mover.entity, rocks, width, height);
                        }
                    }
                }
                Gene::HomoRecessive => {
                    let mut p_min_dist = 150.0 as f32;
                    let mut closest_pred: Predator = Predator::default();
                    for pred in predators.iter().filter(|pred| !self.family.contains(&pred.get_entity().get_id()) ){
                        let distance = ((pred.mover.entity.x - self.mover.entity.x).powi(2) + (pred.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                        if distance < p_min_dist {
                            p_min_dist = distance;
                            closest_pred = pred.clone();
                        }
                    }

                    self.mover.state = 3;
                    self.mover.tick(max_speed, energy, closest_pred.mover.entity,rocks, width, height);
                }
            }

            
        }
        else if !grazers.is_empty(){
            log("hunt graz");
            self.ticks_at_speed += 1;
            //if preds empty seek grazers no matter gene
            let mut min_dist = 150.0 as f32;
            let mut closest_graz = Grazer::default();
            for graz in grazers.iter(){
                let distance = ((graz.mover.entity.x - self.mover.entity.x).powi(2) + (graz.mover.entity.y - self.mover.entity.y).powi(2)).sqrt();
                if distance < min_dist {
                    min_dist = distance;
                    closest_graz = graz.clone();
                }
            }
            log(format!("min dist {}", min_dist).as_str());
            log(format!("graz speed {} vs pred speed {}", closest_graz.mover.max_speed, self.mover.max_speed).as_str());
            if min_dist <= 5.0 {
                //hunt math
                let hunt_chance :u8 = rand::thread_rng().gen_range(0..100);
                match self.strength {
                    Gene::HomoDominant => {
                        if hunt_chance <= 95 as u8{
                            ded_grazs.push(closest_graz);
                            self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                        }                                           
                    }
                    Gene::Hetero => {
                        if hunt_chance <= 75 as u8{
                            //kill grazer gain energy
                            ded_grazs.push(closest_graz);
                            self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                        }
                    }
                    Gene::HomoRecessive => {
                        if hunt_chance <= 50 as u8{
                            //kill grazer gain energy
                            ded_grazs.push(closest_graz);
                            self.mover.energy += (f64::from(closest_graz.mover.energy) * 0.9) as u32;
                        }
                    }
                }
            }
            else{
                //seek closest prey
                self.mover.state = 1;
                self.mover.tick(max_speed, energy, closest_graz.mover.entity, rocks,  width, height);
            }


        }
        
        else {
            log("wander");
            //not mating and no possible prey around
            self.mover.state = 2;
            
            self.mover.tick(max_speed, energy, self.mover.entity, rocks,  width, height);
            }
            
        if self.mover.energy > 0 {
            ret.push(self.clone());
        }
        
        return (ret, ded_preds, ded_grazs);
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
