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
        <div class="brand">
            <div class="logo">
                <img src="../assets/neko.png" alt="" srcset="">
            </div>


        </div>

        <div class="search">
            <input placeholder="Search an anime"
                class="searchinput drac-input drac-input-border-sm drac-input-purple drac-text-purple drac-m-xs" />

            <button class="drac-btn drac-bg-purple drac-btn-outline drac-text-purple drac-m-sm"
                type="submit">Go!</button>

        </div>

        <div class="navigation">
            <div class="links">
                <div class="drac-box">
                    <ul class="drac-tabs drac-tabs-purple">
                        <li class="drac-tab drac-tab-active">
                            <a class="drac-tab-link drac-text" href="#">Currently Watching</a>
                        </li>
                        <li class="drac-tab">
                            <a class="drac-tab-link drac-text" href="#">Completed</a>
                        </li>
                        <li class="drac-tab">
                            <a class="drac-tab-link drac-text" href="#">On Hold</a>
                        </li>
                        <li class="drac-tab">
                            <a class="drac-tab-link drac-text" href="#">Dropped</a>
                        </li>
                        <li class="drac-tab">
                            <a class="drac-tab-link drac-text" href="#">Plant To Watch</a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>


    </div>
</template>


<style scoped>
.nav-wrapper {
    display: flex;
    flex-direction: row;
    height: 3rem;
    flex: 1;
}

.navigation {
    display: flex;
    flex-direction: row;
    align-items: right;
    justify-content: center;
    flex: 2;
}



h1 {
    text-align: center;
    color: red;
}

img {
    height: 100%;
    filter: drop-shadow(0 0 1em #747bff);
}

.brand {
    margin-top: -20px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.searchinput {
    width: 20rem;
}

.search {
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