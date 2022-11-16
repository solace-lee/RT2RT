extern crate serde_json;

use rt2rt::init_data::{
    calc_rt_bounds::{find_pixel_spacing, get_rt_pxdata_and_bounds, get_volume_bounds},
    init_json,
};
use rt2rt::pixel_processing::line_processing::closed_line;

fn main() {
    let result = init_json::ImageInfo::new("./src/RT_fmt.json").expect("出现错误");
    // println!("result{:?}", result);

    let min_spacing = find_pixel_spacing(vec![
        result.row_pixel_spacing,
        result.column_pixel_spacing,
        result.thickness,
    ]);
    println!("寻找最小的像素间距:{}", min_spacing);

    let volume_bounds = get_volume_bounds(&result, &min_spacing);
    println!("volume边界为：{:?}", volume_bounds);

    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&result);
    println!("轮廓的边界为：{:?}", rt_pxdata_and_bounds.bounds);

    let closed_result = closed_line(rt_pxdata_and_bounds);
    println!("轮廓的边界为：{:#?}", closed_result.data);
}
