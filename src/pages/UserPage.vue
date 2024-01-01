<script setup lang="ts">
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api";
import UserDashboard from '../components/UserDashboard.vue';

const router = useRouter();

async function logout() {
    await invoke("logout");
    await router.push("/")
}
</script>

<template>
    <UserDashboard v-on:logout="logout"/>
</template>
