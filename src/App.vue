<template>
  <div class="container">
    <button @click="getFlPath">{{ flpath }}</button>
    <button @click="getBeatsPath">{{ beatspath }}</button>
    <button @click="run">run</button>
    <button @click="reset">reset</button>
    <input type="range" min="1" max="100" v-model="volume" />
    <div class="songs">
      <Song
        v-for="(sound, index) in sounds"
        :key="index"
        :volume="volume"
        :sound="sound"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { appDataDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";
import {
  createDir,
  BaseDirectory,
  readDir,
  copyFile,
  removeDir,
} from "@tauri-apps/api/fs";
import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Store } from "tauri-plugin-store-api";
import Song from "./components/song.vue";
const store = new Store(".settings.dat");

const flpath = ref("");
const beatspath = ref("");
const waitingRoom = ref("");
const destination = ref("");
const sounds = ref([]);
const volume = ref("50");

onMounted(async () => {
  const url = await appDataDir();
  destination.value = url + "exports";

  waitingRoom.value = url + "waitingRoom";

  sounds.value = await readDir(destination.value);

  const fl_path = await store.get("fl-path");
  if (fl_path) flpath.value = fl_path.value;

  const beat_path = await store.get("beat-path");
  if (beat_path) beatspath.value = beat_path.value;

  const vol = await store.get("volume");
  if (vol) volume.value = vol.value;

  try {
    await createDir("exports", { dir: BaseDirectory.AppData, recursive: true });
  } catch (e) {
    console.error(e);
  }
});

watch(
  () => volume.value,
  async () => {
    await store.set("volume", { value: volume.value });
    await store.save();
  }
);

const run = async () => {
  await invoke("run_shell_command", {
    fl: formatUrl(flpath.value) + "/FL64.exe",
    beats: formatUrl(waitingRoom.value),
    destination: formatUrl(destination.value),
  });

  try {
    await removeDir("waitingRoom", {
      dir: BaseDirectory.AppData,
      recursive: true,
    });

    console.log("All items removed from the waitingRoom folder.");
  } catch (e) {
    console.error(e);
  }
};

const reset = async () => {
  const beats = await readDir(beatspath.value);

  const uniqueValues = beats.filter(
    (item2) =>
      !sounds.value.some(
        (item1) => removeExtensions(item1.name) === removeExtensions(item2.name)
      ) && item2.name !== "desktop.ini"
  );

  try {
    await createDir("waitingRoom", {
      dir: BaseDirectory.AppData,
      recursive: true,
    });

    const url = await appDataDir();

    for (const uniqueItem of uniqueValues) {
      const targetPath = `${url}waitingRoom/${uniqueItem.name}`;
      await copyFile(uniqueItem.path, targetPath);
    }
  } catch (e) {
    console.error(e);
  }
};

function removeExtensions(filename: string): string {
  const extensionsToRemove = [".mp3", ".flp"];

  for (const extension of extensionsToRemove) {
    if (filename.endsWith(extension)) {
      return filename.slice(0, -extension.length);
    }
  }

  return filename;
}

const formatUrl = (input: string): string => {
  console.log(input.replace(/\\/g, "/"));
  return input.replace(/\\/g, "/");
};

const getFlPath = async () => {
  flpath.value = await readDirList();
  await store.set("fl-path", { value: flpath.value });
  await store.save();
};
const getBeatsPath = async () => {
  beatspath.value = await readDirList();
  await store.set("beat-path", { value: beatspath.value });
  await store.save();
};

const readDirList = async () => {
  try {
    const selectedPath = await open({
      multiple: false,
      directory: true,
    });
    if (selectedPath) return selectedPath;
  } catch (err) {
    console.error(err);
  }
};
</script>

<style scoped>
.songs {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
</style>
