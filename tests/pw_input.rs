extern crate qe;

use qe::pw::input;
use qe::pw::serialize;

#[test]
fn generate_pw_input() {
    let calculation = input::Calculation::Scf { conv_thr: 1e-8 };

    let control = input::Control {
        restart_mode: None,
        disk_io: Some(input::DiskIO::Low),
        wf_collect: None,
        pseudo_dir: None,
        out_dir: None,
        prefix: None,
    };

    let cell = input::Cell {
        units: input::LatticeUnits::Alat,
        cell: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    let system = input::System {
        ibrav: input::Ibrav::Free(cell),
        alat: 3.0,
        ecutwfc: 60.0,
        ecutrho: 240.0,
        occupations: input::Occupations::Tetrahedra,
        spin_type: None,
    };

    let efield = None;

    let electrons = input::Electrons {
        startingwfc: None,
        diagonalization: None,
    };

    let species = vec![
        input::Species {
            label: String::from("Fe"),
            mass: 55.845,
            pseudopotential_filename: String::from("Fe.UPF"),
        },
    ];

    let atomic_positions = input::Positions {
        coordinate_type: input::PositionCoordinateType::Crystal,
        coordinates: vec![
            input::AtomCoordinate {
                species: String::from("Fe"),
                r: [0.0, 0.0, 0.0],
                if_pos: None,
            },
        ],
    };

    let k_points = input::KPoints::Automatic {
        nk: [8, 8, 8],
        sk: None,
    };

    let test_input = input::Input {
        calculation,
        control,
        system,
        efield,
        electrons,
        species,
        atomic_positions,
        k_points,
    };

    let input_text = serialize::make_input_file(&test_input).unwrap();

    println!("{}", input_text);
}
