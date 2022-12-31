extern crate serde_json;

use rt2rt::pixel_processing::line_processing::closed_line;
use rt2rt::pixel_processing::scan_line::scan_line;
use rt2rt::volume_tools::volume::volume;
use rt2rt::{
    init_data::{
        calc_rt_bounds::{get_rt_pxdata_and_bounds, get_volume_bounds, PixelCoods},
        init_json,
    },
    output_json::output::output,
};

fn main() {
    // 读取json数据
    let result = init_json::ImageInfo::new("./json/RT_fmt.json").expect("出现错误");

    // 获取最小的像素间距
    // let min_spacing = find_pixel_spacing(vec![
    //     result.row_pixel_spacing,
    //     result.column_pixel_spacing,
    //     result.thickness,
    // ]);
    // println!(
    //     "寻找最小的像素间距:{}, 层厚：{}",
    //     min_spacing, result.thickness
    // );

    // 获取体数据的边界
    let volume_bounds = get_volume_bounds(&result);
    println!("volume边界为：{:?}", volume_bounds);

    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&result);
    // println!("轮廓的边界为：{:#?}", rt_pxdata_and_bounds.data[1]);
    // println!("轮廓的layer_bounds：{:#?}", rt_pxdata_and_bounds.layer_bounds[1]);

    let line_result = scan_line(rt_pxdata_and_bounds);
    output(&line_result, "./json/line_result.json");

    return;

    // 计算XY的偏移量
    let mut translation = PixelCoods { x: 0, y: 0 };
    if rt_pxdata_and_bounds.bounds.min_x < 0 {
        translation.x = rt_pxdata_and_bounds.bounds.min_x.abs();
    }
    if rt_pxdata_and_bounds.bounds.min_y < 0 {
        translation.y = rt_pxdata_and_bounds.bounds.min_y.abs();
    }

    // 初始化体数据空间
    let volume = volume::Volume::new(volume_bounds);
    println!("体实例：{:#?}", volume.bounds);

    // 生成闭合轮廓
    // 将轮廓放入体数据中
    let closed_result = closed_line(rt_pxdata_and_bounds, translation, &volume);
    // println!("闭合轮廓为：{:#?}", &volume_data.get_layer_data(3));

    // 将闭合轮廓存入本地

    // 建立层数和Z轴坐标的映射关系（TODO：）

    // 逐行扫描生成实心轮廓的体数据

    // 截取矢状面各层像素数据

    // 截取冠状面各层像素数据
}
