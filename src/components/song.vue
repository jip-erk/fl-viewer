<template>
  <div>
    <button @click="notPlaying ? pause() : play()">
      {{ notPlaying ? "||" : ">" }}
    </button>
    {{ sound.name }}
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { convertFileSrc } from "@tauri-apps/api/tauri";

const props = defineProps<{
  sound: any;
  volume: string;
}>();

const audio = ref<HTMLAudioElement | undefined>();
const notPlaying = ref(false);
onMounted(() => {
  audio.value = new Audio(convertFileSrc(props.sound.path));
});

watch(
  () => props.volume,
  () => {
    if (!audio.value.volume) return;
    audio.value.volume = props.volume / 100;
  }
);

const play = () => {
  audio.value.play();
  notPlaying.value = true;
};

const pause = () => {
  audio.value.pause();
  notPlaying.value = false;
};
</script>
