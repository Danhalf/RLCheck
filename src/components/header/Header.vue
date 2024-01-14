<script setup lang="ts">
import InputText from 'primevue/inputtext';
import InfoCard from '../infoCard/InfoCard.vue';
import { ref } from 'vue';

const value = ref(null);
</script>

<template>
  <div class="header">
    <span class="p-float-label">
      <InputText id="carNumber" v-model="value" />
      <label for="carNumber">Номер авто</label>
    </span>
  </div>
  <InfoCard />
</template>

<style scoped>
.header {
  flex: 1;
  display: flex;
  justify-content: center;
  width: 100%;
}
</style>
