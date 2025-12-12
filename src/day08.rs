const INPUT_SAMPLE: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[derive(Debug, Clone)]
struct JunctionBox {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl JunctionBox {
    pub fn distance_from(&self, cmp: &JunctionBox) -> f32 {
        (((self.x - cmp.x).pow(2) + (self.y - cmp.y).pow(2) + (self.z - cmp.z).pow(2)) as f32)
            .sqrt()
    }
}

fn parse_junction_box(input: &str) -> JunctionBox {
    let values: Vec<&str> = input.split(",").collect();

    JunctionBox {
        x: values.get(0).unwrap().parse().unwrap_or(0),
        y: values.get(1).unwrap().parse().unwrap_or(0),
        z: values.get(2).unwrap().parse().unwrap_or(0),
    }
}

#[derive(Debug, Clone)]
struct Pair {
    a: usize,
    b: usize,
    distance: f32,
}

impl Pair {
    pub fn from(a: usize, b: usize, distance: f32) -> Self {
        Self { a, b, distance }
    }
}

#[derive(Debug)]
struct Circuit(Vec<Pair>);

impl Circuit {
    fn new(pair: Pair) -> Self {
        let list = vec![pair];
        Self(list)
    }
    // Return the pair if it is not added and not a duplicate
    fn add(&mut self, pair: &Pair) -> bool {
        // If one of the junction boxes in the pair is in the ciruit then add it
        if let Some(p) = &self.0.last() {
            if (p.a == pair.a && p.b != pair.b) || (p.a != pair.a && p.b == pair.b) {
                self.0.push(pair.clone());
                return true;
            } else if (p.a == pair.a && p.b == pair.b) || (p.a == pair.b && p.b == pair.a) {
                return true;
            }
        }

        false
    }
}

pub fn run() {
    let boxes: Vec<JunctionBox> = INPUT_SAMPLE
        .split("\n")
        .map(|row| parse_junction_box(row))
        .collect();

    let mut pairs = vec![];

    for (index_a, a) in boxes.iter().enumerate() {
        for (index_b, b) in boxes.iter().enumerate() {
            if index_a == index_b {
                break;
            }

            pairs.push(Pair::from(index_a, index_b, a.distance_from(b)));
        }
    }

    pairs.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut circuits: Vec<Circuit> = vec![];
    let mut insert_new_circuit: bool = true;

    for pair in pairs {
        for circuit in &mut circuits {
            if circuit.add(&pair) {
                insert_new_circuit = false;
            }
        }

        if insert_new_circuit {
            let circuit = Circuit::new(pair);
            circuits.push(circuit);
        }
    }

    println!("circuits: {circuits:#?}");
}
