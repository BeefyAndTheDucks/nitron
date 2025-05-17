use renderer::types::Vert;

pub const VERTICES: [Vert; 4] = [
    Vert {
        position: [ 10.0,  10.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
    Vert {
        position: [-10.0,  10.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
    Vert {
        position: [-10.0, -10.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
    Vert {
        position: [ 10.0, -10.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
];

pub const INDICES: [u32; 6] = [
    0, 1, 2,
    2, 3, 0
];