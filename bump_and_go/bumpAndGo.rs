extern crate env_logger;
#[macro_use]
extern crate rosrust;
#[macro_use]
extern crate rosrust_codegen;
extern crate rand;
use rand::Rng;
use std::time::{Duration, SystemTime};

rosmsg_include!();

// Copy and clone implementations to be able to send messages
impl Copy for msg::geometry_msgs::Vector3 { }
impl Clone for msg::geometry_msgs::Vector3 {
    fn clone(&self) -> msg::geometry_msgs::Vector3 {
        *self
    }
}
impl Copy for msg::geometry_msgs::Twist { }
impl Clone for msg::geometry_msgs::Twist {
    fn clone(&self) -> msg::geometry_msgs::Twist {
        *self
    }
}

const VA: f64 = 0.3;
const STOP: i64 = 0;
const GO: i64 = 1;
const TURN: i64 = 2;

static mut BUMP_EVENT: u8 = 0;
static mut BUMP_SIDE: u8 = 0;

// Turn base randomly to the oposite side of the active bumper
fn turn_base(side: u8, base_cmd: &mut msg::geometry_msgs::Twist) {
	let mut rng = rand::thread_rng();
   	let mut random: f64;

   	while { random = rng.gen::<f64>() % 10.0 - 5.0; random == 0.0 }{}

	let vg = random / 10.0;
	base_cmd.linear.x = 0.0;
	base_cmd.angular.z = match side {
		0 => -VA * 4.0,
		1 => vg,
		2 => VA * 4.0,
		_ => panic!("Unexpected invalid side {:?}", side),
	}
	ros_info!("Turn velocity: {}, {}", base_cmd.linear.x, base_cmd.angular.z);
}

fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("bumpAndGo");

    //Create subscriber
	let _subscriber = rosrust::subscribe("mobile_base/events/bumper", |msg: msg::kobuki_msgs::BumperEvent| {
        // Callback for handling received messages
        ros_info!("Bumper_Event: {}, Bumper_Side: {}", msg.state, msg.bumper);
		unsafe {
			BUMP_EVENT = msg.state;
			BUMP_SIDE = msg.bumper;
		}
	}).unwrap();


    // Create publisher
    let mut _publisher = rosrust::publish("mobile_base/commands/velocity").unwrap();

    let mut _count = 0;

    // Create object that maintains 10Hz between sleep requests
    let mut rate = rosrust::rate(10.0);
	let mut state = 1;
	let mut now: SystemTime;
	let mut end = SystemTime::now();
	let mut base_cmd: msg::geometry_msgs::Twist = 
				msg::geometry_msgs::Twist{linear: msg::geometry_msgs::Vector3{x: VA, y: 0.0, z: 0.0}, 
											angular: msg::geometry_msgs::Vector3{x: 0.0, y: 0.0, z: 0.0}};

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
    	    	match state {
   			STOP => {
   				if unsafe{BUMP_EVENT == 0} {
					state = TURN;	
				}
			},
			TURN => {
				now = SystemTime::now();
				if (unsafe{BUMP_EVENT == 0}) && (now > end){
					turn_base(unsafe{BUMP_SIDE}, &mut base_cmd);
					state = GO;
					now = SystemTime::now();
					end = now + Duration::new(3, 0);
				}
				ros_info!("Send turn velocity: {}, {}", base_cmd.linear.x, base_cmd.angular.z);
				_publisher.send(base_cmd).unwrap();
			},
			GO => {
				now = SystemTime::now();
				if now > end {
					base_cmd.angular.z = 0.0;
					base_cmd.linear.x = VA;
					if unsafe{BUMP_EVENT == 1} {
						state = STOP;
						base_cmd.linear.x = -VA;
						now = SystemTime::now();
						end = now + Duration::new(1, 0);
					}
				}
				ros_info!("Send go velocity: {}, {}", base_cmd.linear.x, base_cmd.angular.z);
				_publisher.send(base_cmd).unwrap();
			},
			_ => panic!("Unexpected invalid state {:?}", state),
	   	}
        // Sleep to maintain 10Hz rate
        rate.sleep();

        _count += 1;
    }
}