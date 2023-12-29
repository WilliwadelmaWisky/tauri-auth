<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
    onSubmit: (username: string, password: string) => void
}>();

const username = ref("");
const password = ref("");

function submit() {
    if (!username.value || !password.value)
        return;

    props.onSubmit(username.value, password.value);
}

</script>

<template>
    <form name="signin-form" @submit.prevent="submit">
        <p>
            <label for="email-input">Email</label>
            <input type="email" name="email-input" placeholder="example@email.com" v-model="username" required/>
        </p>
        <p>
            <label for="password-input">Password</label>
            <input type="password" name="password-input" placeholder="Enter password..." v-model="password" required/>
        </p>
        <p>
            <input type="submit" value="Sign In"/>
        </p>
    </form>
</template>

<style scoped>
p {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
input {
    padding: 12px 20px;
    margin: 8px 0;
    display: inline-block;
    box-sizing: border-box;
    border: 1px solid #ccc;
    border-radius: 2px;
    flex-grow: 1;
    box-shadow: 4px 4px 4px black;
}
label {
    width: 6em;
}
</style>