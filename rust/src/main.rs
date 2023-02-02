extern crate serde_json;

use std::time::SystemTime;

use rt2rt::pixel_processing::build_xy_rt::{generate_mask, mask_to_rt};
// use rt2rt::pixel_processing::line_processing::closed_line;
use rt2rt::pixel_processing::scan_line::scan_line;
use rt2rt::{
    init_data::{
        calc_rt_bounds::{get_rt_pxdata_and_bounds, get_volume_bounds},
        init_json,
    },
    output_json::output::output,
};

fn main() {
    // 读取json数据
    let result = init_json::ImageInfo::new("./json/RT_fmt.json").expect("出现错误");

    // 获取体数据的边界
    let volume_bounds = get_volume_bounds(&result);
    println!("volume边界为：{:?}", volume_bounds);

    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&result);
    println!("轮廓的边界为：{:#?}", rt_pxdata_and_bounds.bounds);

    let sys_time1 = SystemTime::now();
    // 扫描线算法
    let line_result = scan_line(rt_pxdata_and_bounds);
    let sys_time2 = SystemTime::now();
    println!(
        "扫描线算法耗时：{:?}",
        sys_time2.duration_since(sys_time1).expect("时间倒转了")
    );

    // output(&line_result, "./json/line_result.json");

    let sys_time1 = SystemTime::now();
    // 生成切面mask轮廓
    let rt_build_mask = generate_mask(line_result, &volume_bounds);
    let sys_time2 = SystemTime::now();
    println!(
        "切面mask耗时：{:?}",
        sys_time2.duration_since(sys_time1).expect("时间倒转了")
    );

    let sys_time1 = SystemTime::now();
    // 轮廓提取
    let rt_build_result = mask_to_rt(rt_build_mask, &volume_bounds);
    let sys_time2 = SystemTime::now();
    println!(
        "轮廓提取耗时：{:?}",
        sys_time2.duration_since(sys_time1).expect("时间倒转了")
    );

    output(&rt_build_result, "./json/rt_coord_result.json");
}
