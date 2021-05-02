#[macro_use]
extern crate rosrust_codegen;

rosmsg_main!("geometry_msgs/Twist", "std_msgs/String", "kobuki_msgs/BumperEvent");