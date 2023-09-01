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
        grazer_max_speeed: f32,
        //predator
        init_predator_count: u32,
        max_speed_hod: f32,
        max_speed_hed: f32,
        max_speed_hor: f32,
        predator_maintain_speed: f32,
        predator_energy_to_reproduce: u32,
        predator_max_offspring: u32,
        predator_gestation: f32,
        predator_offspring_energy: u32,
    }
    

    trait BaseEntity {
        fn get_id(&self) -> u32;
        fn get_x(&self) -> i32;
        fn get_y(&self) -> i32;
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
        fn get_id(&self) -> u32 {
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
    struct Rock {
        entity: Entity,
        diameter: u32,
    }

    impl Rock {
        fn get_entity(&self) -> &Entity {
            &self.entity
        }
        fn get_diameter(&self) -> u32 {
            self.diameter
        }
    }

    struct Plant {
        id: u32,
        x: i32,
        y: i32,
        state: i32, // might need more complexity
        growth_rate: f32,
        // MAX_SIZE: i32,
        // MAX_SEED_NUMBER: i32,
        // MAX_SEED_DISTANCE: f32,
        // SEED_VIABILITY: i32, //should this be constant?
        //plant height and diameter is the same
    }
}
