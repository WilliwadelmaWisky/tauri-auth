<script setup lang="ts">
import { useRouter } from 'vue-router';
import UserDashboard from '../components/UserDashboard.vue';

const router = useRouter();

async function logout() {
    await router.push("/")
}
</script>

<template>
    <UserDashboard v-on:logout="logout"/>
</template>
