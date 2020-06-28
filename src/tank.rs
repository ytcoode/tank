struct Tank {
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

impl Tank {
    fn new() -> Tank {
        Tank {
            position: Point2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}
