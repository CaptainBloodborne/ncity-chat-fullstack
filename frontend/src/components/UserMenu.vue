<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuth } from '@/composables/useAuth'

const { user, logout } = useAuth()
const router = useRouter()

// STATE: Is the popup open?
const isOpen = ref(false)

const menuRef = ref(null)

// ACTIONS
function toggleMenu() {
  isOpen.value = !isOpen.value
}

async function handleLogout() {
  isOpen.value = false

  await logout()

  router.push({ name: 'login' })
}

// CLOSE ON CLICK OUTSIDE
function closeOnClickOutside(event) {
  if (isOpen.value && menuRef.value && !menuRef.value.contains(event.target)) {
    isOpen.value = false
  }
}

onMounted(() => document.addEventListener('click', closeOnClickOutside))
onUnmounted(() => document.removeEventListener('click', closeOnClickOutside))
</script>

<template>
  <div ref="menuRef" class="relative">
    
    <button 
      @click="toggleMenu" 
      class="flex items-center gap-2 hover:bg-indigo-700 px-3 py-2 rounded transition focus:outline-none"
    >
      <span class="font-bold text-white">{{ user?.display_name }}</span>
      <svg 
        xmlns="http://www.w3.org/2000/svg" 
        class="h-4 w-4 text-indigo-200 transition-transform duration-200"
        :class="isOpen ? 'rotate-180' : ''"
        fill="none" viewBox="0 0 24 24" stroke="currentColor"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <div 
      v-if="isOpen"
      class="absolute right-0 mt-2 w-56 bg-white rounded-md shadow-xl border border-gray-100 z-50 overflow-hidden"
    >
      <div class="px-4 py-3 bg-gray-50 border-b border-gray-100">
        <p class="text-xs text-gray-500 uppercase tracking-wider font-semibold">Email</p>
        <p class="text-sm font-medium text-gray-900 truncate">{{ user?.email }}</p>
        <span class="inline-block mt-1 text-xs bg-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full">
          {{ user?.role || 'User' }}
        </span>
      </div>

      <div class="py-1">
        <button 
          @click="handleLogout"
          class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50"
        >
          Sign Out
        </button>
      </div>
    </div>

  </div>
</template>