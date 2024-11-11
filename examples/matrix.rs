use anyhow::Result;
use rust_template::{multiply, Matrix};

#[allow(clippy::needless_borrows_for_generic_args)]
fn main() -> Result<()> {
    let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new(&[1, 2, 3, 4, 5, 6], 3, 2);
    let c = multiply(&a, &b)?;
    println!("{:?}", c);
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[allow(clippy::needless_borrows_for_generic_args)]
    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(2, c.col);
        assert_eq!(2, c.row);
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row = 2, col = 2, [[22,28][49,64]])"
        );
        Ok(())
    }
}
