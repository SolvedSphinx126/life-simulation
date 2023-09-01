trait Entity {
    fn get_id(&self) -> u32;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}

struct Rock{
    id: u32,
    x: i32,
    y: i32,
    diameter: u32,
}

impl Entity for Rock {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}

impl Rock{
    fn get_diameter(&self) -> u32 {
        self.diameter
    }
}


struct Plant{
    id: u32;
    x: i32;
    y: i32;
    state: i32; // might need more complexity
    growth_rate: i32;
    const MAX_SIZE: i32;
    const MAX_SEED_NUMBER: i32;
    const MAX_SEED_DISTANCE: i32;
    const SEED_VIABILITY: i32; //should this be constant?
    //plant height and diameter is the same
}

impl Entity for Plant{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

}

impl Plant{
    fn get_growth_rate(&self) -> u32 {
        self.growth_rate
    }
}

