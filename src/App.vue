<script setup lang="ts">
import Header from './components/header/Header.vue';
import InputGroup from './components/inputGroup/InputGroup.vue';
import OutputGroup from './components/outputGroup/OutputGroup.vue';
</script>

<template>
  <div class="container">
    <div class="app">
      <Header />
      <div class="content">
        <InputGroup />
        <OutputGroup />
      </div>
    </div>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
.content {
  display: flex;
  justify-content: space-around;
}
</style>
