<script setup lang="ts">

const props = defineProps<{
    onLogout: () => void
}>();

function logout() {
    props.onLogout();
}

</script>

<template>
    <div>
        <h1>Username</h1>
        <button>Settings</button>
        <button v-on:click="logout">Logout</button>
    </div>
</template>