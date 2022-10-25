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
    <div class="list-wrapper">
        <div class="anime-table">
            <table class="drac-table drac-table-purple">
                <thead>
                    <tr>
                        <th class="drac-text drac-text-white">Title</th>
                        <th class="drac-text drac-text-white">Progress</th>
                        <th class="drac-text drac-text-white">Score</th>
                        <th class="drac-text drac-text-white">Type</th>
                        <th class="drac-text drac-text-white">Season</th>
                        <th class="drac-text drac-text-white">Last Update</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>


                        <td class="drac-text drac-text-white">Strike The Blood</td>
                        <td class="drac-text drac-text-white">4/24</td>
                        <td class="drac-text drac-text-white">
                            7.2
                        </td>
                        <td class="drac-text drac-text-white">TV</td>
                        <td class="drac-text drac-text-white">Fall 2013</td>
                        <td class="drac-text drac-text-white">25/10/2022</td>

                    </tr>
                    <tr>
                        <td class="drac-text drac-text-white">Strike The Blood</td>
                        <td class="drac-text drac-text-white">4/24</td>
                        <td class="drac-text drac-text-white">
                            7.2
                        </td>
                        <td class="drac-text drac-text-white">TV</td>
                        <td class="drac-text drac-text-white">Fall 2013</td>
                        <td class="drac-text drac-text-white">25/10/2022</td>

                    </tr>
                    <tr>
                        <td class="drac-text drac-text-white">Strike The Blood</td>
                        <td class="drac-text drac-text-white">4/24</td>
                        <td class="drac-text drac-text-white">
                            7.2
                        </td>
                        <td class="drac-text drac-text-white">TV</td>
                        <td class="drac-text drac-text-white">Fall 2013</td>
                        <td class="drac-text drac-text-white">25/10/2022</td>

                    </tr>
                    <tr>
                        <td class="drac-text drac-text-white">Strike The Blood</td>
                        <td class="drac-text drac-text-white">4/24</td>
                        <td class="drac-text drac-text-white">
                            7.2
                        </td>
                        <td class="drac-text drac-text-white">TV</td>
                        <td class="drac-text drac-text-white">Fall 2013</td>
                        <td class="drac-text drac-text-white">25/10/2022</td>

                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>


<style scoped>
.list-wrapper {
    padding: 10px;
    display: flex;
    margin: 0 auto;
    justify-content: center;
    margin-top: -170px;
    margin-left: 250px;
    border-radius: 8px;
    background-color: #1a1a1a;
}

.anime-table td {
    width: 210px;
    max-width: 1000px;
}

.anime-table th {
    text-align: center;
}
</style>