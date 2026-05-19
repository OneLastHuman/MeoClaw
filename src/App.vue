<script setup lang="ts">
import DesktopPet from "./components/DesktopPet.vue";
</script>

<template>
  <DesktopPet />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
  position: relative;
}
</style>
