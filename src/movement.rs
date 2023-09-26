/*Ben and Jake's*/

use std::rc::Rc; 
use rand::Rng;

#[derive(Clone)]
enum State {
    Seek(Rc<Entity>),
    Stay()
}

#[derive(Clone, Copy)]
pub struct Entity {
    id: u32,
    x: f64,
    y: f64,
}

impl Entity {
    fn new(new_id: u32, new_x: f64, new_y: f64) -> Entity {
        Entity {
            id: new_id,
            x: new_x,
            y: new_y,
        }
    }
    fn get_id(&self) ->u32 {
        self.id
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn set_id(&mut self, new_id: u32) {
        self.id = new_id;
    }

    fn set_x(&mut self, new_x: f64) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: f64) {
        self.y = new_y;
    }
}

#[derive(Clone)]
pub struct Mover {
    entity: Entity,
    state: State, // needs to be enum of state
    velocity_x: f64,
    velocity_y: f64,
    orientation: f64,
    rotation: f64,
    target_x: f64,
    target_y: f64,
    energy: f64,
    max_accel: f64,
    velocity: f64,
}

impl Mover {
    fn new(new_id: u32, new_x:f64, new_y: f64, new_state: State, new_velocity_x: f64, new_velocity_y: f64, new_orientation: f64, new_rotation: f64, new_target_x: f64,new_target_y: f64, new_energy: f64, new_max_acceleration: f64, new_max_velocity: f64) -> Mover {
        Mover { entity: Entity::new(new_id, new_x, new_y), state: new_state, velocity_x: new_velocity_x, velocity_y: new_velocity_y, orientation: new_orientation, rotation: new_rotation, target_x: new_target_x, target_y: new_target_y, energy: new_energy, max_accel: new_max_acceleration, velocity: new_max_velocity }
    }
    fn get_state(&self) -> State {
        //change to enum in future
        self.state.clone()
    }
    fn get_velocity_x(&self) -> f64 {
        self.velocity_x
    }
    fn get_velocity_y(self) -> f64 {
        self.velocity_y
    }
    fn get_orientation(&self) -> f64 {
        self.orientation
    }
    fn get_rotation(&self) -> f64 {
	self.rotation
    }
    fn get_target_x(&self) -> f64 {
        self.target_x
    }
    fn get_target_y(&self) -> f64 {
        self.target_y
    }
    fn get_energy(&self) -> f64 {
        self.energy
    }
    fn set_state(&mut self, new_state: State) {
        //need to be enum here once we do that
        self.state = new_state;
    }
    fn set_velocity_x(&mut self, new_velocity_x: f64) {
        self.velocity_x = new_velocity_x;
    }
    fn set_velocity_y(&mut self, new_velocity_y: f64) {
        self.velocity_y = new_velocity_y;
    }
    fn set_orientation(&mut self, new_orientation: f64) {
        self.orientation = new_orientation;
    }
    fn set_target_x(&mut self, new_target_x: f64) {
        self.target_x = new_target_x;
    }
    fn set_target_y(&mut self, new_target_y: f64) {
        self.target_y = new_target_y;
    }
    fn set_energy(&mut self, new_energy: f64) {
        self.energy = new_energy;
    }
}



fn main() {
   
	let a = Mover {
        entity: Entity { id: 0, x: 0.0, y: 0.0 },
        state: State::Stay(), // needs to be enum of state
        velocity_x: 0.0,
        velocity_y: 0.0,
        orientation: 0.0,
	rotation: 0.0,
        target_x: 0.0,
        target_y: 0.0,
        energy: 0.0,
        max_accel: 0.0,
        velocity: 0.0
	};

    let mut b = Mover {
        entity: Entity { id: 0, x: 50.0, y: -50.0 },
        state: State::Seek(Rc::new(a.entity)), // needs to be enum of state
        velocity_x: 2.0,
        velocity_y: 2.0,
        orientation: 0.0,
	rotation: 0.0,
        target_x: 0.0,
        target_y: 0.0,
        energy: 0.0,
        max_accel: 0.5,
        velocity: 5.0
    };
    
   
    for _ in 1..100 {
        let b = wander(&mut b, 1.0);
        let mut b = b.clone();

        println!("{},{}", b.entity.x,b.entity.y);
    }

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
