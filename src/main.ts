import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { useThemeColor } from "./composables/useThemeColor";
import "./styles/variables.css";

// 初始化主题色系（默认珊瑚橙）
const { loadPrimaryColor } = useThemeColor()
loadPrimaryColor()

const app = createApp(App);
app.use(router);
app.mount("#app");
