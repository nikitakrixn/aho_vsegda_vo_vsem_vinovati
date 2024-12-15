<template>
  <div class="flex min-h-screen font-onest bg-slate-100">
    <aside class="w-20 border-r border-slate-200 fixed top-0 left-0 h-full bg-white">
      <div class="h-full p-4">
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
    <div class="flex-1 p-8 ml-20">
      <Header :title="currentCategory" :links="linksByCategory" />
      <main class="mt-6">
        <slot />
      </main>
    </div>
  </div>
</template>
<script setup lang="ts">

const route = useRoute();

const currentCategory = computed<string>(() => {
  return typeof route.meta.category === 'string' ? route.meta.category : 'Выберите категорию';
});
const linksByCategory = computed(() => {
  return Array.isArray(route.meta.links) ? route.meta.links : [];
});

useHead({
  title: computed(() => `${route.meta.title} | БУОО "Омскоблстройзаказчик"`),
})

</script>