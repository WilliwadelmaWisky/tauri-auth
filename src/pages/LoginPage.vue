<script setup lang="ts">
import LoginForm from "../components/LoginForm.vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api";

const router = useRouter();

async function login(username: string, password: string) {
    const success: boolean = await invoke("login", { username: username, password: password });
    if (!success) {
        return;
    }

    await router.push("/");
}

async function signup() {
    await router.push("/new");
}

</script>

<template>
    <div>
        <h1>Login</h1>
        <LoginForm 
            v-on:submit="login"
            v-on:signup="signup"
        />
    </div>  
</template>

<style scoped>
    div {
        display: flex;
        flex-direction: column;
        width: 50%;
        align-items: center;
        margin: auto;
        background-color: #ddd;
        border: 2px solid rgb(30, 30, 30);
        border-radius: 1em;
        box-shadow: 10px 10px 1em black;
    }
</style>