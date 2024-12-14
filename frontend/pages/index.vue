<script setup lang="ts">
const userCount = ref(100);
const serverCount = ref(20);
const recentEvents = ref([
  { id: 1, message: 'Пользователь JohnDoe вошел в систему.' },
  { id: 2, message: 'Сервер server1 перезагружен.' },
]);
</script>

<template>
  <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <!-- Карточка с общей информацией -->
    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Системные ресурсы</h2>
      <div>
        <div class="mb-4">
          <label class="text-gray-600">CPU Usage</label>
          <div class="w-full bg-gray-200 rounded-full h-4 mt-2">
            <div class="bg-blue-500 h-4 rounded-full" style="width: 65%;"></div>
          </div>
          <span class="text-sm text-gray-500">65%</span>
        </div>
        <div class="mb-4">
          <label class="text-gray-600">Memory Usage</label>
          <div class="w-full bg-gray-200 rounded-full h-4 mt-2">
            <div class="bg-green-500 h-4 rounded-full" style="width: 45%;"></div>
          </div>
          <span class="text-sm text-gray-500">45%</span>
        </div>
        <div>
          <label class="text-gray-600">Disk Space</label>
          <div class="w-full bg-gray-200 rounded-full h-4 mt-2">
            <div class="bg-red-500 h-4 rounded-full" style="width: 80%;"></div>
          </div>
          <span class="text-sm text-gray-500">80% used</span>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Последние события</h2>
      <ul class="text-gray-600">
        <li class="mb-2">[12:30] Backup completed successfully.</li>
        <li class="mb-2">[11:15] New user account created: johndoe.</li>
        <li class="mb-2">[10:05] Warning: High CPU usage detected.</li>
      </ul>
    </div>

    <!-- Карточка с состоянием услуг -->
    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Статус сервисов</h2>
      <ul class="text-gray-600">
        <li class="flex items-center mb-2">
          <span class="w-3 h-3 bg-green-500 rounded-full mr-2"></span>
          Web Server: <span class="ml-auto text-green-500">Active</span>
        </li>
        <li class="flex items-center mb-2">
          <span class="w-3 h-3 bg-green-500 rounded-full mr-2"></span>
          Database Server: <span class="ml-auto text-green-500">Active</span>
        </li>
        <li class="flex items-center mb-2">
          <span class="w-3 h-3 bg-red-500 rounded-full mr-2"></span>
          Email Server: <span class="ml-auto text-red-500">Down</span>
        </li>
      </ul>
    </div>
  </section>
</template>
