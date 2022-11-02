/**
 * Author: hezhiwei
 * Create Time: 2019-06-09 00:28
 * Description:
 */

import { createApp } from "vue";
import App from "./pages/app.vue";
import router from "./route";

const app = createApp(App);
app.use(router);
app.mount('#root');












