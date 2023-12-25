import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import "./styles.css";
import App from "./App.vue";
import LoginRoute from "./pages/LoginRoute.vue";
import UserRoute from "./pages/UserRoute.vue";

const routes = [
    { path: '/', component: LoginRoute },
    { path: '/user', component: UserRoute }
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes
})

const app = createApp(App);
app.use(router);
app.mount("#app");
