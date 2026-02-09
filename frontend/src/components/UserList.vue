<script setup>
import { ref, onMounted } from 'vue'
import defaultAvatar from '../assets/avatar.png'

// 1. STATE: The three realities
const users = ref([])          // The data
const isLoading = ref(true)    // Are we waiting?
const error = ref(null)        // Did something break?

// 2. LOGIC: The Fetch Function
async function fetchUsers() {
  try {
    // Reset states
    isLoading.value = true
    error.value = null

    // making the request (using a fake API for now)
    const response = await fetch('/api/users')

    // Check if the backend actually said "OK" (Status 200-299)
    if (!response.ok) {
      throw new Error(`Server Error: ${response.status}`)
    }

    // Parse JSON
    users.value = await response.json()
    
  } catch (err) {
    // If network fails or server errors
    error.value = err.message
  } finally {
    // This runs whether we succeeded or failed
    isLoading.value = false
  }
}

// 3. LIFECYCLE: When should we run this?
// 'onMounted' runs automatically when the component is first put on screen.
onMounted(() => {
  fetchUsers()
})
</script>

<template>
  <div class="p-6 max-w-2xl mx-auto">
    <h2 class="text-2xl font-bold mb-4">User Directory</h2>

    <div v-if="isLoading" class="text-gray-500 animate-pulse">
      Loading users from backend...
    </div>

    <div v-else-if="error" class="text-red-500 bg-red-50 p-4 rounded border border-red-200">
      <p class="font-bold">Error loading data:</p>
      <p>{{ error }}</p>
      <button @click="fetchUsers" class="mt-2 text-sm underline">Try Again</button>
    </div>

    <ul v-else class="space-y-2">
      <li 
        v-for="user in users" 
        :key="user.id"
        class="p-4 bg-white shadow rounded hover:shadow-md transition border border-gray-100"
      >
      
        <div class="flex items-center gap-4">
        
        <img 
          :src="defaultAvatar" 
          class="w-10 h-10 rounded-full object-cover border border-gray-200"
        />
        
        <div>
          <p class="font-medium text-gray-900">{{ user.name }}</p>
          <p class="text-sm text-gray-500">{{ user.email }}</p>
        </div>
      </div>
      </li>
    </ul>

  </div>
</template>