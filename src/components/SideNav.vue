<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>

<template>
    <div class="nav-wrapper">
        <h1>sex</h1>
    </div>
</template>


<style scoped>
h1 {
    text-align: left;
    color: red;
}
</style>