<script setup lang="ts">
import LoginForm from "../components/LoginForm.vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api";

const router = useRouter();

async function login(username: string, password: string) {
    const success = await invoke("signin", { username: username, password: password });
    if (!success)
        return;

    await router.push("/user");
}

</script>

<template>
    <div>
        <h1>Login</h1>
        <LoginForm v-on:submit="login"/>
    </div>  
</template>

<style scoped>
    div {
        display: flex;
        flex-direction: column;
        width: 50%;
        align-items: center;
        margin: auto;
        background-color: darkgray;
        border: 2px solid aquamarine;
        border-radius: 1em;
        box-shadow: 10px 10px 1em black;
    }
</style>