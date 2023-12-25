<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
</script>

<template>
  <p>{{ route.fullPath }}</p>
  <hr/>
  <router-view></router-view>
</template>
