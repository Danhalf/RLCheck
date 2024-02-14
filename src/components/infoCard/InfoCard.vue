<template>
  <div class="card">
    <Panel
      toggleable
      :pt="{
        toggler: {},
      }"
    >
      <template #header>{{ car.name }}</template>
      <Card>
        <template #title>{{ car.name }}</template>
        <template #subtitle>{{ car.number }}</template>
        <template #content>
          <div class="font-bold">Належність: {{ car.owner }}</div>
        </template>
      </Card>
      <template #footer>
        <Button icon="pi pi-check" label="">Редагувати</Button>
      </template>
    </Panel>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import Card from 'primevue/card';
import { ref } from 'vue';
import Panel from 'primevue/panel';

const car = ref({
  name: 'Volvo V70',
  number: '0000J1',
  owner: 'Бігун',
});

// if (await invoke('name')) {
invoke('name').then((res) => {
  console.log(res);
});
//   car.value.name = await invoke('name');
// }
</script>

<style scoped>
.card {
  min-width: 500px;
  max-width: 900px;
}
</style>
