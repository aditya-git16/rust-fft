// Polynomial evaluation function

#[derive(Debug)]
pub struct Polynomial{ 
    pub coefficients : Vec<f64>,
}

impl Polynomial{
    pub fn new_polynomial(coefficients: Vec<f64>) -> Self{
        Polynomial {coefficients}
    }
}

// pub fn evaluate_polynomial()