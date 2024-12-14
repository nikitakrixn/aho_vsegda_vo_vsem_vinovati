<script setup lang="ts">
import { RouterLink } from 'vue-router';

const route = useRoute();
const props = defineProps({
  to: {
    type: [String, Object],
    required: true,
  },
  icon: {
    type: String,
    required: true,
  },
  text: {
    type: String,
    required: false,
  },
  baseClass: {
    type: String,
    default: 'flex justify-center p-2 bg-light text-primary hover:bg-slate-200 rounded-md text-xl',
  },
  activeClass: {
    type: String,
    default: 'bg-primary text-white',
  },
  iconClass: {
    type: String,
    default: 'shrink-0',
  },
});
const isActive = computed(() => route.path === props.to);
</script>

<template>
  <RouterLink :to="to" :class="[baseClass, isActive ? activeClass : '']">
      <Icon :name="icon" :class="iconClass" />
      <span v-if="text">{{ text }}</span>
  </RouterLink>
</template>