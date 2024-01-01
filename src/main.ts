import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import "./main.css";
import App from "./App.vue";
import IndexPage from "./pages/IndexPage.vue";
import LoginPage from "./pages/LoginPage.vue";
import SignupPage from "./pages/SignupPage.vue";
import UserPage from "./pages/UserPage.vue";

const routes = [
    { path: '/', component: IndexPage },
    { path: '/login', component: LoginPage },
    { path: '/user', component: UserPage },
    { path: '/new', component: SignupPage }
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes
})

const app = createApp(App);
app.use(router);
app.mount("#app");
