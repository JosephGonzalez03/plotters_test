use std::ops::Add;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    chart.configure_mesh().draw()?;
    let letter: Vec<(f32, f32)> = LetterPart::Crescent(1.0)
                .to_points(&PolarCoordinate::new(5.0, Degree(90.0)))
                .get(0)
                .unwrap()
                .to_vec();

    chart
        .draw_series(LineSeries::new(
            letter,
            BLUE,
        ))?;
    chart
        .draw_series(LineSeries::new(
            arc3_d(
                &PolarCoordinate::new(3.0, Degree(0.0)),
                &PolarCoordinate::new(1.0, Degree(90.0)),
                0.5,
                (Degree(0.0), Degree(360.0))
            ),
            GREEN,
        ))?;
    chart
        .draw_series(LineSeries::new(
            arc(&PolarCoordinate::new(0.0, Degree(0.0)), 5.0, (Degree(0.0), Degree(360.0))),
            RED,
        ))?;
    chart
        .draw_series(LineSeries::new(
            vec![(-0.499,4.975)],
            RED.filled(),
        )
        .point_size(2))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    Ok(())
}

#[derive(Copy, Clone)]
struct Degree(f32);

impl Add for Degree {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add for &Degree {
    type Output = Degree;

    fn add(self, other: Self) -> Degree {
        Degree(self.0 + other.0)
    }
}

struct PolarCoordinate {
    radius: f32,
    angle: Degree,
}

impl PolarCoordinate {
   fn new(radius: f32, angle: Degree) -> Self {
       Self {
           radius,
           angle
       }
   }
}

fn arc3_d(position1: &PolarCoordinate, position2: &PolarCoordinate, radius: f32, range: (Degree, Degree)) -> Vec<(f32,f32)> {
    let start_range = range.0.0.round() as i32;
    let end_range = range.1.0.round() as i32;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range+360)
    }
        .map(|angle| (
            position1.radius * position1.angle.0.to_radians().cos() + position2.radius * position2.angle.0.to_radians().cos() + radius * (angle as f32).to_radians().cos(),
            position1.radius * position1.angle.0.to_radians().sin() + position2.radius * position2.angle.0.to_radians().sin() + radius * (angle as f32).to_radians().sin(),
        ))
        .collect::<Vec<(f32,f32)>>()
}

fn arc(position: &PolarCoordinate, radius: f32, range: (Degree, Degree)) -> Vec<(f32,f32)> {
    let start_range = range.0.0.round() as i32;
    let end_range = range.1.0.round() as i32;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range+360)
    }
        .map(|angle| (
            position.radius * position.angle.0.to_radians().cos() + radius * (angle as f32).to_radians().cos(),
            position.radius * position.angle.0.to_radians().sin() + radius * (angle as f32).to_radians().sin(),
        ))
        .collect::<Vec<(f32,f32)>>()
}

fn dot(position: &PolarCoordinate, orientation: PolarCoordinate) -> Vec<(f32,f32)> {
    vec![
        (
            position.radius * position.angle.0.to_radians().cos() + 1.2 * orientation.radius * orientation.angle.0.to_radians().cos(),
            position.radius * position.angle.0.to_radians().sin() + 1.2 * orientation.radius * orientation.angle.0.to_radians().sin(),
        )
    ]
}

fn normal_line(position: &PolarCoordinate, orientation: PolarCoordinate) -> Vec<(f32,f32)> {
    vec![
        (
            position.radius * position.angle.0.to_radians().cos() + orientation.radius * orientation.angle.0.to_radians().cos(),
            position.radius * position.angle.0.to_radians().sin() + orientation.radius * orientation.angle.0.to_radians().sin(),
        ),
        (
            position.radius * position.angle.0.to_radians().cos() + 1.5 * orientation.radius * orientation.angle.0.to_radians().cos(),
            position.radius * position.angle.0.to_radians().sin() + 1.5 * orientation.radius * orientation.angle.0.to_radians().sin(),
        )
    ]
}

enum LetterPart {
    Vowel(f32, f32, Degree),
    Crescent(f32),
    Full(f32),
    Quarter(f32),
    New(f32),
    Dot1(f32),
    Dot2(f32),
    Dot3(f32),
    Dot4(f32),
    Line1(f32, Degree),
    Line2(f32),
    Line3(f32),
}

impl LetterPart {
    fn to_points(self, position: &PolarCoordinate) -> Vec<Vec<(f32,f32)>> {
        match self {
            LetterPart::Vowel(letter_radius, vowel_radius, vowel_angle) => vec![
                arc3_d(
                    &position,
                    &PolarCoordinate::new(letter_radius, vowel_angle),
                    vowel_radius,
                    (Degree(0.0), Degree(360.0))
                )
            ],
            LetterPart::Crescent(letter_radius) => vec![
                arc3_d(
                    &position,
                    &PolarCoordinate::new(0.90 * letter_radius, position.angle + Degree(180.0)),
                    letter_radius,
                    (position.angle + Degree(30.0), position.angle + Degree(330.0))
                )
            ],
            LetterPart::Full(letter_radius) => vec![
                arc3_d(
                    &position,
                    &PolarCoordinate::new(1.2 * letter_radius, position.angle + Degree(180.0)),
                    letter_radius,
                    (Degree(0.0), Degree(360.0))
                )
            ],
            LetterPart::Quarter(letter_radius) => vec![
                arc3_d(
                    &position,
                    &PolarCoordinate::new(0.0, position.angle + Degree(180.0)),
                    letter_radius,
                    (position.angle + Degree(95.0), position.angle + Degree(265.0))
                )
            ],
            LetterPart::New(letter_radius) => vec![
                arc3_d(
                    &position,
                    &PolarCoordinate::new(0.0, position.angle + Degree(180.0)),
                    letter_radius,
                    (position.angle + Degree(0.0), position.angle + Degree(360.0))
                )
            ],
            LetterPart::Dot1(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(180.0))
                )
            ],
            LetterPart::Dot2(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(135.0))
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(225.0))
                )
            ],
            LetterPart::Dot3(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(135.0))
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(180.0))
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(225.0))
                )
            ],
            LetterPart::Dot4(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(150.0))
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(165.0))
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(195.0))
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(210.0))
                )
            ],
            LetterPart::Line1(letter_radius, orientation) => vec![
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &orientation)
                )
            ],
            LetterPart::Line2(letter_radius) => vec![
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(135.0))
                ),
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(225.0))
                )
            ],
            LetterPart::Line3(letter_radius) => vec![
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(135.0))
                ),
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(180.0))
                ),
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle + &Degree(225.0))
                )
            ],
        }
    }
}

enum GallifreyanLetter {
    A, E, I, O, U,
    B, CH, D, G, H, F,
    J, PH, K, L, C, N, P, M,
    T, WH, SH, R, V, W, S,
    TH, GH, Y, Z, Q, QU, X, NG,
}

impl GallifreyanLetter {
    fn to_letter_parts(&self, position: PolarCoordinate) -> Vec<LetterPart> {
        let letter_radius = position.radius / 3.0;
        match self {
            GallifreyanLetter::A => vec![
                LetterPart::Vowel(letter_radius, letter_radius/3.0, Degree(0.0))
            ],
            GallifreyanLetter::E => vec![
                LetterPart::Vowel(0.0, letter_radius/3.0, Degree(0.0))
            ],
            GallifreyanLetter::I => vec![
                LetterPart::Vowel(0.0, letter_radius/3.0, Degree(0.0)),
                LetterPart::Line1(letter_radius, Degree(180.0))
            ],
            GallifreyanLetter::O => vec![
                LetterPart::Vowel(letter_radius, letter_radius/3.0, Degree(180.0))
            ],
            GallifreyanLetter::U => vec![
                LetterPart::Vowel(0.0, letter_radius/3.0, Degree(0.0)),
                LetterPart::Line1(letter_radius, Degree(0.0))
            ],
            GallifreyanLetter::B => vec![
                LetterPart::Crescent(letter_radius)
            ],
            GallifreyanLetter::CH => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Dot2(letter_radius)
            ],
            GallifreyanLetter::D => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::G => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Line1(letter_radius, Degree(180.0)),
            ],
            GallifreyanLetter::H => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::F => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
            GallifreyanLetter::J => vec![
                LetterPart::Full(letter_radius)
            ],
            GallifreyanLetter::PH => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot1(letter_radius)
            ],
            GallifreyanLetter::K => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot2(letter_radius)
            ],
            GallifreyanLetter::L => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::C => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot4(letter_radius),
            ],
            GallifreyanLetter::N => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Line1(letter_radius, Degree(180.0)),
            ],
            GallifreyanLetter::P => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::M => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
            GallifreyanLetter::T => vec![
                LetterPart::Quarter(letter_radius)
            ],
            GallifreyanLetter::WH => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Dot1(letter_radius)
            ],
            GallifreyanLetter::SH => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Dot2(letter_radius)
            ],
            GallifreyanLetter::R => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::V => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Line1(letter_radius, Degree(180.0)),
            ],
            GallifreyanLetter::W => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::S => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
            GallifreyanLetter::TH => vec![
                LetterPart::New(letter_radius)
            ],
            GallifreyanLetter::GH => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot1(letter_radius)
            ],
            GallifreyanLetter::Y => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot2(letter_radius)
            ],
            GallifreyanLetter::Z => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::Q => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot4(letter_radius),
            ],
            GallifreyanLetter::QU => vec![
                LetterPart::New(letter_radius),
                LetterPart::Line1(letter_radius, Degree(180.0)),
            ],
            GallifreyanLetter::X => vec![
                LetterPart::New(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::NG => vec![
                LetterPart::New(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
        }
    }
}
