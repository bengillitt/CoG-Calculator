enum ShapeType {
    RegularPolygon,
    Circle,
}

struct Point(f64, f64, f64);

struct Shape {
    shape_type: ShapeType,
    points: Vec<Point>,
    mass: u32,
}

fn main() {
    let p1: Shape = Shape {
        shape_type: ShapeType::RegularPolygon,
        points: vec![Point(-4.0, 3.0, 0.0)],
        mass: 2,
    };

    let p2: Shape = Shape {
        shape_type: ShapeType::RegularPolygon,
        points: vec![Point(-1.0, -3.0, 0.0)],
        mass: 3,
    };

    let p3: Shape = Shape {
        shape_type: ShapeType::RegularPolygon,
        points: vec![Point(4.0, -3.0, 0.0)],
        mass: 4,
    };

    let p4: Shape = Shape {
        shape_type: ShapeType::RegularPolygon,
        points: vec![Point(3.0, 3.0, 0.0)],
        mass: 1,
    };

    let centre_of_mass: Point = calculate_centre_of_mass(vec![p1, p2, p3, p4]);

    println!(
        "Point({0}, {1}, {2})",
        centre_of_mass.0, centre_of_mass.1, centre_of_mass.2
    );

    let c1: Shape = Shape {
        shape_type: ShapeType::RegularPolygon,
        points: vec![Point(0.0, 0.0, 0.0), Point(4.0, 0.0, 0.0), Point(0.0, 4.0, 0.0)],
        mass: 4*4*4
    };
}

fn calculate_centre_of_mass(shapes: Vec<Shape>) -> Point {
    let mut total_weighted_x: f64 = 0.0;
    let mut total_weighted_y: f64 = 0.0;
    let mut total_weighted_z: f64 = 0.0;

    let mut total_mass: u32 = 0;

    for shape in shapes {
        if let ShapeType::RegularPolygon = shape.shape_type {
            let mut average_x: f64 = 0.0;
            let mut average_y: f64 = 0.0;
            let mut average_z: f64 = 0.0;

            for point in &shape.points {
                average_x += point.0;
                average_y += point.1;
                average_z += point.2;
            }

            average_x = average_x / (shape.points.len() as f64);
            average_y = average_y / (shape.points.len() as f64);
            average_z = average_z / (shape.points.len() as f64);

            total_weighted_x += average_x * (shape.mass as f64);
            total_weighted_y += average_y * (shape.mass as f64);
            total_weighted_z += average_z * (shape.mass as f64);

            total_mass += shape.mass;
        } else if let ShapeType::Circle = shape.shape_type{
            if shape.points.len() > 0 {
                total_weighted_x += shape.points[0].0 * (shape.mass as f64);
                total_weighted_y += shape.points[0].1 * (shape.mass as f64);
                total_weighted_y += shape.points[0].2 * (shape.mass as f64);

                total_mass += shape.mass;
            }
        }
    }

    return Point(
        total_weighted_x / (total_mass as f64),
        total_weighted_y / (total_mass as f64),
        total_weighted_z / (total_mass as f64),
    );
}
