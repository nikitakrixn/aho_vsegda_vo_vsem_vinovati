<template>
  <div class="flex items-center justify-center h-screen bg-gray-100">
    <div class="text-center">
      <h1 class="text-6xl font-bold text-red-500 mb-4">
        {{ error.statusCode || 500 }}
      </h1>
      <p class="text-xl text-gray-700 mb-6">
        {{ error.statusMessage || 'Произошла ошибка' }}
      </p>
      <NuxtLink
        to="/"
        class="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
      >
        Вернуться на главную
      </NuxtLink>
    </div>
  </div>
</template>

<script setup>
defineProps(['error']);
</script>