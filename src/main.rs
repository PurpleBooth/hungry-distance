use std::error::Error;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::app().get_matches();
    let coordinate1 = Coordinate(
        match matches.value_of("x1") {
            Some(value) => value.parse()?,
            None => return Err("missing x1".into()),
        },
        match matches.value_of("y1") {
            Some(value) => value.parse()?,
            None => return Err("missing y1".into()),
        },
        match matches.value_of("z1") {
            Some(value) => value.parse()?,
            None => return Err("missing z1".into()),
        },
    );
    let coordinate2 = Coordinate(
        match matches.value_of("x2") {
            Some(value) => value.parse()?,
            None => return Err("missing x2".into()),
        },
        match matches.value_of("y2") {
            Some(value) => value.parse()?,
            None => return Err("missing y2".into()),
        },
        match matches.value_of("z2") {
            Some(value) => value.parse()?,
            None => return Err("missing z2".into()),
        },
    );
    println!("{}", distance(&coordinate1, &coordinate2,));

    Ok(())
}

struct Coordinate(f64, f64, f64);

fn distance(a: &Coordinate, b: &Coordinate) -> f64 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2) + (b.2 - a.2).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::{distance, Coordinate};

    #[test]
    fn coordinate_distance() {
        assert_eq!(
            true,
            (distance(
                &Coordinate(-11.46875, 39.78125, 22.78125,),
                &Coordinate(73.875, -3.5625, -52.625,)
            ) - 121.853_760_168_439_2_f64)
                .abs()
                < f64::EPSILON
        );
    }
}
