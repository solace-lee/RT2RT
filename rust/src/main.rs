extern crate serde_json;

use std::time::SystemTime;
mod init_data;
mod output_json;
mod pixel_processing;

use crate::pixel_processing::build_xy_rt::{generate_mask, mask_to_rt};

use crate::pixel_processing::scan_line::scan_line;
use crate::{
    init_data::{
        calc_rt_bounds::get_rt_pxdata_and_bounds,
        init_json,
    },
    output_json::output::output,
};

fn main() {
    // 读取json数据
    let result = init_json::ImageInfo::new("./json/RT_fmt.json").expect("出现错误");
    // println!("格式化数据：{:#?}", result.image_position_matrix);

    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&result);
    
    // output(&rt_pxdata_and_bounds, "./json/line_result.json");
    let sys_time1 = SystemTime::now();
    // 扫描线算法
    let mask_volume = scan_line(rt_pxdata_and_bounds, &result);
    let sys_time2 = SystemTime::now();
    println!(
      "扫描线算法耗时：{:?}",
      sys_time2.duration_since(sys_time1).expect("时间倒转了")
    );


    let sys_time1 = SystemTime::now();
    // 生成切面mask轮廓
    let rt_build_mask = generate_mask(&mask_volume, &result);
    let sys_time2 = SystemTime::now();
    println!(
        "切面mask耗时：{:?}",
        sys_time2.duration_since(sys_time1).expect("时间倒转了")
    );
    println!("mask长度为：{:#?}", rt_build_mask.x_rt[0].len());

    let sys_time1 = SystemTime::now();
    // 轮廓提取
    let rt_build_result = mask_to_rt(rt_build_mask, mask_volume,&result);
    let sys_time2 = SystemTime::now();
    println!(
        "轮廓提取耗时：{:?}",
        sys_time2.duration_since(sys_time1).expect("时间倒转了")
    );

    output(&rt_build_result, "./json/rt_coord_result.json");
}
