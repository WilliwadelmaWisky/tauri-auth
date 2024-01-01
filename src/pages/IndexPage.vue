<script setup lang="ts">
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api";

const router = useRouter();
redirect();

async function redirect() {
    const authInfo: {
        is_authenticated: boolean,
        user: string
    } = await invoke("is_authenticated");

    if (authInfo.is_authenticated) {
        await router.push('/user');
        return;
    }

    await router.push('/login');
}
</script>

<template>
    <p>Loading...</p>
</template>