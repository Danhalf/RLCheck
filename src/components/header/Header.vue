<script setup lang="ts">
import InputText from 'primevue/inputtext';
import { ref } from 'vue';

const value = ref(null);
</script>

<template>
  <div class="header">
    <span class="p-float-label">
      <InputText id="carNumber" v-model="value" />
      <label for="carNumber">Car number</label>
    </span>
  </div>
</template>

<style scoped>
.header {
  flex: 1;
  width: 100%;
}
</style>
