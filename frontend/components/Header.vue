<template>
  <header class="flex justify-between items-center pb-4 border-b border-slate-200">
    <h1 class="text-2xl font-bold text-gray-800">{{ title }}</h1>
    <div class="flex space-x-4">
      <NuxtLink
        v-for="link in links"
        :key="link.to"
        :to="link.to"
        class="text-gray-600 hover:text-gray-900"
      >
        {{ link.text }}
      </NuxtLink>
    </div>
  </header>
</template>

<script setup>
defineProps({
  title: {
    type: String,
    required: true,
  },
  links: {
    type: Array,
    default: () => [],
  },
});
</script>