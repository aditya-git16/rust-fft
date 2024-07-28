use  std::io::{self, Write};
// use  std::io::{self, stdin, Cursor, Write};
use polynomials::Polynomial;

pub mod polynomials;

pub fn get_polynomial_from_input<R: io::BufRead>(mut reader:R) -> io::Result<Polynomial> {
    let mut input = String::new();

    print!("Coefficients for the polynomial: ");
    io::stdout().flush()?;

    reader.read_line(&mut input)?;
    // io::stdin().read_line(&mut input)?;

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
    use std::io::Cursor;

    use crate::get_polynomial_from_input;

    #[test]
    fn test_get_polynomial_from_input_valid(){
        // Simulating user input
        let input= b"1.0 2.5 3.7\n";
        let cursor = Cursor::new(input);
        let result = get_polynomial_from_input(cursor);
        assert!(result.is_ok());
        let polynomial =result.unwrap();
        assert_eq!(polynomial.coefficients, vec![1.0, 2.5, 3.7]);
    }
}