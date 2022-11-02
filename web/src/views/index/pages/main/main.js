import { Contour } from "@/core/Contour";
import { ref, onMounted } from "vue";
import * as data from "@/asset/data/data.json";

export default {
    name: "main",
    setup(props, context){
        const refs = ref(null);

        onMounted(() => {
            // 获取二维图形的绘图上下文
            const canvas = refs.value;
            const contour = new Contour(canvas);
            console.log(data, "draw");
            contour.draw(new Float32Array(data));
        });

        return {
            refs
        };
    }
};















