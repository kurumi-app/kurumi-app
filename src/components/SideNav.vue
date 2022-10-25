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
        <div class="links">
            <button class="drac-btn drac-bg-purple drac-m-sm">Anime List</button>

            <hr class="divider drac-divider drac-border-purple" />

            <button class="drac-btn drac-bg-purple drac-m-sm">Search Torrents</button>

            <a class="refreshhref" href="#"><span class="badgerefresh drac-badge drac-bg-animated"><span
                        class="drac-text drac-line-height drac-text-black">REFRESH</span></span></a>
        </div>
    </div>
</template>


<style scoped>
h1 {
    text-align: left;
    color: red;
}

.nav-wrapper {
    display: flex;
    flex-direction: column;
    align-items: left;
    margin-top: 60px;
}

.links {
    display: flex;
    max-width: 200px;
    flex-direction: column;
    align-items: left;
}

.links button {
    margin: 0.5rem;
}

.divider {
    width: 180px;
}

.badgerefresh {
    max-width: 100px;
    margin: 0 auto;
    align-content: center;
}

.badgerefresh:hover {
    cursor: pointer;
}

.refreshhref {
    margin-top: 20px;
    text-decoration: none;
}
</style>