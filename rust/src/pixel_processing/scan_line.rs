use crate::init_data::calc_rt_bounds::{BoundsLimit, PixelCoods, PxData};

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct TagEdge {
    pub xi: f32,
    pub dx: f32,
    pub ymax: isize,
    pub id: usize,
}

pub struct NextEdge {
    next: Vec<isize>,
    head: isize,
}

pub struct AET {
    sl_net: Vec<Vec<TagEdge>>,
    lines: Rc<RefCell<Vec<TagEdge>>>,
}

pub fn scan_line(rs: PxData) -> Vec<Vec<PixelCoods>> {
    let PxData {
        data,
        // bounds,
        layer_bounds,
        ..
    } = rs;

    let mut result: Vec<Vec<PixelCoods>> = Vec::new();
    for index in 0..data.len() {
        let layer = &data[index];
        let item_bounds = &layer_bounds[index];

        // 归档活动边表
        let aet = init_net(layer, item_bounds);
        let line_result = process_scan_line_fill(aet, item_bounds);
        result.push(line_result);
        // println!("第{}层, 共{}层", index, data.len())
    }
    return result;
}

// 初始化新边表
fn init_net(layer_coords: &Vec<Vec<PixelCoods>>, item_bounds: &BoundsLimit) -> AET {
    let mut aet = AET {
        sl_net: vec![Vec::new(); (item_bounds.max_y - item_bounds.min_y) as usize],
        lines: Rc::new(RefCell::new(Vec::new())),
    };
    let mut count = 0;
    for i in 0..layer_coords.len() {
        // 单个轮廓(同层)
        let coords_item = &layer_coords[i];
        for coord_index in 0..coords_item.len() {
            let start = &coords_item[coord_index];
            let end = &coords_item[(coord_index + 1) % coords_item.len()];
            let start_pre = &coords_item
                [(coord_index as i32 - 1 + coords_item.len() as i32) as usize % coords_item.len()];
            let end_next = &coords_item[(coord_index + 2) % coords_item.len()];
            let mut e = TagEdge {
                xi: 0.0,
                dx: 0.0,
                ymax: 0,
                id: count,
            };
            if end.y != start.y {
                e.dx = (end.x - start.x) as f32 / (end.y - start.y) as f32;
                if end.y > start.y {
                    e.xi = start.x as f32;
                    if end_next.y >= end.y {
                        e.ymax = (end.y - 1) as isize;
                    } else {
                        e.ymax = end.y as isize
                    }
                    aet.sl_net[(start.y - item_bounds.min_y) as usize].push(e);
                } else {
                    e.xi = end.x as f32;
                    if start_pre.y >= start.y {
                        e.ymax = (start.y - 1) as isize;
                    } else {
                        e.ymax = start.y as isize;
                    }
                    aet.sl_net[(end.y - item_bounds.min_y) as usize].push(e);
                }
            }
            aet.lines.borrow_mut().push(e);
            count += 1;
        }
    }

    return aet;
}

fn process_scan_line_fill(
    AET { sl_net, lines }: AET,
    item_bounds: &BoundsLimit,
) -> Vec<PixelCoods> {
    let next_edge = Rc::new(RefCell::new(NextEdge {
        next: vec![-1; lines.borrow().len()],
        head: -1,
    }));

    let mut line_result: Vec<PixelCoods> = Vec::new();

    let insert = |y: i32| {
        for i in 0..sl_net[y as usize].len() {
            let temp = sl_net[y as usize][i];

            if temp.ymax == 0 && temp.dx == 0.0 {
                break;
            }

            if next_edge.borrow().head == -1 {
                next_edge.borrow_mut().head = temp.id as isize;
            } else {
                if temp.xi < lines.borrow()[next_edge.borrow().head as usize].xi {
                    let mut current_edge = next_edge.borrow_mut();
                    current_edge.next[temp.id] = current_edge.head as isize;
                    current_edge.head = temp.id as isize;
                } else {
                    let mut pre = next_edge.borrow().head;
                    let mut j = next_edge.borrow().next[next_edge.borrow().head as usize];
                    loop {
                        if j == -1 || temp.xi < lines.borrow()[j as usize].xi {
                            next_edge.borrow_mut().next[pre as usize] = temp.id as isize;
                            next_edge.borrow_mut().next[temp.id] = j;
                            break;
                        }
                        pre = j;
                        j = next_edge.borrow_mut().next[j as usize];
                    }
                }
            }
        }
        // println!("Next表：{:?}, {:?}", next, head);
    };

    let remove = |y: i32| {
        // let next_edge_clone = Rc::clone(&next_edge);
        // let mut current_edge = RefCell::borrow_mut(&next_edge_clone);
        let mut current_edge = next_edge.borrow_mut();
        let mut pre = current_edge.head;

        while current_edge.head != -1
            && lines.borrow()[current_edge.head as usize].ymax == y as isize
        {
            current_edge.head = current_edge.next[current_edge.head as usize];
            current_edge.next[pre as usize] = -1;
            pre = current_edge.head;
        }

        if current_edge.head == -1 {
            return;
        }

        let mut nxt = current_edge.next[current_edge.head as usize];
        let mut i = nxt;
        loop {
            if i == -1 {
                break;
            }
            nxt = current_edge.next[i as usize];
            if lines.borrow()[i as usize].ymax == y as isize {
                current_edge.next[pre as usize] = current_edge.next[i as usize];
                current_edge.next[i as usize] = -1
            } else {
                pre = i
            }
            i = nxt;
        }
    };

    let update_aet = || {
        let mut current_edge = next_edge.borrow_mut();
        let mut i = current_edge.head;
        let mut current_lines = lines.borrow_mut();
        loop {
            if i == -1 {
                break;
            }
            current_lines[i as usize].xi += current_lines[i as usize].dx;
            i = current_edge.next[i as usize];
        }

        if current_edge.head == -1 {
            return;
        }
        if current_edge.next[current_edge.head as usize] == -1 {
            return;
        }

        let mut pre = current_edge.head;
        if current_lines[current_edge.head as usize].xi
            > current_lines[current_edge.next[current_edge.head as usize] as usize].xi
        {
            current_edge.head = current_edge.next[current_edge.head as usize];
            current_edge.next[pre as usize] = current_edge.next[current_edge.head as usize];
            let current_head = current_edge.head;
            current_edge.next[current_head as usize] = pre;
            pre = current_edge.head;
        }
        let mut j = current_edge.next[current_edge.head as usize];
        let mut i = j;
        loop {
            if i == -1 {
                break;
            } else {
                j = current_edge.next[i as usize];
                if j == -1 {
                    break;
                }
                if current_lines[i as usize].xi
                    > current_lines[current_edge.next[i as usize] as usize].xi
                {
                    current_edge.next[pre as usize] = current_edge.next[i as usize];
                    current_edge.next[i as usize] =
                        current_edge.next[current_edge.next[i as usize] as usize];
                    current_edge.next[j as usize] = i;
                } else {
                    pre = i;
                }
            }
            i = j;
        }
    };

    for y in item_bounds.min_y..item_bounds.max_y {
        // 行
        let index = y - item_bounds.min_y;
        // 闭包
        insert(index);
        // 输出结果
        {
            let current_edge = next_edge.borrow();
            let mut i = current_edge.head;
            loop {
                if i == -1 {
                    break;
                };
                if current_edge.next[i as usize] != -1 {
                    line_result.push(PixelCoods {
                        x: lines.borrow()[i as usize].xi as i32,
                        y,
                    });
                    line_result.push(PixelCoods {
                        x: lines.borrow()[current_edge.next[i as usize] as usize].xi as i32,
                        y,
                    });
                    // println!(
                    //     "连线{:?}, {:?}",
                    //     PixelCoods {
                    //         x: lines.borrow()[i as usize].xi as i32,
                    //         y
                    //     },
                    //     PixelCoods {
                    //         x: lines.borrow()[current_edge.next[i as usize] as usize].xi as i32,
                    //         y
                    //     }
                    // );
                }
                if current_edge.next[i as usize] == -1 {
                    break;
                }
                i = current_edge.next[current_edge.next[i as usize] as usize];
            }
        }

        // 删除非活动边
        remove(y);
        update_aet();
    }

    return line_result;
}
