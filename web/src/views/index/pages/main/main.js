import { Contour } from "@/core/Contour";
import { ref, onMounted } from "vue";
import * as data from "@/asset/data/data.json";
import RT_px_data from "@/asset/data/RT_px_data.json";

export default {
  name: "main",
  setup(props, context) {
    const refs = ref(null);

    onMounted(() => {
      // 获取二维图形的绘图上下文
      const canvas = refs.value;
      const contour = new Contour(canvas);
      console.log(data, "draw");
      const arr = []
      RT_px_data['3'][0].forEach(element => {
        arr.push((element.x + 113) / 430)
        arr.push((element.y + 160) / 319)
      });
      console.log(arr);
      contour.draw(new Float32Array(arr));
    });

    return {
      refs
    };
  }
};















