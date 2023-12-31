<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
    onSubmit: (username: string, password: string) => void
    onLogin: () => void
}>();

const name = ref("");
const username = ref("");
const password = ref("");
const cpassword = ref("");

function submit() {
    if (!name.value || !username.value || !password.value || !cpassword.value)
        return;

    if (password.value != cpassword.value)
        return;

    props.onSubmit(username.value, password.value);
}

function login() {
    props.onLogin();
}
</script>

<template>
    <form name="signin-form" @submit.prevent="submit">
        <label for="name-input">Name <span>*</span></label>
        <input type="text" name="name-input" placeholder="Name..." v-model="name" required/>
        <label for="email-input">Email <span>*</span></label>
        <input type="email" name="email-input" placeholder="example@email.com" v-model="username" required/>
        <label for="password-input">Password <span>*</span></label>
        <input type="password" name="password-input" placeholder="Enter password..." v-model="password" required/>
        <label for="cpassword-input">Confirm password <span>*</span></label>
        <input type="password" name="cpassword-input" placeholder="Enter password..." v-model="cpassword" required/>
        <input type="submit" value="Sign Up"/>
    </form>
    <div>
        <p>Already a member?</p>
        <button v-on:click="login">Login</button>
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
input[type=email], input[type=password], input[type=text] {
    background-color: #eee;
    border: 1px solid #ccc;
}
input[type=submit] {
    background-color: rgb(40, 180, 20);
    border: 1px solid rgb(40, 160, 20);
}
label {
    margin-top: 4px;
    font-size: 12px;
}
span {
    color: red;
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