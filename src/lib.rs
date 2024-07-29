use  std::io::{self, Write};
// use  std::io::{self, stdin, Cursor, Write};
use polynomials::Polynomial;

pub mod polynomials;

pub fn get_polynomial_from_input() -> io::Result<Polynomial> {
    let mut input = String::new();

    print!("Coefficients for the polynomial: ");
    io::stdout().flush()?;

    //reader.read_line(&mut input)?;
    io::stdin().read_line(&mut input)?;

    let coefficients: Vec<f64> = input
    .trim()
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

    if coefficients.is_empty(){
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"No valid coefficients entered"));
    }

    Ok(Polynomial::new_polynomial(coefficients))
}
#[cfg(test)]
mod tests{
    use std::{io::Cursor, result};

    use crate::{get_polynomial_from_input, polynomials::Polynomial};

    #[test]
    fn test_get_polynomial_from_input_valid(){
        println!("This test requires user input.");
        println!("Enter coefficients when prompted.");

        match get_polynomial_from_input(){
            Ok(polynomial) => {
                println!("Received polynomial: {:?}" , polynomial);

            },
            Err(e) => {
                panic!("Error occured: {}" , e);
            }
        }
    }

    #[test]
    fn test_evaluate_function(){
        println!("Enter coefficients for the polynomial.");
        match get_polynomial_from_input(){
            Ok(polynomial) => {
                let result = Polynomial::evaluate(&polynomial, 2.0);
                assert_eq!(result,31.0);
                println!("Evaluated polynomial is: {:?}" , result);
            }
            Err(e) => {
                panic!("Error occured: {}" , e);
            }
        }
    }
}