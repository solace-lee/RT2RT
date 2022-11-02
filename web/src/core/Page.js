/**
 * @description 页面类，决定这个页面以何种方式加载
 * @author GSSL
 */
import "./style/common.scss";

class Page {
    constructor(is_mobile = false) {
        if (is_mobile) {
            // px转rem需要注入html头样式
            let font_size = document.documentElement.clientWidth / 10;
            document.getElementsByTagName("html")[0].style.fontSize = `${font_size}px`;
        }
    }
}

export default Page;











