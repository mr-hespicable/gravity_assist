use std::f64::consts::SQRT_2;

#[derive(Debug, Clone, Copy)]
/// A 2-dimensional vector.
///
/// Direction is given by an angle, in radians, from the x-axis.
pub struct Vector {
    /// angle in radians
    pub angle: f64,
    /// magnitude in arbitrary units
    pub magnitude: f64,
}

impl Vector {
    /// Returns a 2-dimensional vector
    pub fn new(angle: f64, magnitude: f64) -> Vector {
        Vector { angle, magnitude }
    }

    /// Returns the 2-dimensional unit vector.
    pub fn unit_vector() -> Vector {
        Vector {
            angle: 45_f64.to_radians(),
            magnitude: SQRT_2,
        }
    }
    /// Returns the x component of the vector
    pub fn x(&self) -> f64 {
        self.magnitude * self.angle.cos()
    }

    /// Returns the y component of the vector
    pub fn y(&self) -> f64 {
        self.magnitude * self.angle.sin()
    }

    /// Adds the given vectors to the current vector,
    /// returning the resultant vector.
    pub fn add_vectors(&self, vector_list: Vec<&Vector>) -> Vector {
        let mut vx_list: Vec<f64> = vec![]; // list of x components of vectors
        let mut vy_list: Vec<f64> = vec![]; // list of y components of vectors

        vx_list.push(self.x());
        vy_list.push(self.y());

        for vector in vector_list {
            vx_list.push(vector.x());
            vy_list.push(vector.y());
        }

        let vxr: f64 = vx_list.iter().sum();
        let vyr: f64 = vy_list.iter().sum();

        let vr = (vxr.powi(2) + vyr.powi(2)).sqrt();
        let ar = (vyr / vr).asin();

        Vector {
            angle: ar,
            magnitude: vr,
        }
    }

    /// Returns the dot product of a given vector with the current vector.
    pub fn dot_product(&self, vector: &Vector) -> f64 {
        let x_product: f64 = self.x() * vector.x();
        let y_product: f64 = self.y() * vector.y();

        x_product + y_product
    }

    pub fn components_to_vector(&self, x_component: f64, y_component: f64) -> Vector {
        let eh_squared_plus_bee_squared = x_component.powi(2) + y_component.powi(2);
        let angle = {
            let oh_over_ay = y_component / x_component;
            oh_over_ay.atan()
        };
        Vector {
            angle,
            magnitude: f64::sqrt(eh_squared_plus_bee_squared),
        }
    }
}
