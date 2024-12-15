<script setup lang="ts">
definePageMeta({
  title: 'Список сотрудников',
  category: 'Полный список сотрудников',
  links: [
    { to: '/employees', text: 'Список сотрудников по отделам' },
    { to: '/employees/add', text: 'Добавить сотрудника' }
  ]
})
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
</script>
<template>
    <div class="1"> 
      <div class="overflow-x-auto">
        <table class="w-full bg-white shadow-md rounded my-6">
          <thead class="bg-gray-50 border-b-2 border-gray-200">
            <tr>
              <th class="border border-gray-300 px-4 py-2 text-left text-sm font-semibold text-gray-700">#</th>
              <th class="border border-gray-300 px-4 py-2 text-left text-sm font-semibold text-gray-700">ФИО</th>
              <th class="border border-gray-300 px-4 py-2 text-left text-sm font-semibold text-gray-700">Отдел</th>
              <th class="border border-gray-300 px-4 py-2 text-left text-sm font-semibold text-gray-700">Должность</th>
              <th class="border border-gray-300 px-4 py-2 text-left text-sm font-semibold text-gray-700">Дата приема</th>
              <th class="border border-gray-300 px-4 py-2 text-left text-sm font-semibold text-gray-700">Статус</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(employee, index) in employees"
              :key="employee.id"
              class="hover:bg-gray-50 transition"
            >
              <td class="border border-gray-300 px-4 py-2 text-gray-700">{{ index + 1 }}</td>
              <td class="border border-gray-300 px-4 py-2 text-gray-700">{{ employee.name }}</td>
              <td class="border border-gray-300 px-4 py-2 text-gray-700">{{ employee.department }}</td>
              <td class="border border-gray-300 px-4 py-2 text-gray-700">{{ employee.position }}</td>
              <td class="border border-gray-300 px-4 py-2 text-gray-700">{{ employee.hireDate }}</td>
              <td
                class="border border-gray-300 px-4 py-2"
                :class="{
                  'text-green-600': employee.status === 'Работает',
                  'text-yellow-600': employee.status === 'В отпуске',
                  'text-red-600': employee.status === 'Уволен',
                }"
              >
                {{ employee.status }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </template>