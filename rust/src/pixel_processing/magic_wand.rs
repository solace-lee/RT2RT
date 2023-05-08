use std::isize;

use serde::{Deserialize, Serialize};

pub struct Mask<'a> {
    pub data: &'a Vec<isize>,
    pub width: isize,
    pub height: isize,
    pub minx: isize,
    pub miny: isize,
    pub maxx: isize,
    pub maxy: isize,
}

struct PreMask {
    data: Vec<isize>,
    width: isize,
    height: isize,
    offset_x: isize,
    offset_y: isize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contours {
    pub inner: bool,
    pub label: isize,
    pub points: Vec<isize>,
}

fn prepare_mask(mask: Mask) -> PreMask {
    let mut x: isize;
    let mut y: isize;
    let w = mask.width;
    let data = mask.data;
    let minx = mask.minx;
    let maxx = mask.maxx;
    let miny = mask.miny;
    let maxy = mask.maxy;
    let rw = maxx - minx + 3; // bounds size +1 px on each side (a "white" border)
    let rh = maxy - miny + 3;
    let mut result_data = vec![0; (rw * rh) as usize];

    for i in miny..maxy {
        y = i;
        for j in minx..(maxx + 1) {
            x = j;
            if data[(y * w + x) as usize] == 1 {
                result_data[((y - miny + 1) * rw + (x - minx + 1)) as usize] = 1;
            }
        }
    }

    PreMask {
        data: result_data,
        width: rw,
        height: rh,
        offset_x: minx - 1,
        offset_y: miny - 1,
    }
}

pub fn trace_contours(mask: Mask) -> Vec<Contours> {
    const DIRECTIONS: [[isize; 2]; 8] = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ];
    let m = prepare_mask(mask);
    let mut contours: Vec<Contours> = Vec::new();
    let mut label = 0;
    let w = m.width;
    let w2 = w * 2;
    let h = m.height;
    let src = m.data;
    let dx = m.offset_x;
    let dy = m.offset_y;
    let mut dest = src.to_vec();
    let mut i: isize;
    // let mut j: isize;
    let mut x: isize;
    let mut y: isize;
    let mut k: isize;
    let mut k1: isize;
    let mut c: Vec<isize>;
    let mut inner: bool;
    let mut dir: isize;
    let mut first: (isize, isize);
    let mut second: (isize, isize, bool);
    let mut current: (isize, isize);
    let mut previous: (isize, isize);
    let mut next: (isize, isize, bool) = (0, 0, false);
    let mut d: [isize; 2];

    for a in 1..(h - 1) {
        y = a;
        for b in 1..(w - 1) {
            x = b;
            k = y * w + x;

            if src[k as usize] == 1 {
                i = -w;
                while i < w2 {
                    if src[(k + i) as usize] == 0 && dest[(k + i) as usize] == 0 {
                        inner = i == w;
                        label += 1;

                        c = Vec::new();
                        if inner {
                            dir = 2;
                        } else {
                            dir = 6
                        }
                        current = (x, y);
                        previous = (x, y);
                        first = (x, y);
                        second = (0, 0, false);
                        loop {
                            dest[(current.1 * w + current.0) as usize] = label;

                            for _j in 0..8 {
                                dir = (dir + 1) % 8;

                                d = DIRECTIONS[dir as usize];
                                next = (current.0 + d[0], current.1 + d[1], true);

                                k1 = next.1 * w + next.0;
                                if src[k1 as usize] == 1 {
                                    dest[k1 as usize] = label;
                                    break;
                                }
                                dest[k1 as usize] = -1;
                                next.2 = false
                            }
                            if next.2 == false {
                                break;
                            }
                            current.0 = next.0;
                            current.1 = next.1;
                            if second.2 {
                                if previous.0 == first.0
                                    && previous.1 == first.1
                                    && current.0 == second.0
                                    && current.1 == second.1
                                {
                                    break;
                                }
                            } else {
                                second = next;
                            }
                            c.push(previous.0 + dx);
                            c.push(previous.1 + dy);
                            previous = current;
                            dir = (dir + 4) % 8;
                        }

                        if next.2 {
                            c.push(first.0 + dx);
                            c.push(first.1 + dy);
                            contours.push(Contours {
                                inner,
                                label,
                                points: c,
                            });
                        }
                    }

                    i += w2;
                }
            }
        }
    }
    contours
}

pub fn simplify_contours (contours: Vec<Contours>, simplifyTolerant: f64, simplifyCount: f64) -> Vec<Contours> {
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;
    let mut x: f64;
    let mut y: f64;
    let mut x0: f64;
    let mut y0: f64;
    let mut x1: f64;
    let mut y1: f64;
    let mut x2: f64;
    let mut y2: f64;
    let mut d: f64;
    let mut dmax: f64;
    let mut index: usize;
    let mut points: Vec<f64>;
    let mut contour: Contours;
    let mut simplified: Vec<f64>;
    let mut len: usize;
    let mut count: usize;
    let mut tolerance: f64 = simplifyTolerant;
    let mut maxCount: usize = simplifyCount as usize;

    for i in 0..contours.len() {
        contour = contours[i];
        points = contour.points;
        simplified = Vec::new();
        len = points.len();
        if len < 6 {
            continue;
        }
        count = maxCount;
        if count > len / 2 {
            count = len / 2;
        }
        if count < 2 {
            continue;
        }
        if tolerance <= 0.0 {
            tolerance = 0.01;
        }
        while count > 2 {
            dmax = 0.0;
            index = 0;
            x0 = points[0];
            y0 = points[1];
            x1 = points[len - 2];
            y1 = points[len - 1];
            for j in 2..(len - 3) {
                k = j + 1;
                x2 = points[k];
                y2 = points[k + 1];
                d = distance(x0, y0, x1, y1, x2, y2);
                if d > dmax {
                    index = j;
                    dmax = d;
                }
            }
            if dmax > tolerance {
                simplified.push(points[index]);
                simplified.push(points[index + 1]);
                points.remove(index);
                points.remove(index);
                len -= 2;
                count -= 1;
            } else {
                break;
            }
        }
        simplified.push(points[0]);
        simplified.push(points[1]);
        simplified.push(points[len - 2]);
        simplified.push(points[len - 1]);
        contour.points = simplified;
        contours[i] = contour;
    }
    contours
    
}
