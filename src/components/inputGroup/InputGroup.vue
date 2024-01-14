<script setup lang="ts">
import InputText from 'primevue/inputtext';
import { ref } from 'vue';

const value = ref(null);
</script>

<template>
  <div class="card flex justify-content-center w-100">
    <span class="p-float-label">
      <InputText id="carNumber" v-model="value" />
      <label for="carNumber">Car fuel before journey</label>
    </span>
  </div>
</template>

<style scoped></style>
