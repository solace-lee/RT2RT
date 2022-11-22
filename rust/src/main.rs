extern crate serde_json;

use rt2rt::{init_data::{
    calc_rt_bounds::{find_pixel_spacing, get_rt_pxdata_and_bounds, get_volume_bounds},
    init_json,
}, output_json::output::output};
use rt2rt::pixel_processing::line_processing::closed_line;

fn main() {
    // 读取json数据
    let result = init_json::ImageInfo::new("./json/RT_fmt.json").expect("出现错误");

    // 获取最小的像素间距
    let min_spacing = find_pixel_spacing(vec![
        result.row_pixel_spacing,
        result.column_pixel_spacing,
        result.thickness,
    ]);
    println!("寻找最小的像素间距:{}", min_spacing);

    // 获取体数据的边界
    let volume_bounds = get_volume_bounds(&result, &min_spacing);
    println!("volume边界为：{:?}", volume_bounds);

    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&result);
    println!("轮廓的边界为：{:#?}", rt_pxdata_and_bounds.bounds);

    // 生成闭合轮廓
    let closed_result = closed_line(rt_pxdata_and_bounds);
    // println!("闭合轮廓为：{:#?}", closed_result.data);

    // 将闭合轮廓存入本地
    output(&closed_result, "./json/closed_result.json");
}
