<template>
  <div class="card flex align-items-center justify-content-center">
    <Card style="width: 100%">
      <template #header> </template>
      <template #title> Volvo V60 </template>
      <template #subtitle> 0000J1 </template>
      <template #content>
        <div class="font-bold">Належність: Бігун</div>
      </template>
      <template #footer>
        <Button icon="pi pi-check" label="">Редагувати</Button>
      </template>
    </Card>
  </div>
</template>

<script setup>
import Card from 'primevue/card';
</script>
