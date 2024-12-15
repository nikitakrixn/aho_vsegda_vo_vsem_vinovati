<script setup lang="ts">
const employees = [
  {
    id: 1,
    name: 'Иван Иванов',
    department: 'Отдел разработки',
    position: 'Разработчик',
    hireDate: '2022-01-15',
    adLogin: 'ivanov',
    email: 'ivanov@example.com',
    status: 'Работает',
    phone: '+7 (900) 123-45-67',
    photo: 'https://via.placeholder.com/150',
  },
  {
    id: 2,
    name: 'Ольга Петрова',
    department: 'Отдел разработки',
    position: 'Тестировщик',
    hireDate: '2021-03-10',
    adLogin: 'petrova',
    email: 'petrova@example.com',
    status: 'Работает',
    phone: '+7 (900) 234-56-78',
    photo: 'https://via.placeholder.com/150',
  },
  {
    id: 3,
    name: 'Сергей Смирнов',
    department: 'Отдел продаж',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: 'https://via.placeholder.com/150',
  },
];

const departments = [
  {
    name: 'Отдел разработки',
    head: 'Иван Иванов',
  },
  {
    name: 'Отдел продаж',
    head: 'Сергей Смирнов',
  },
];

const positions = [
  { name: 'Разработчик' },
  { name: 'Тестировщик' },
  { name: 'Менеджер' },
];

// Получаем ID сотрудника из URL
const route = useRoute();
const router = useRouter();
const employeeId = parseInt(route.params.id);

// Находим сотрудника в массиве данных
const employee = employees.find((emp) => emp.id === employeeId);

if (!employee) {
  throw createError({
    statusCode: 404,
    statusMessage: 'Сотрудник не найден',
  });
}

// Создаём локальные свойства для редактирования
const editedEmployee = ref({
  name: employee.name,
  department: employee.department,
  position: employee.position,
  hireDate: employee.hireDate,
  adLogin: employee.adLogin,
  email: employee.email,
  phone: employee.phone,
  status: employee.status,
});

// Управление состоянием модального окна для удаления
const isDeleteModalOpen = ref(false);
const openDeleteModal = () => (isDeleteModalOpen.value = true);
const closeDeleteModal = () => (isDeleteModalOpen.value = false);

// Функция для сохранения изменений
const saveChanges = () => {
  // На этом этапе отправьте запрос на сервер (в будущем)
  // Пример: await api.put(`/employees/${employee.id}`, editedEmployee.value);

  // Временно обновляем данные в локальном массиве
  Object.assign(employee, editedEmployee.value);
  alert('Данные сохранены!');

  // Перенаправляем на страницу списка сотрудников
  router.push('/employees');
};

// Функция для удаления сотрудника
const deleteEmployee = (action) => {
  if (action === 'delete') {
    // Полное удаление (отправьте запрос на сервер в будущем)
    const index = employees.findIndex((emp) => emp.id === employeeId);
    if (index !== -1) employees.splice(index, 1);
    alert('Сотрудник удалён!');
  } else if (action === 'dismiss') {
    // Перевод в статус "Уволен"
    employee.status = 'Уволен';
    alert('Сотрудник переведён в статус "Уволен"!');
  }
  closeDeleteModal();
  router.push('/employees'); // Перенаправление на список сотрудников
};
</script>

<template>
  <div class="p-6">
    <h1 class="text-4xl font-bold text-gray-800 mb-6">Редактирование сотрудника</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Фото -->
      <div class="flex items-center justify-center">
        <img
          :src="employee.photo"
          alt="Фото сотрудника"
          class="w-64 h-64 rounded-lg shadow-lg"
        />
      </div>

      <!-- Форма редактирования -->
      <div>
        <form @submit.prevent="saveChanges" class="space-y-4">
          <div>
            <label for="name" class="block text-gray-700">Имя</label>
            <input
              id="name"
              v-model="editedEmployee.name"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg"
              type="text"
            />
          </div>

          <div>
            <label for="department" class="block text-gray-700">Отдел</label>
            <input
              id="department"
              v-model="editedEmployee.department"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg"
              type="text"
            />
          </div>

          <div>
            <label for="position" class="block text-gray-700">Должность</label>
            <input
              id="position"
              v-model="editedEmployee.position"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg"
              type="text"
            />
          </div>

          <div>
            <label for="email" class="block text-gray-700">Email</label>
            <input
              id="email"
              v-model="editedEmployee.email"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg"
              type="email"
            />
          </div>

          <div>
            <label for="phone" class="block text-gray-700">Телефон</label>
            <input
              id="phone"
              v-model="editedEmployee.phone"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg"
              type="tel"
            />
          </div>

          <div>
            <label for="status" class="block text-gray-700">Статус</label>
            <select
              id="status"
              v-model="editedEmployee.status"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg"
            >
              <option value="Работает">Работает</option>
              <option value="В отпуске">В отпуске</option>
              <option value="Уволен">Уволен</option>
            </select>
          </div>

          <div class="flex space-x-4">
            <button
              type="submit"
              class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              Сохранить
            </button>
            <button
              type="button"
              @click="openDeleteModal"
              class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
            >
              Удалить
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Модальное окно для удаления -->
    <Modal
      v-if="isDeleteModalOpen"
      :isOpen="isDeleteModalOpen"
      title="Удаление сотрудника"
      @close="closeDeleteModal"
    >
      <p class="text-gray-700 mb-4">
        Вы уверены, что хотите удалить сотрудника? Вы можете либо полностью удалить сотрудника, либо перевести его в статус "Уволен".
      </p>
      <div class="flex justify-end space-x-4">
        <button
          @click="deleteEmployee('delete')"
          class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Полностью удалить
        </button>
        <button
          @click="deleteEmployee('dismiss')"
          class="px-4 py-2 bg-yellow-500 text-white rounded hover:bg-yellow-600"
        >
          Перевести в "Уволен"
        </button>
        <button
          @click="closeDeleteModal"
          class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300"
        >
          Отмена
        </button>
      </div>
    </Modal>
  </div>
</template>