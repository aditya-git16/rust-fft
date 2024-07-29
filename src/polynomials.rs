// Polynomial evaluation function

#[derive(Debug)]
pub struct Polynomial{ 
    pub coefficients : Vec<f64>,
}

impl Polynomial{
    pub fn new_polynomial(coefficients: Vec<f64>) -> Self{
        Polynomial {coefficients}
    }

    pub fn evaluate(&self , x:f64) -> f64 {
        let mut result = 0.0;
        let mut power = 1.0;

        for coefficient in self.coefficients.iter(){
            result += coefficient * power;
            power *= x;
        }
        result
    }

    // Implement the addition and multiplication functions
}
