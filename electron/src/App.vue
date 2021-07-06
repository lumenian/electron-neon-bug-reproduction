<template>
  <div id="app">
    <button @click="startTask" style="width: 200px; height: 120px;">
      Test
    </button>
  </div>
</template>

<script>
import * as PIXI from 'pixi.js'

export default {
  name: "App",
  data() {
    return {
      pixiApp: null,
    };
  },
  methods: {
    async startTask() {
      console.log(await window.api.startTask());
    },
  },
  mounted() {
    // Create PIXI application for strategy subcells rendering
    this.pixiApp = PIXI.Application({
      width: 100,
      height: 100,
    });
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
