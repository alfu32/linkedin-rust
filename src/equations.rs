pub fn first_degree(a: f64, b: f64) -> Option<f64> {
    Some(-b / a)
}
pub fn second_degree(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta_squared = b * b - 4.0 * a * c;
    if delta_squared < 0.0 {
        None
    } else {
        let x1 = (-b + delta_squared.sqrt()) / 2.0 / a;
        let x2 = (-b - delta_squared.sqrt()) / 2.0 / a;
        Some((x1, x2))
    }
}
pub(crate) struct Vector2 {
    x: f64,
    y: f64,
}

pub(crate) struct System2 {
    matrix: [[f64; 3]; 2],
}

impl System2 {
    pub fn new(matrix: [[f64; 3]; 2]) -> System2 {
        System2 { matrix }
    }
    pub(crate) fn resolve(&self) -> Option<Vector2> {
        let det = self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0];
        if det == 0.0 {
            None
        } else {
            let dx = self.matrix[0][2] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][2];
            let dy = self.matrix[0][0] * self.matrix[1][2] - self.matrix[0][2] * self.matrix[1][0];
            Some(Vector2 {
                x: dx / det,
                y: dy / det,
            })
        }
    }
}
