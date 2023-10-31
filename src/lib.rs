use std::{cmp, collections::HashMap};

use integer_sqrt::IntegerSquareRoot;

#[derive(Eq, PartialEq, Default, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Default, Eq, PartialEq, Debug)]
struct GridValue {
    coord: Coordinate,
    value: u64,
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Diagonal {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone)]
struct GeneratorAttributes {
    base_value: u8,
    x_incr: i8,
    y_incr: i8,
}

fn get_diagonal_ahead(coord: &Coordinate) -> Diagonal {
    let (x, y) = (coord.x, coord.y);
    if x.abs() == y.abs() {
        if x > 0 && y > 0 {
            Diagonal::TopRight
        } else if x > 0 && y < 0 {
            Diagonal::BottomRight
        } else if x < 0 && y > 0 {
            Diagonal::TopLeft
        } else if x < 0 && y < 0 {
            Diagonal::BottomLeft
        } else {
            unreachable!()
        }
    } else if x > y.abs() {
        // right quadrant
        Diagonal::TopRight
    } else if x < -y.abs() {
        // left quadrant
        Diagonal::BottomLeft
    } else if x.abs() < y {
        // top quadrant
        Diagonal::TopLeft
    } else if -x.abs() > y {
        // bottom quadrant
        Diagonal::BottomRight
    } else {
        unreachable!()
    }
}

fn get_diagonal_value(x_incr: i8, y_incr: i8, base_value: u8, ring: u64) -> GridValue {
    GridValue {
        coord: Coordinate {
            x: x_incr as i64 * ring as i64,
            y: y_incr as i64 * ring as i64,
        },
        value: {
            (4 * ring.pow(2) as i128 - ring as i128 * (5 - base_value as i8) as i128 + 1) as u64
        }, // math magic
    }
}

fn ceil_sqrt(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        (n - 1).integer_sqrt() + 1
    }
}

fn get_number_ring(target: u64) -> u64 {
    ceil_sqrt(target) / 2
} // integer divison

fn get_coords(
    target: u64,
    diagonal_properties: &HashMap<Diagonal, GeneratorAttributes>,
) -> Coordinate {
    let ring = get_number_ring(target);

    let mut diffs: HashMap<&Diagonal, (GridValue, u64)> = HashMap::new();
    for (diag, props) in diagonal_properties {
        let value = get_diagonal_value(props.x_incr, props.y_incr, props.base_value, ring);
        if value.value >= target {
            let diff = value.value - target;
            diffs.insert(diag, (value, diff));
        }
    }

    let mut smallest: (&Diagonal, u64, GridValue) =
        (&Diagonal::TopLeft, u64::MAX, GridValue::default()); // placeholder value
    for (diag, (gridvalue, diff)) in diffs {
        if diff < smallest.1 {
            smallest = (diag, diff, gridvalue)
        }
    }

    let diagonal_ahead = smallest.0;
    let value_ahead = smallest.2;

    let mut x_offset = 0;
    let mut y_offset = 0;

    match diagonal_ahead {
        Diagonal::TopLeft => x_offset = -(target as i128 - value_ahead.value as i128) as i64,
        Diagonal::TopRight => y_offset = (target as i128 - value_ahead.value as i128) as i64,
        Diagonal::BottomLeft => y_offset = -(target as i128 - value_ahead.value as i128) as i64,
        Diagonal::BottomRight => x_offset = (target as i128 - value_ahead.value as i128) as i64,
    }

    Coordinate {
        x: value_ahead.coord.x + x_offset,
        y: value_ahead.coord.y + y_offset,
    }
}

fn get_value_at_coords(
    coord: &Coordinate,
    diagonal_properties: &HashMap<Diagonal, GeneratorAttributes>,
) -> u64 {
    if coord == (&Coordinate { x: 0, y: 0 }) {
        return 1;
    }
    let ring = cmp::max(coord.x.abs(), coord.y.abs()) as u64;

    let diagonal_ahead = get_diagonal_ahead(coord);

    let props = diagonal_properties[&diagonal_ahead];

    match diagonal_ahead {
        Diagonal::TopRight | Diagonal::BottomLeft => {
            let gridvalue = get_diagonal_value(props.x_incr, props.y_incr, props.base_value, ring);
            gridvalue.value - (gridvalue.coord.y - coord.y).unsigned_abs()
        }
        Diagonal::BottomRight | Diagonal::TopLeft => {
            let gridvalue = get_diagonal_value(props.x_incr, props.y_incr, props.base_value, ring);
            gridvalue.value - (gridvalue.coord.x - coord.x).unsigned_abs()
        }
    }
}

fn get_diagonal_properties() -> HashMap<Diagonal, GeneratorAttributes> {
    HashMap::from([
        (
            Diagonal::TopLeft,
            GeneratorAttributes {
                base_value: 5,
                x_incr: -1,
                y_incr: 1,
            },
        ),
        (
            Diagonal::TopRight,
            GeneratorAttributes {
                base_value: 3,
                x_incr: 1,
                y_incr: 1,
            },
        ),
        (
            Diagonal::BottomLeft,
            GeneratorAttributes {
                base_value: 7,
                x_incr: -1,
                y_incr: -1,
            },
        ),
        (
            Diagonal::BottomRight,
            GeneratorAttributes {
                base_value: 9,
                x_incr: 1,
                y_incr: -1,
            },
        ),
    ])
}

/// Perform grambulation upon two given positive integers.
///
/// # Grambulation
///
/// Using the following spiral of positive integers:
///
/// | | | | | | | |
/// |---|---|---|---|---|---|---|
/// | 37 | 36 | 35 | 34 | 33 | 32 | 31 |
/// | 38 | 17 | 16 | 15 | 14 | 13 | 30 |
/// | 39 | 18 | 5 | 4 | 3 | 12 | 29 |
/// | 40 | 19 | 6 | 1 | 2 | 11 | 28 |
/// | 41 | 20 | 7 | 8 | 9 | 10 | 27 |
/// | 42 | 21 | 22 | 23 | 24 |25 | 26 |
/// | 43 | 44 | 45 | 46 | 47 | 48 | ... |
///
/// We can define grambulation as an operation ◊ : N×N → N.
/// The spiral is not limited to any specific size.
///
/// To grambulate two numbers `a` and `b`, find their coordinates `a'` and `b'` on the grid.
/// Define a vector from `a'` to `b'` and calculate the coordinates `c'` of the solution `c` by applying it to the point `b'`.
///
/// ## Example of grambulation
///
/// To give an example: Let `a` be 5 and `b` be 15.  
/// That would mean `a' = (-1, 1)` and `b' = (0, 2)`.  
/// The connecting vector is `b' - a'`, so `(1, 1)`.  
/// Applying that vector to `b'` gives us `(1, 3)`, so `c'`.  
/// The value at those coordinates (33) is our result: 5 ◊ 15 = 33.  
///
/// This can be seen especially well graphically using the above grid.
///
/// # Errors
///
/// If either of the provided inputs is zero, an error will be returned. Succeeds otherwise.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), String> {
/// assert_eq!(grambulate::grambulate(1, 2)?, 11);
/// assert_eq!(grambulate::grambulate(5, 15)?, 33);
/// assert_eq!(grambulate::grambulate(7, 21)?, 43);
/// assert_eq!(grambulate::grambulate(21, 7)?, 1);
/// #   Ok(())
/// # }
/// ```
/// ```
/// assert!(grambulate::grambulate(0, 1).is_err());
/// ```
///
pub fn grambulate(value_a: u64, value_b: u64) -> Result<u64, String> {
    if value_a == 0 || value_b == 0 {
        return Err("inputs values may not be zero".to_owned());
    }

    let diagonal_properties = get_diagonal_properties();

    let a_coords = get_coords(value_a, &diagonal_properties);
    let b_coords = get_coords(value_b, &diagonal_properties);

    let diff = (b_coords.x - a_coords.x, b_coords.y - a_coords.y);
    let result_coords = Coordinate {
        x: (b_coords.x + diff.0),
        y: (b_coords.y + diff.1),
    };
    Ok(get_value_at_coords(&result_coords, &diagonal_properties))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceil_sqrt() {
        assert_eq!(ceil_sqrt(1), 1);
        assert_eq!(ceil_sqrt(2), 2);
        assert_eq!(ceil_sqrt(3), 2);
        assert_eq!(ceil_sqrt(4), 2);
        assert_eq!(ceil_sqrt(5), 3);

        assert_eq!(ceil_sqrt(8), 3);
        assert_eq!(ceil_sqrt(9), 3);
        assert_eq!(ceil_sqrt(10), 4);

        assert_eq!(ceil_sqrt(4611686018427387904), 2147483648);
        assert_eq!(ceil_sqrt(4611686018427387904 + 1), 2147483648 + 1);
    }

    #[test]
    fn test_get_number_ring() {
        assert_eq!(get_number_ring(1), 0);

        // with pytest.raises(ValueError):
        // get_number_ring(0)
        // with pytest.raises(ValueError):
        // get_number_ring(-3457834857)

        assert_eq!(get_number_ring(2), 1);
        assert_eq!(get_number_ring(9), 1);
        assert_eq!(get_number_ring(10), 2);
        assert_eq!(get_number_ring(25), 2);
        assert_eq!(get_number_ring(26), 3);

        assert_eq!(get_number_ring(289), 8);
    }

    #[test]
    fn test_get_diagonal_value() {
        let diagonal_properties = get_diagonal_properties();

        let mut diag = &Diagonal::TopLeft;
        let mut x_incr = diagonal_properties[diag].x_incr;
        let mut y_incr = diagonal_properties[diag].y_incr;
        let mut base_value = diagonal_properties[diag].base_value;
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 0),
            GridValue {
                coord: Coordinate { x: 0, y: 0 },
                value: 1
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 1),
            GridValue {
                coord: Coordinate { x: -1, y: 1 },
                value: 5
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 2),
            GridValue {
                coord: Coordinate { x: -2, y: 2 },
                value: 17
            }
        );

        diag = &Diagonal::TopRight;
        x_incr = diagonal_properties[diag].x_incr;
        y_incr = diagonal_properties[diag].y_incr;
        base_value = diagonal_properties[diag].base_value;
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 0),
            GridValue {
                coord: Coordinate { x: 0, y: 0 },
                value: 1
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 1),
            GridValue {
                coord: Coordinate { x: 1, y: 1 },
                value: 3
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 2),
            GridValue {
                coord: Coordinate { x: 2, y: 2 },
                value: 13
            }
        );

        diag = &Diagonal::BottomLeft;
        x_incr = diagonal_properties[diag].x_incr;
        y_incr = diagonal_properties[diag].y_incr;
        base_value = diagonal_properties[diag].base_value;
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 0),
            GridValue {
                coord: Coordinate { x: 0, y: 0 },
                value: 1
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 1),
            GridValue {
                coord: Coordinate { x: -1, y: -1 },
                value: 7
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 2),
            GridValue {
                coord: Coordinate { x: -2, y: -2 },
                value: 21
            }
        );
        diag = &Diagonal::BottomRight;
        x_incr = diagonal_properties[diag].x_incr;
        y_incr = diagonal_properties[diag].y_incr;
        base_value = diagonal_properties[diag].base_value;
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 0),
            GridValue {
                coord: Coordinate { x: 0, y: 0 },
                value: 1
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 1),
            GridValue {
                coord: Coordinate { x: 1, y: -1 },
                value: 9
            }
        );
        assert_eq!(
            get_diagonal_value(x_incr, y_incr, base_value, 2),
            GridValue {
                coord: Coordinate { x: 2, y: -2 },
                value: 25
            }
        );
    }

    #[test]
    fn test_get_coords() {
        let diagonal_props = &get_diagonal_properties();

        assert_eq!(get_coords(1, diagonal_props), Coordinate { x: 0, y: 0 },);
        assert_eq!(get_coords(2, diagonal_props), Coordinate { x: 1, y: 0 },);
        assert_eq!(get_coords(3, diagonal_props), Coordinate { x: 1, y: 1 },);
        assert_eq!(get_coords(7, diagonal_props), Coordinate { x: -1, y: -1 },);
        assert_eq!(get_coords(9, diagonal_props), Coordinate { x: 1, y: -1 },);
        assert_eq!(get_coords(25, diagonal_props), Coordinate { x: 2, y: -2 },);
        assert_eq!(get_coords(49, diagonal_props), Coordinate { x: 3, y: -3 },);
        assert_eq!(get_coords(281, diagonal_props), Coordinate { x: 0, y: -8 },);
        assert_eq!(get_coords(100, diagonal_props), Coordinate { x: -4, y: 5 },);
    }

    #[test]
    fn test_get_diagonal_ahead() {
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 0, y: 1 }),
            Diagonal::TopLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -1, y: 1 }),
            Diagonal::TopLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 1, y: 2 }),
            Diagonal::TopLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -1, y: 2 }),
            Diagonal::TopLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -2, y: 2 }),
            Diagonal::TopLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 9999, y: 10000 }),
            Diagonal::TopLeft
        );

        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 1, y: 0 }),
            Diagonal::TopRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 1, y: 1 }),
            Diagonal::TopRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 2, y: 1 }),
            Diagonal::TopRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 2, y: -1 }),
            Diagonal::TopRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 2, y: 2 }),
            Diagonal::TopRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 10000, y: -9999 }),
            Diagonal::TopRight
        );

        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -1, y: 0 }),
            Diagonal::BottomLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -1, y: -1 }),
            Diagonal::BottomLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -2, y: 1 }),
            Diagonal::BottomLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -2, y: -2 }),
            Diagonal::BottomLeft
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -10000, y: 9999 }),
            Diagonal::BottomLeft
        );

        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 0, y: -1 }),
            Diagonal::BottomRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: 1, y: -2 }),
            Diagonal::BottomRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -1, y: -2 }),
            Diagonal::BottomRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate { x: -2, y: -3 }),
            Diagonal::BottomRight
        );
        assert_eq!(
            get_diagonal_ahead(&Coordinate {
                x: -9999,
                y: -10000
            }),
            Diagonal::BottomRight
        );
    }

    #[test]
    fn test_get_value_at_coords() {
        let diagonal_props = &get_diagonal_properties();

        assert_eq!(
            get_value_at_coords(&Coordinate { x: 0, y: 0 }, diagonal_props),
            1,
        );
        assert_eq!(
            get_value_at_coords(&Coordinate { x: 1, y: 0 }, diagonal_props),
            2,
        );
        assert_eq!(
            get_value_at_coords(&Coordinate { x: 1, y: 1 }, diagonal_props),
            3,
        );
        assert_eq!(
            get_value_at_coords(&Coordinate { x: 0, y: 1 }, diagonal_props),
            4,
        );

        assert_eq!(
            get_value_at_coords(&Coordinate { x: 1, y: -1 }, diagonal_props),
            9,
        );
        assert_eq!(
            get_value_at_coords(&Coordinate { x: 2, y: -1 }, diagonal_props),
            10,
        );
        assert_eq!(
            get_value_at_coords(&Coordinate { x: 2, y: 0 }, diagonal_props),
            11,
        );
    }
}
