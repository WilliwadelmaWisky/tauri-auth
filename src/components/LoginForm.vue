<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
    onSubmit: (username: string, password: string) => void
    onSignup: () => void
}>();

const username = ref("");
const password = ref("");

function submit() {
    if (!username.value || !password.value)
        return;

    props.onSubmit(username.value, password.value);
}

function signup() {
    props.onSignup();
}
</script>

<template>
    <form name="signin-form" @submit.prevent="submit">
        <input type="email" placeholder="Email" v-model="username" required/>
        <input type="password" placeholder="Password" v-model="password" required/>
        <input type="submit" value="Login"/>
    </form>
    <div>
        <p>Not a member?</p>
        <button v-on:click="signup">Sign up</button>
    </div>
</template>

<style scoped>
form {
    display: flex;
    flex-direction: column;
}
input {
    padding: 12px 20px;
    margin: 8px 0;
    display: inline-block;
    box-sizing: border-box;
    border-radius: 2px;
    box-shadow: 4px 4px 4px black;
}
input[type=email], input[type=password] {
    background-color: #eee;
    border: 1px solid #ccc;
}
input[type=submit] {
    background-color: rgb(40, 180, 20);
    border: 1px solid rgb(40, 160, 20);
}
div {
    margin-top: 1em;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 2px;
}
div p {
    font-size: 12px;
}
div button {
    font-size: 16px;
    background-color: transparent;
    border: none;
}
</style>