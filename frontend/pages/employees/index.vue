<script setup lang="ts">

definePageMeta({
  title: 'Сотрудники по отделам', // Заголовок для лейаута
  category: 'Сотрудники по отделам', // Категория для Header
  links: [
    { to: '/employees/list', text: 'Список сотрудников' },
    { to: '/employees/add', text: 'Добавить сотрудника' }
  ]
});

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
    photo: '',
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
    photo: '',
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
    photo: '',
  },
  {
    id: 4,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
  },
  {
    id: 5,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
  },
  {
    id: 6,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
  },
  {
    id: 7,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
  },
  {
    id: 8,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
  },
  {
    id: 9,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
  },
  {
    id: 10,
    name: 'Сергей Смирнов',
    department: 'Отдел разработки',
    position: 'Менеджер',
    hireDate: '2019-08-20',
    adLogin: 'smirnov',
    email: 'smirnov@example.com',
    status: 'В отпуске',
    phone: '+7 (900) 345-67-89',
    photo: '',
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

// Состояние модального окна
const isModalOpen = ref(false); // Открыто ли модальное окно
const selectedEmployee = ref(null); // Сотрудник, чья информация отображается в модальном окне

// Состояние для выпадающего меню
const activeMenu = ref(null); // Хранит ID сотрудника, у которого открыто меню

// Подключение роутера
const router = useRouter();

// Открыть модальное окно для просмотра информации о сотруднике
const openInfoModal = (employee) => {
  selectedEmployee.value = employee;
  isModalOpen.value = true;
};

// Закрыть модальное окно
const closeModal = () => {
  selectedEmployee.value = null;
  isModalOpen.value = false;
};

// Открыть выпадающее меню
const openMenu = (employeeId) => {
  activeMenu.value = employeeId;
};

// Закрыть выпадающее меню
const closeMenu = () => {
  activeMenu.value = null;
};

// Закрытие выпадающего меню при клике вне карточки
const handleOutsideClick = (event) => {
  if (!event.target.closest('[data-menu]')) {
    closeMenu();
  }
};

// Перенаправление на страницу редактирования сотрудника
const editEmployee = (employee) => {
  router.push(`/employees/${employee.id}`);
};

// Удалить сотрудника
const deleteEmployee = (employee) => {
  const index = employees.findIndex((emp) => emp.id === employee.id);
  if (index !== -1) employees.splice(index, 1);
  alert(`Сотрудник ${employee.name} удалён!`);
  closeMenu();
};

// Перевести сотрудника в статус "Уволен"
const dismissEmployee = (employee) => {
  employee.status = 'Уволен';
  alert(`Сотрудник ${employee.name} переведён в статус "Уволен"!`);
  closeMenu();
};

function getInitials(name: string): string {
  const nameParts = name.split(' ');
  let initials = '';
  for (let i = 0; i < nameParts.length && i < 2; i++) {
    initials += nameParts[i].charAt(0).toUpperCase();
  }
  return initials;
}


function generateAvatar(name: string, size: number = 128): string {
      const initials = getInitials(name);
      const textColor = '#556ee6';
      const svg = `
      <svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 ${size} ${size}">
        <rect width="100%" height="100%" fill="#eef1ff" />
        <text x="50%" y="50%" dominant-baseline="central" font-family="Arial" text-anchor="middle" font-size="${size / 2.5}" fill="${textColor}">${initials}</text>
      </svg>
    `;
    const textEncoder = new TextEncoder();
    const encodedSvg = `data:image/svg+xml;base64,${btoa(String.fromCharCode(...textEncoder.encode(svg)))}`;
    return encodedSvg;
}

// Группировка сотрудников по отделам
const employeesByDepartment = computed(() => {
  return departments.map((department) => ({
    department: department.name,
    head: department.head,
    employees: employees.filter((employee) => employee.department === department.name).map(employee => {
        return {
            ...employee,
            photo: generateAvatar(employee.name)
        }
    }),
  }));
});
</script>

<template>
  <div @click="handleOutsideClick">
    <!-- Список отделов -->
    <div v-for="group in employeesByDepartment" :key="group.department" class="mb-8">
      <div class="mb-6">
        <h2 class="text-2xl font-semibold text-gray-700">{{ group.department }}</h2>
        <p class="text-gray-500">Руководитель: <strong class="text-gray-800">{{ group.head }}</strong></p>
      </div>

      <!-- Карточки сотрудников -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="employee in group.employees"
          :key="employee.id"
          class="relative bg-white shadow-lg rounded-lg flex flex-col p-4"
        >
          <!-- Карточка -->
          <div class="flex justify-end items-center w-full">
            <p
              class="text-xs px-2 py-1 rounded-full"
              :class="{
                'bg-green-100 text-green-800': employee.status === 'Работает',
                'bg-yellow-100 text-yellow-800': employee.status === 'В отпуске',
                'bg-red-100 text-red-800': employee.status === 'Уволен',
              }"
            >
              {{ employee.status }}
            </p>
            <!-- Иконка три точки -->
            <div class="mt-1" data-menu>
              <button
                @click.stop="openMenu(employee.id)"
              >
                <Icon name="ph:dots-three-vertical-bold" class="w-6 h-6" />
              </button>

              <!-- Выпадающее меню -->
              <div
                v-if="activeMenu === employee.id"
                class="absolute right-0 mt-2 w-44 mr-4 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5"
              >
                <button
                  @click.stop="deleteEmployee(employee)"
                  class="block w-full text-left px-4 py-2 text-sm text-red-500 hover:bg-red-50"
                >
                  Удалить
                </button>
                <button
                  @click.stop="dismissEmployee(employee)"
                  class="block w-full text-left px-4 py-2 text-sm text-yellow-500 hover:bg-yellow-50"
                >
                  Уволить
                </button>
              </div>
            </div>
          </div>
          <div class="flex items-center mb-4">
            <img
              :src="employee.photo"
              alt="Photo"
              class="w-16 h-16 rounded-full ring-2 ring-blue-100"
            />
            <div class="flex flex-col ml-4">
              <h3 class="text-lg font-bold text-slate-800">{{ employee.name }}</h3>
              <p class="text-sm text-slate-500">{{ employee.position }}</p>
            </div>
          </div>
          
          <div class="flex flex-col space-y-2 bg-blue-50 rounded-md p-2">
            <div class="flex items-center text-sm"><Icon name="ph:phone" class="w-5 h-5 text-blue-600 mr-2" /> {{ employee.phone }}</div>
            <div class="flex items-center text-sm"><Icon name="ph:envelope" class="w-5 h-5 text-blue-600 mr-2" /> {{ employee.email }}</div>
          </div>
          <!-- Кнопки под карточкой -->
          <div class="flex space-x-4 mt-4">
            <button
              @click.stop="openInfoModal(employee)"
              class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              Просмотр
            </button>
            <button
              @click.stop="editEmployee(employee)"
              class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300"
            >
              Изменить
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Модальное окно -->
    <Modal
      v-if="selectedEmployee"
      :isOpen="isModalOpen"
      :title="selectedEmployee.name"
      @close="closeModal"
    >
      <div class="space-y-2">
        <p><strong>Отдел:</strong> {{ selectedEmployee.department }}</p>
        <p><strong>Должность:</strong> {{ selectedEmployee.position }}</p>
        <p><strong>Дата приема:</strong> {{ selectedEmployee.hireDate }}</p>
        <p><strong>Логин AD:</strong> {{ selectedEmployee.adLogin }}</p>
        <p><strong>Email:</strong> {{ selectedEmployee.email }}</p>
        <p><strong>Телефон:</strong> {{ selectedEmployee.phone }}</p>
        <p><strong>Статус:</strong> {{ selectedEmployee.status }}</p>
      </div>
    </Modal>
  </div>
</template>
<style scoped>
/* Анимация появления выпадающего меню */
div[role="menu"] {
  transition: opacity 0.2s ease, transform 0.2s ease;
  opacity: 0;
  transform: scale(0.95);
}

div[role="menu"][v-cloak] {
  opacity: 1;
  transform: scale(1);
}
</style>