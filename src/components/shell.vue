<template>
  <button @click="run" type="submit">run</button>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

const run = async () => {
  await invoke("run_shell_command", {
    fl: "C:/Users/jiper/FL Studio 20/FL64.exe",
    beats: "C:/Users/jiper/Desktop/testfl",
  });
};
</script>
