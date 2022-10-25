<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import SideNav from "./components/SideNav.vue";
import TopNav from "./components/TopNav.vue";
import AnimeList from "./components/AnimeList.vue";
</script>

<template>
  <div class="container">
    <div class="navigations">
      <TopNav />
      <SideNav />
    </div>

    <div class="content">
      <AnimeList />
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
