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
        <div class="logo">
            <img src="../assets/neko.png" alt="" srcset="">
        </div>

        <div class="search">
            <input placeholder="Search an anime"
                class="searchinput drac-input drac-input-border-sm drac-input-purple drac-text-purple drac-m-xs" />

            <button class="drac-btn drac-bg-purple drac-btn-outline drac-text-purple drac-m-sm"
                type="submit">Go!</button>

        </div>

        <div class="navigation">
            <div class="links">
                <button class="drac-btn drac-bg-purple drac-m-sm">Currently Watching</button>
                <button class="drac-btn drac-bg-purple drac-m-sm">Completed</button>
                <button class="drac-btn drac-bg-purple drac-m-sm">On Hold</button>
                <button class="drac-btn drac-bg-purple drac-m-sm">Dropped</button>
                <button class="drac-btn drac-bg-purple drac-m-sm">Plan To Watch</button>
            </div>
        </div>


    </div>
</template>


<style scoped>
.nav-wrapper {
    display: flex;
    flex-direction: row;
    align-items: center;
    height: 3rem;
}

.navigation {
    display: flex;
    flex-direction: row;
    align-items: right;
    justify-content: right;
    flex: 1;
}

.links button {
    margin-right: 5px;
}

h1 {
    text-align: center;
    color: red;
}

img {
    height: 100%;
    filter: drop-shadow(0 0 1em #747bff);
}

.search {
    display: flex;
}

.searchinput {
    width: 20rem;
    margin-top: 15px;
}

.refresh-btn {
    text-align: center;
    font-size: 15px;
}


.refresh-btn:hover {
    cursor: pointer;
}
</style>