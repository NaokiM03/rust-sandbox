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

struct Parking {
    cars: Vec<Box<CarTrait>>,
    // capacity_limit: u64,
    current_capacity: u64,
}


trait CarTrait {
    fn get_info(&self) -> &str;
}

trait WagonCarTrait {
    fn get_info(&self) -> &str;
    fn fly(&self) -> &str;
}

trait ParkingTrait {
    fn size(&self) -> usize;
    fn get_current_capacity(&self) -> u64;
    fn enter_simple(&mut self, car: Car<Body, Chassis, Engine>) -> ();
    fn enter_wagon(&mut self, car: Car<WagonBody, WagonChassis, WagonEngine>) -> ();
}


impl ParkingTrait for Parking {
    fn get_current_capacity(&self) -> u64 {
        self.current_capacity
    }
    fn size(&self) -> usize {
        self.cars.len()
    }
    fn enter_simple(&mut self, car: Car<Body, Chassis, Engine>) -> () {
        self.cars.push(Box::new(car));
        self.current_capacity += 1;
    }
    fn enter_wagon(&mut self, car: Car<WagonBody, WagonChassis, WagonEngine>) -> () {
        self.cars.push(Box::new(car));
        self.current_capacity += 2;
    }
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

    let mut parking = Parking {
        cars: Vec::new(),
        // capacity_limit: 100,
        current_capacity:0,
    };
    parking.enter_simple(car);
    parking.enter_wagon(wagon_car);
    println!("count: {}", parking.size());
    println!("current_capacity: {}", parking.get_current_capacity());
}