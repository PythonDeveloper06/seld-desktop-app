<script setup>
import { useUserData } from '~/store/authuser';
import { invoke } from '@tauri-apps/api/core';

definePageMeta({
  middleware: 'auth',
  layout: 'custom'
})

const userData = useUserData()

const router = useRouter();
const devices = ref([]);
const setVar = ref('');

onBeforeUnmount(() => {
    clearInterval(setVar.value)
});

setVar.value = setInterval(async() => {
    devices.value = await invoke('get_devices');
}, 1000);

</script>

<template>
    <main>
        <div class="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-2 pt-5">
            <div v-for="key in devices">
                <div class="max-w-sm overflow-hidden shadow-xl w-50 lg:w-100 p-4 border border-black-500 box-device">
                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Device name</label>
                        <div class="text-center">
                            {{ key.device_name }}
                        </div>
                    </div>
                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Status</label>
                        <div class="text-center">
                            {{ key.status }}
                        </div>
                    </div>
                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Admin</label>
                        <div class="text-center">
                            {{ key.admin }}
                        </div>
                    </div>
                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Sync</label>
                        <div class="text-center">
                            {{ key.sync }}
                        </div>
                    </div>
                    <div class="mt-2">
                        <button type="button" @click=" router.push('/keys'), userData.serial_num = key.serial_num " class="items-center w-full p-1 rounded-lg btn-device">Ключи</button>
                    </div>
                </div>            
            </div>
        </div>
    </main>
</template>