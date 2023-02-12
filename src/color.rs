const XYZ_X_REFERENCE: f32 = 95.047;
const XYZ_Y_REFERENCE: f32 = 100.0;
const XYZ_Z_REFERENCE: f32 = 108.883;


pub fn rgb2xyz(rgb: &[u8; 3]) -> [f32; 3] {
    let rgb = [
        rgb[0] as f32 / 255.0,
        rgb[1] as f32 / 255.0,
        rgb[2] as f32 / 255.0
    ];

    let r = (if rgb[0] > 0.04045 {((rgb[0] + 0.055) / 1.055).powf(2.4)} else {rgb[0] / 12.92}) * 100.0;
    let g = (if rgb[1] > 0.04045 {((rgb[1] + 0.055) / 1.055).powf(2.4)} else {rgb[1] / 12.92}) * 100.0;
    let b = (if rgb[2] > 0.04045 {((rgb[2] + 0.055) / 1.055).powf(2.4)} else {rgb[2] / 12.92}) * 100.0;

    return [
        r * 0.4124 + g * 0.3576 + b * 0.1805,
        r * 0.2126 + g * 0.7152 + b * 0.0722,
        r * 0.0193 + g * 0.1192 + b * 0.9505
    ];
}

pub fn xyz2lab(xyz: &[f32; 3]) -> [f32; 3] {
    let new_x = xyz[0] / XYZ_X_REFERENCE;
    let new_y = xyz[1] / XYZ_Y_REFERENCE;
    let new_z = xyz[2] / XYZ_Z_REFERENCE;

    let l = if new_y > 0.008856 {116.0 * new_y.powf(1.0 / 3.0) - 16.0} else {903.3 * new_y};

    let new_x = if new_x > 0.008856 { new_x.powf(1. / 3.)} else {7.787 * new_x + 16. / 116.};
    let new_z = if new_z > 0.008856 { new_z.powf(1. / 3.)} else {7.787 * new_z + 16. / 116.};
    let new_y = if new_y > 0.008856 { new_y.powf(1. / 3.)} else {7.787 * new_y + 16. / 116.};

    [
        l,
        500. * (new_x - new_y),
        200. * (new_y - new_z),
    ]

}

pub fn rgb2lab(rgb: &[u8; 3]) -> [f32; 3] {
    let xyz = rgb2xyz(rgb);
    xyz2lab(&xyz)
}

pub fn lab2xyz(lab: &[f32; 3]) -> [f32; 3] {
    let p = (lab[0] + 16.) / 116.;

    [
        XYZ_X_REFERENCE * (p + lab[1] / 500.).powf(3.0),
        XYZ_Y_REFERENCE * p.powf(3.0),
        XYZ_Z_REFERENCE * (p - lab[2] / 200.).powf(3.0),
    ]
}

pub fn xyz2rgb(xyz: &[f32; 3]) -> [u8; 3] {
    let x = xyz[0] / 100.0;
    let y = xyz[1] / 100.0;
    let z = xyz[2] / 100.0;

    let r = x * 3.2406 + y * -1.5372 + z * -0.4986;
    let g = x * -0.9689 + y * 1.8758 + z * 0.0415;
    let b = x * 0.0557 + y * -0.2040 + z * 1.0570;

    let r = if r > 0.0031308 {1.055 * r.powf(1. / 2.4) - 0.055} else {12.92 * r};
    let g = if g > 0.0031308 {1.055 * g.powf(1. / 2.4) - 0.055} else {12.92 * g};
    let b = if b > 0.0031308 {1.055 * b.powf(1. / 2.4) - 0.055} else {12.92 * b};


    [
        255f32.min((r * 255.0).max(0.0) as f32) as u8,
        255f32.min((g * 255.0).max(0.0) as f32) as u8,
        255f32.min((b * 255.0).max(0.0) as f32) as u8,
    ]
}

pub fn lab2rgb(lab: &[f32; 3]) -> [u8; 3] {
    let xyz = lab2xyz(lab);
    xyz2rgb(&xyz)
}