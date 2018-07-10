#[derive(Debug)]
struct Body;

#[derive(Debug)]
struct Chassis;

#[derive(Debug)]
struct Engine;

#[derive(Debug)]
struct WagonBody;

#[derive(Debug)]
struct WagonChassis;

#[derive(Debug)]
struct WagonEngine;

#[derive(Debug)]
struct Car<BodyT, ChassisT, EngineT> {
    body: BodyT,
    chassis: ChassisT,
    engine: EngineT,
}

trait CarTrait {
    fn get_info(&self) -> &str;
}

impl CarTrait for Car<Body, Chassis, Engine> {
    fn get_info(&self) -> &str {
        "simple car!"
    }
}

impl CarTrait for Car<WagonBody, WagonChassis, WagonEngine> {
    fn get_info(&self) -> &str {
        "wagon car!"
    }
}

fn main() {
    let car: Car<Body, Chassis, Engine> = Car {
        body: Body {},
        chassis: Chassis {},
        engine: Engine {},
    };
    println!("{:?}", car);
    println!("{}", car.get_info());

    let wagon_car: Car<WagonBody, WagonChassis, WagonEngine> = Car {
        body: WagonBody {},
        chassis: WagonChassis {},
        engine: WagonEngine {},
    };
    println!("{:?}", wagon_car);
    println!("{}", wagon_car.get_info());

    let broken_car: Car<Body, WagonChassis, WagonEngine> = Car {
        body: Body {},
        chassis: WagonChassis {},
        engine: WagonEngine {},
    };
    println!("{:?}", broken_car);
    // println!("{}", broken_car.get_info());
    //=> no method named `get_info`

}