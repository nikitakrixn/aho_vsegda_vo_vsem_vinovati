<template>
  <div class="flex h-screen font-onest">
    <aside class="w-20">
      <div class="p-4">
        <CommonApplicationLogo class="h-12 w-auto mx-auto" />
        <nav class="mt-6">
          <ul class="space-y-6">
            <NavLink to="/" icon="ph:buildings-bold"/>
            <NavLink to="/employees" icon="ph:users-bold" />
            <NavLink to="/departments" icon="ph:folders-bold" />
            <NavLink to="/equipment" icon="ph:cpu-bold" />
          </ul>
        </nav>
      </div>
    </aside>
    <div class="flex-1 p-8 bg-slate-100">
      <header class="flex justify-between items-center pb-4 border-b border-slate-300">
        <h1 class="text-2xl font-bold text-gray-800">{{ currentCategory }}</h1>
        <div class="flex space-x-4">
          <router-link
            v-for="link in linksByCategory[currentCategory]"
            :key="link.to"
            :to="link.to"
            class="text-gray-600 hover:text-gray-900"
          >
            {{ link.text }}
          </router-link>
        </div>
      </header>
      <main class="mt-6">
        <slot />
      </main>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
const currentCategory = computed(() => {
  switch (route.name) {
    case 'index':
      return 'Главная';
    case 'employees':
      return 'Сотрудники';
    case 'departments':
      return 'Отделы';
    case 'equipment':
      return 'Оборудование';
    default:
      return 'Выберите категорию';
  }
});


const linksByCategory = {
  'Сотрудники': [
    { to: '/employees/list', text: 'Список сотрудников' },
    { to: '/employees/add', text: 'Добавить сотрудника' },
  ],
  'Отделы': [
    { to: '/departments/list', text: 'Список отделов' },
    { to: '/departments/add', text: 'Добавить отдел' },
  ],
  'Оборудование': [
    { to: '/equipment/list', text: 'Список оборудования' },
    { to: '/equipment/add', text: 'Добавить оборудование' },
  ],
};
</script>