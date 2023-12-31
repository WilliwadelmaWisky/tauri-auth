<script setup lang="ts">
import { useRouter } from "vue-router";
import SignupForm from "../components/SignupForm.vue";

const router = useRouter();

async function signup() {
    await router.push("/user");
}

async function login() {
    await router.push("/");
}
</script>

<template>
    <div>
        <h1>Sign Up</h1>
        <SignupForm 
            v-on:submit="signup" 
            v-on:login="login"
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