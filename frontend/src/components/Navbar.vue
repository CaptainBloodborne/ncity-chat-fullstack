<script setup>
import { RouterLink } from 'vue-router'
import UserMenu from './UserMenu.vue'
import { useAuth } from '../composables/useAuth';

const { isAdmin } = useAuth()
</script>

<template>
    <nav class="bg-indigo-600 text-white shadow-md">
        <div class="container mx-auto px-4 h-16 flex items-center justify-between">

            <div class="flex items-center gap-8">
                <RouterLink to="/" class="flex items-center gap-2 group">

                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-8 h-8 text-indigo-200 transition-all duration-300 ease-in-out
           group-hover:text-white 
           group-hover:scale-110 
           group-hover:rotate-6">
                        <path d="M12 2l9 4.9V17L12 22l-9-4.9V6.9L12 2z" />
                        <path d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
                    </svg>

                    <span
                        class="text-xl font-bold tracking-tight text-white group-hover:text-indigo-100 transition-colors">
                        N-City Chat Network
                    </span>

                </RouterLink>

                <RouterLink v-if="isAdmin" to="/admin" active-class="text-white font-bold"
                    class="text-indigo-200 hover:text-white transition flex items-center gap-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-4 h-4">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M9 12.75 11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 0 1-1.043 3.296 3.745 3.745 0 0 1-3.296 1.043A3.745 3.745 0 0 1 12 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 0 1-3.296-1.043 3.745 3.745 0 0 1-1.043-3.296A3.745 3.745 0 0 1 3 12c0-1.268.63-2.39 1.593-3.068a3.745 3.745 0 0 1 1.043-3.296 3.746 3.746 0 0 1 3.296-1.043A3.746 3.746 0 0 1 12 3c1.268 0 2.39.63 3.068 1.593a3.746 3.746 0 0 1 3.296 1.043 3.746 3.746 0 0 1 1.043 3.296A3.745 3.745 0 0 1 21 12Z" />
                    </svg>
                    Панель управления
                </RouterLink>
            </div>

            <div class="flex items-center">
                <UserMenu />
            </div>

        </div>
    </nav>
</template>