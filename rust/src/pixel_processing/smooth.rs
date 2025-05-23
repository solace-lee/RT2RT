use super::magic_wand::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Vec4Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub old_y: f64,
}

#[derive(Debug)]
struct BoundingBox {
    max: Point,
    min: Point,
}

#[derive(Debug)]
pub struct Options {
    pub radius: f64,
}

fn calculate_avg_point(group: &[Point], bounding_box: &BoundingBox) -> Point {
    let mut result = Point { x: 0.0, y: 0.0 };
    let mut limit_x = None;
    let mut limit_y = None;

    for point in group {
        result.x += point.x;
        result.y += point.y;

        if point.x == bounding_box.max.x {
            limit_x = Some(bounding_box.max.x);
        }
        if point.x == bounding_box.min.x {
            limit_x = Some(bounding_box.min.x);
        }
        if point.y == bounding_box.max.y {
            limit_y = Some(bounding_box.max.y);
        }
        if point.y == bounding_box.min.y {
            limit_y = Some(bounding_box.min.y);
        }
    }

    Point {
        x: limit_x.unwrap_or(result.x / group.len() as f64),
        y: limit_y.unwrap_or(result.y / group.len() as f64),
    }
}

fn calculate_2d_bounding_box(contour: &[Point]) -> BoundingBox {
    if contour.is_empty() {
        panic!("Contour should not be empty");
    }

    let mut bb = BoundingBox {
        max: contour[0].clone(),
        min: contour[0].clone(),
    };

    for point in &contour[1..] {
        if bb.max.x < point.x {
            bb.max.x = point.x;
        }
        if bb.max.y < point.y {
            bb.max.y = point.y;
        }
        if bb.min.x > point.x {
            bb.min.x = point.x;
        }
        if bb.min.y > point.y {
            bb.min.y = point.y;
        }
    }

    bb
}

fn calculate_tow_points_dist(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub fn smooth_by_radius(contour: &[Point], opts: &Options) -> Vec<Point> {
    if contour.len() < 3 {
        return contour.to_vec();
    }

    let bb = calculate_2d_bounding_box(contour);
    let mut radius = opts.radius;
    if radius < 1.0 {
        radius = 1.0;
    }

    let mut result = Vec::new();
    let mut curr = 0;
    let mut next = 1;
    let prev = contour.len() - 1;
    let mut is_run = true;

    while is_run {
        let mut group = vec![contour[curr].clone()];

        while let Some(point_next) = contour.get(next) {
            let dist_next = calculate_tow_points_dist(&contour[curr], &point_next);
            let is_next_over = dist_next > radius;

            if !is_next_over {
                group.push(point_next.clone());
                next += 1;
            } else {
                break;
            }

            if next > prev {
                is_run = false;
                break;
            }
        }

        // 求平均值
        result.push(calculate_avg_point(&group, &bb));
        curr = next;
    }

    if result.len() < 3 {
        return contour.to_vec();
    }

    result
}

pub fn contour_smooth_by_level(contour: &Vec<Vec4Point>, level: f64) -> Vec<Point> {
    let p = 3;
    let n = 6;
    let radius_points = contour;

    // NURBS 中顶点必须大于阶次数
    if radius_points.len() < 3 || radius_points.len() < p {
        return radius_points
            .into_iter()
            .map(|p| Point { x: p.x, y: p.y })
            .collect();
    }

    let mut _d = Vec::new();
    for point in radius_points {
        _d.push([point.x, point.y]);
    }

    let amount = n * _d.len();
    // 首尾不相连
    let l = _d.len() - 1;
    if _d[0][0] == _d[l][0] && _d[0][1] == _d[l][1] {
        _d.pop();
    }

    let _r = get_uniform_bspline_curve(&_d, p, amount, true);

    if true {
        let mut result = Vec::new();
        for p_old in &_r {
            result.push(Point {
                x: p_old[0],
                y: p_old[1],
            });
        }
        result
    } else {
        _r.into_iter().map(|p| Point { x: p[0], y: p[1] }).collect()
    }
}

fn get_uniform_bspline_curve(
    points: &Vec<[f64; 2]>,
    p: usize,
    amount: usize,
    closed: bool,
) -> Vec<[f64; 2]> {
    let mut points_new = points.clone();

    if closed {
        for i in 0..p {
            points_new.push(points_new[i].clone());
        }
    }

    let u = get_uniform_knots(&points_new, p);
    let mut line = Vec::new();

    // const u_max = U[U.length - 1 - p];
    // const u_min = U[p];
    let u_max = u[u.len() - 1 - p];
    let u_min = u[p];
    let u_sub = u_max - u_min;

    for i in 0..amount {
        let u_val = (1.0 / (amount as f64 - 1.0) * i as f64) * u_sub + u_min;
        line.push(curve_point(p, &u, &points_new, u_val));
    }

    line
}

fn get_uniform_knots(points: &Vec<[f64; 2]>, p: usize) -> Vec<f64> {
    let n = points.len();
    let m = n + p + 1;
    let mut u = vec![0.0; m];
    let loop_n = (m - 1) as f64;
    for i in 0..m {
        u[i] = i as f64 / loop_n;
    }
    u
}

fn curve_point(p: usize, u: &Vec<f64>, points: &Vec<[f64; 2]>, u_val: f64) -> [f64; 2] {
    let span = find_span(p, u_val, u);
    let n = basis_funs(span, u_val, p, u);
    let mut c = points[span - p].map(|v| v * n[0]);
    for i in 1..=p {
      let a = points[span - p + i].map(|v| v * n[i]);
      c = [c[0] + a[0], c[1] + a[1]];
    }
    c
}

fn basis_funs(span: usize, u_val: f64, p: usize, u: &[f64]) -> Vec<f64> {
  let mut n = vec![1.0];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    for j in 1..=p {
        left[j] = u_val - u[span + 1 - j];
        right[j] = u[span + j] - u_val;
        let mut saved = 0.0;
        for r in 0..j {
            let a = right[r + 1] + left[j - r];
            let temp = if a == 0.0 { 0.0 } else { n[r] / a };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n.push(saved);
    }
    n
}

fn find_span(p: usize, u_val: f64, u: &[f64]) -> usize {
    let n = u.len() - p - 1;
    if u_val >= u[n] {
        return n - 1;
    }
    if u_val <= u[p] {
        return p;
    }

    let mut low = p;
    let mut high = n + 1;
    let mut mid = ((low + high) as f64 / 2.0).floor() as usize;
    while u_val < u[mid] || u_val >= u[mid + 1] {
        if u_val < u[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = ((low + high) as f64 / 2.0).floor() as usize;
    }
    mid
}
