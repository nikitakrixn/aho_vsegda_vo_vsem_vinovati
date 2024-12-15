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
    default: 'flex justify-center p-2 text-primary hover:bg-blue-600 hover:text-white rounded-md transition duration-300 ease-in-out',
  },
  activeClass: {
    type: String,
    default: 'bg-blue-600 text-white',
  },
  iconClass: {
    type: String,
    default: 'shrink-0 w-5 h-5',
  },
});
const isActive = computed(() => route.path === props.to || route.path.startsWith(props.to + '/'));
</script>

<template>
  <RouterLink :to="to" :class="[isActive ? activeClass : 'bg-slate-100', baseClass ]">
      <Icon :name="icon" :class="iconClass" />
      <span v-if="text">{{ text }}</span>
  </RouterLink>
</template>