use std::io;

const DIMENSION: usize = 3;
const DIMENSIONS: [char; DIMENSION] = ['x', 'y', 'z'];

struct Point(f64, f64, f64);

struct Polygon {
    points: Vec<Point>,
    mass: Option<f64>,
    density: Option<f64>,
    area: Option<f64>,
}

impl Polygon {
    fn centre_of_mass(&self) -> Point {
        let mut total_x: f64 = 0.0;
        let mut total_y: f64 = 0.0;
        let mut total_z: f64 = 0.0;

        for point in &self.points {
            total_x += point.0;
            total_y += point.1;
            total_z += point.2;
        }

        let num_points: f64 = self.points.len() as f64;

        return Point(
            total_x / num_points,
            total_y / num_points,
            total_z / num_points,
        );
    }

    fn get_mass(&self) -> f64 {
        match self.mass {
            Some(num) => {
                return num;
            }
            None => (),
        }

        let density: f64 = match self.density {
            Some(num) => num,
            None => 1.0,
        };

        let area: f64 = match self.area {
            Some(num) => num,
            None => 1.0,
        };

        return density * area;
    }
}

struct Circle {
    centre: Point,
    mass: Option<f64>,
    density: Option<f64>,
    area: Option<f64>,
}

impl Circle {
    fn centre_of_mass(&self) -> &Point {
        return &self.centre;
    }

    fn get_mass(&self) -> f64 {
        match self.mass {
            Some(num) => {
                return num;
            }
            None => (),
        }

        let density: f64 = match self.density {
            Some(num) => num,
            None => 1.0,
        };

        let area: f64 = match self.area {
            Some(num) => num,
            None => 1.0,
        };

        return density * area;
    }
}

enum Shape {
    RegularPolygon(Polygon),
    Circle(Circle),
}

fn main() {
    let shapes: Vec<Shape> = create_shapes();

    let centre_of_mass = calculate_centre_of_mass(shapes);

    println!(
        "The centre of mass is: Point({0}, {1}, {2})",
        centre_of_mass.0, centre_of_mass.1, centre_of_mass.2
    );
}

fn create_point() -> Point {
    let mut point_data: [f64; DIMENSION] = [0.0; DIMENSION];

    for i in 0..DIMENSION {
        println!("Enter {0}", DIMENSIONS[i]);

        let mut data = String::new();
        match io::stdin().read_line(&mut data) {
            Ok(_) => (),
            Err(_) => {
                continue;
            }
        }

        let data: f64 = match data.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        point_data[i] = data;
    }

    return Point(point_data[0], point_data[1], point_data[2]);
}

fn create_regular_polygon() -> Option<Shape> {
    println!("How many points: ");
    let mut num_points = String::new();

    match io::stdin().read_line(&mut num_points) {
        Ok(_) => (),
        Err(_) => {
            return None;
        }
    }

    let num_points: i32 = match num_points.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut points: Vec<Point> = Vec::new();

    for _ in 0..num_points {
        points.push(create_point());
    }

    println!("Enter mass");
    let mut data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let mass: Option<f64> = match data.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    println!("Enter density");
    data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let density: Option<f64> = match data.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    println!("Enter area");
    data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let area: Option<f64> = match data.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    return Some(Shape::RegularPolygon(Polygon {
        points,
        mass,
        density,
        area,
    }));
}

fn create_circle() -> Option<Shape> {
    println!("Enter centre");
    let centre: Point = create_point();

    println!("Enter mass");
    let mut data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let mass: Option<f64> = match data.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    println!("Enter density");
    data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let density: Option<f64> = match data.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    println!("Enter area");
    let mut data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let area: Option<f64> = match data.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    return Some(Shape::Circle(Circle {
        centre,
        mass,
        density,
        area
    }))
}

fn create_shape() -> Option<Shape> {
    println!("Enter shape type: ");
    println!("1. Regular Polygon");
    println!("2. Circle");

    let mut shape_type = String::new();
    match io::stdin().read_line(&mut shape_type) {
        Ok(_) => (),
        Err(_) => {
            return None;
        }
    }

    let shape_type: i32 = match shape_type.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match shape_type {
        1 => {
            return create_regular_polygon();
        }
        2 => {
            return create_circle();
        }
        _ => (),
    }

    None
}

fn create_shapes() -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::new();

    println!("How many shapes do you want?");
    let mut data = String::new();

    match io::stdin().read_line(&mut data) {
        Ok(_) => (),
        Err(_) => (),
    }

    let num_shapes: usize = match data.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    for _ in 0..num_shapes {
        let shape: Option<Shape> = create_shape();

        match shape {
            Some(shape) => {
                shapes.push(shape);
            },
            None => {
                println!("Invalid Shape, Shape not added");
            }
        }
    }

    return shapes;
}

fn calculate_centre_of_mass(shapes: Vec<Shape>) -> Point {
    let mut total_x: f64 = 0.0;
    let mut total_y: f64 = 0.0;
    let mut total_z: f64 = 0.0;

    let mut total_mass: f64 = 0.0;

    for shape in &shapes {
        match shape {
            Shape::RegularPolygon(polygon) => {
                let centre: Point = polygon.centre_of_mass();

                let mass = polygon.get_mass();

                total_x += centre.0 * mass;
                total_y += centre.1 * mass;
                total_z += centre.2 * mass;

                total_mass += mass;
            }
            Shape::Circle(circle) => {
                let centre: &Point = circle.centre_of_mass();
                let mass = circle.get_mass();

                total_x += centre.0 * mass;
                total_y += centre.1 * mass;
                total_z += centre.2 * mass;

                total_mass += mass;
            }
        }
    }

    return Point(
        total_x / total_mass,
        total_y / total_mass,
        total_z / total_mass,
    );
}
