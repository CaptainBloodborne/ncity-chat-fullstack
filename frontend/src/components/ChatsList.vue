<script setup>
import { useAuth } from '../composables/useAuth';
import { ref, onMounted, computed } from 'vue'
import { fetchApi } from '../utils/http'

const chats = ref([])
const isLoading = ref(true)
const error = ref(null)

const { isAdmin } = useAuth()

async function fetchChats() {
  try {
    isLoading.value = true
    error.value = null

    const response = await fetchApi('/api/chats')

    if (!response.ok) {

      throw new Error(`Server Error: ${response.status}`)

    }

    chats.value = await response.json()
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

const searchQuery = ref('')
const sortKey = ref('name') // 'name' or 'users'
const sortOrder = ref('asc') // 'asc' or 'desc'


const filteredAndSortedChats = computed(() => {
  let result = chats.value.filter(chat => {
    return chat.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      chat.location.toLowerCase().includes(searchQuery.value.toLowerCase())
  })

  result.sort((a, b) => {

    let valA = a[sortKey.value]
    let valB = b[sortKey.value]

    if (typeof valA === 'string') {
      valA = valA.toLowerCase()
      valB = valB.toLowerCase()
    }

    if (valA < valB) return sortOrder.value === 'asc' ? -1 : 1
    if (valA > valB) return sortOrder.value === 'asc' ? 1 : -1
    return 0
  })

  return result
})

async function handleDeleteChat(chatId) {
  if (!confirm('Вы уверены?')) return

  try {
    const response = await fetchApi('/api/admin/chat', {
            method: 'DELETE',
            body: JSON.stringify({
                id: chatId,
            })
        })

    if (!response.ok) throw new Error('Не удалось удалить чат')

    chats.value = chats.value.filter(c => c.id != chatId)

  } catch (error) {
    alert("Error deleting chat: " + error.message)
  }
}

onMounted(() => {
  fetchChats()
})
</script>

<template>
  <div class="p-6 max-w-5xl mx-auto space-y-6">
    
    <h1 class="text-3xl font-bold text-gray-800">Сообщества и чаты Нижнего Новгорода</h1>

    <div v-if="isLoading" class="text-gray-500 animate-pulse py-8 text-center">
      Загрузка списка чатов...
    </div>

    <div v-else-if="error" class="text-red-500 bg-red-50 p-4 rounded border border-red-200 flex justify-between items-center">
      <span>Ошибка: {{ error }}</span>
      <button @click="fetchChats" class="text-sm underline hover:text-red-700">Обновить</button>
    </div>

    <div v-else class="space-y-6">

      <div class="flex flex-col sm:flex-row gap-4">
        <input 
          v-model="searchQuery" 
          placeholder="Поиск чата..." 
          class="border p-2 rounded w-full max-w-sm focus:ring-2 focus:ring-indigo-500 outline-none" 
        />

        <select v-model="sortKey" class="border p-2 rounded bg-white focus:ring-2 focus:ring-indigo-500 outline-none">
          <option value="name">Сортировка по имени</option>
          <option value="users">Сортировка по популярности</option>
        </select>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">

        <div 
          v-for="chat in filteredAndSortedChats" 
          :key="chat.id"
          class="bg-white p-6 rounded-lg shadow hover:shadow-lg transition border border-gray-100 relative group"
        >

          <button 
            v-if="isAdmin"
            @click.stop="handleDeleteChat(chat.id)"
            class="absolute top-3 right-3 p-2 bg-white text-gray-400 hover:text-red-600 hover:bg-red-50 rounded-full transition-colors shadow-sm border border-gray-100 z-10"
            title="Удалить чат"
          >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
              <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
            </svg>
          </button>

          <div class="flex justify-between items-start pr-8">
            <h3 class="font-bold text-lg text-gray-900 leading-tight mb-1">{{ chat.name }}</h3>
          </div>
          
          <div class="mb-3">
             <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800">
              {{ chat.users_count }} пользователи
            </span>
          </div>

          <p class="text-gray-600 text-sm mb-4 line-clamp-3 min-h-10">
            {{ chat.description }}
          </p>

          <div class="text-gray-500 text-xs flex items-center gap-1 mt-auto pt-4 border-t border-gray-50">
            <span>📍</span> {{ chat.location }}
          </div>

        </div>
      </div>
    </div>
  </div>
</template>