<script setup>
import { computed } from 'vue'
import { RouterView, useRoute } from 'vue-router'
import { useAuth } from '@/composables/useAuth'
import Navbar from '@/components/Navbar.vue' // <--- Import Navbar

const route = useRoute()
const { isAuthenticated } = useAuth()

// Only show Navbar if logged in
const showNavbar = computed(() => {
  return route.name !== 'login' && isAuthenticated.value
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 font-sans text-gray-900">
    
    <Navbar v-if="showNavbar" />

    <main class="container mx-auto px-4 py-8">
      <RouterView /> 
    </main>

  </div>
</template>