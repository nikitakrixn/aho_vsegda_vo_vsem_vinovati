<template>
  <div class="flex h-screen font-onest">
    <aside class="w-16">
      <div class="p-4">
        <CommonApplicationLogo class="h-10 w-auto" />
        <nav class="mt-6">
          <ul class="space-y-2">
            <li>
              <router-link to="/" class="flex items-center py-2 px-4  bg-light hover:bg-gray-200 rounded-md">
                <Icon name="uil:github" class="text-primary" />
              </router-link>
            </li>
            <li>
              <router-link to="/employees" class="flex items-center py-2 px-4 text-gray-600 hover:bg-gray-200 rounded-md">
                <Icon name="ph:users-light" class="text-primary" style="color: black;" />
              </router-link>
            </li>
            <li>
              <router-link to="/departments" class="flex items-center py-2 px-4 text-gray-600 hover:bg-gray-200 rounded-md">
                Отделы
              </router-link>
            </li>
            <li>
              <router-link to="/equipment" class="flex items-center py-2 px-4 text-gray-600 hover:bg-gray-200 rounded-md">
                Оборудование
              </router-link>
            </li>
          </ul>
        </nav>
      </div>
    </aside>
    <div class="flex-1 p-8 bg-slate-100">
      <header class="flex justify-between items-center mb-4">
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
      <main class="p-4">
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