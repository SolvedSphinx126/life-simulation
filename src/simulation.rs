pub mod simulation {

    struct Map {
        width: u32,
        height: u32,
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
    }

    impl Map{
        fn get_width(&self) -> &u32{
            &self.width
        }
        fn get_height(&self) -> &u32{
            &self.height
        }
        fn set_width(&mut self, new_width: u32) {
            self.width = new_width;
        }
        fn set_height(&mut self, new_height: u32) {
            self.height = new_height;
        }
        //plants
        fn get_init_plant_count(&self) -> &u32{
            &self.init_plant_count
        }
        fn get_growth_rate(&self) -> &f32{
            &self.growth_rate
        }
        fn get_max_size(&self) -> &u32{
            &self.max_size
        }
        fn get_max_seed_cast_distance(&self) -> &u32{
            &self.max_seed_cast_distance
        }
        fn get_max_seed_number(&self) -> &u32{
            &self.max_seed_number
        }
        fn get_seed_viability(&self) -> &f32{
            &self.seed_viability
        }
        fn set_init_plant_count(&mut self, new_init_plant_count: u32){
            self.init_plant_count = new_init_plant_count;
        }
        fn set_growth_rate(&mut self, new_growth_rate: f32){
            self.growth_rate = new_growth_rate;
        }
        fn set_max_size(&mut self, new_max_size: u32){
            self.max_size = new_max_size;
        }
        fn set_max_seed_cast_distance(&mut self, new_max_seed_cast_distance: u32){
            self.max_seed_cast_distance = new_max_seed_cast_distance;
        }
        fn set_max_seed_number(&mut self , new_max_seed_number: u32){
            self.max_seed_number = new_max_seed_number;
        }
        fn set_seed_viability(&mut self, new_seed_viability: f32){
            self.seed_viability = new_seed_viability;
        }
        //Grazers
        fn get_init_grazer_count(&self) -> &u32{
            &self.init_grazer_count
        }
        fn get_grazer_energy_input(&self) -> &u32{
            &self.grazer_energy_input
        }
        fn get_grazer_energy_output(&self) -> &u32{
            &self.grazer_energy_output
        }
        fn get_grazer_energy_to_reproduce(&self) -> &u32{
            &self.grazer_energy_to_reproduce
        }
        fn get_grazer_maintain_speed(&self) -> &f32{
            &self.grazer_maintain_speed
        }
        fn get_grazer_max_speed(&self) -> &f32{
            &self.grazer_max_speed
        }
        fn set_init_grazer_count(&mut self, new_init_grazer_count: u32){
            self.init_grazer_count = new_init_grazer_count;
        }
        fn set_grazer_energy_input(&mut self, new_grazer_energy_input: u32){
            self.grazer_energy_input = new_grazer_energy_input;
        }
        fn set_grazer_energy_output(&mut self, new_grazer_energy_output: u32){
            self.grazer_energy_output = new_grazer_energy_output;
        }
        fn set_grazer_energy_to_reproduce(&mut self, new_grazer_energy_to_reproduce: u32){
            self.grazer_energy_to_reproduce = new_grazer_energy_to_reproduce;
        }
        fn set_grazer_maintain_speed(&mut self, new_maintain_speed: f32){
            self.grazer_maintain_speed = new_maintain_speed
        }
        fn set_grazer_max_speed(&mut self, new_max_speed: f32){
            self.grazer_max_speed = new_max_speed
        }
        //predators
        fn get_init_predator_count(&self) -> &u32{
            &self.init_predator_count
        }
        fn get_max_speed_hod(&self) -> &f32{
            &self.max_speed_hod
        }
        fn get_max_speed_hed(&self) -> &f32{
            &self.max_speed_hed
        }
        fn get_max_speed_hor(&self) -> &f32{
            &self.max_speed_hor
        }
        fn get_predator_maintain_speed(&self) -> &f32{
            &self.predator_maintain_speed
        }
        fn get_predator_energy_output(&self) -> &u32{
            &self.predator_energy_output
        }
        fn get_predator_energy_to_reproduce(&self) -> &u32{
            &self.predator_energy_to_reproduce
        }
        fn get_predator_max_offspring(&self) -> &u32{
            &self.predator_max_offspring
        }
        fn get_predator_gestation(&self) -> &f32{
            &self.predator_gestation
        }
        fn get_predator_offspring_energy(&self) -> &u32{
            &self.predator_offspring_energy
        }
        fn set_init_predator_count(&mut self, new_init_predator_count: u32){
            self.init_predator_count = new_init_predator_count;
        }
        fn set_max_speed_hod(&mut self, new_max_speed_hod: f32){
            self.max_speed_hod = new_max_speed_hod;
        }
        fn set_max_speed_hed(&mut self, new_max_speed_hed: f32){
            self.max_speed_hed = new_max_speed_hed;
        }
        fn set_max_speed_hor(&mut self, new_max_speed_hor: f32){
            self.max_speed_hor = new_max_speed_hor;
        }
        fn set_predator_maintain_speed(&mut self, new_predator_maintain_speed: f32){
            self.predator_maintain_speed = new_predator_maintain_speed;
        }
        fn set_predator_energy_output(&mut self, new_predator_energy_output: u32){
            self.predator_energy_output = new_predator_energy_output;
        }
        fn set_predator_energy_to_reproduce(&mut self, new_predator_energy_to_reproduce: u32){
            self.predator_energy_to_reproduce = new_predator_energy_to_reproduce;
        }
        fn set_predator_gestation(&mut self, new_predator_gestation: f32){
            self.predator_gestation = new_predator_gestation;
        }
        fn set_predator_offspring_energy(&mut self, new_predator_offspring_energy: u32){
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
    struct Entity {
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
    struct Rock {
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
        fn set_diameter(&mut self, new_diameter: u32){
            self.diameter = new_diameter
        }

    }

    struct Plant {
        entity: Entity,
        diameter: u32,
        is_max_size: bool,
        
    }
    impl Plant{
        fn get_entity(&self) -> &Entity{
            &self.entity
        }
        fn get_diameter(&self) -> &u32 {
            &self.diameter
        }
        // This function returns if the plant is max size
        // if bool flag is set then return true
        // else check if it has reached max size by comapring diameter to map max size then update plants bool flag
        // else return false not max size
        fn is_max_size(&mut self, map: &Map)-> bool{
            if self.is_max_size == true{
                return self.is_max_size;
            }
            else if self.get_diameter() == map.get_max_size(){
                self.is_max_size == true;
                return self.is_max_size;
            }
            else{
                return false;
            }
        }
        fn set_diameter(&mut self, new_diameter: u32){
            self.diameter = new_diameter;
        }
        //need actual seeding functions

    }
}
