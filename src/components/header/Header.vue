<script setup lang="ts">
import InputText from 'primevue/inputtext';
import InfoCard from '../infoCard/InfoCard.vue';
import { ref } from 'vue';

const value = ref(null);
const changeNumber = (evt: Event) => console.log(evt?.target?.value);
</script>

<template>
  <div class="header">
    <span class="p-float-label">
      <InputText id="carNumber" v-model="value" @input="changeNumber" />
      <label for="carNumber">Номер авто: {{ value }}</label>
    </span>
  </div>
  <div class="info-card">
    <InfoCard />
  </div>
</template>

<style scoped>
.header,
.info-card {
  flex: 1;
  display: flex;
  justify-content: center;
  width: 100%;
}
</style>
