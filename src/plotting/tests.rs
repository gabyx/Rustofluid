#[cfg(test)]
mod tests {

    use crate::plotting::plot::grid;
    use crate::types::*;

    #[test]
    fn test_grid() -> Result<(), Box<dyn std::error::Error>> {
        let get_data = |index: Index2| {
            if index.y == 300 - 1 {
                return None;
            }

            return Some(((index.x as Scalar) / 15.0).sin() * ((index.y as Scalar) / 10.0).cos());
        };

        let file = "test.png";
        grid(dim!(500, 500), dim!(300, 300), get_data, file.to_string(), None, "Nothing".to_string())?;

        Ok(())
    }
}