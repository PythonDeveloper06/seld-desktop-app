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

const update_name = ref("");

devices.value = await invoke('get_devices');

onBeforeUnmount(() => {
    clearInterval(setVar.value)
});

setVar.value = setInterval(async() => {
    devices.value = await invoke('get_devices');
}, 1500);

async function changState(serial_num, param, value) {
    await invoke("update_device", { serialNum: serial_num, changeParam: param, value: value })
}

</script>

<template>
    <main>
        <div class="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-2 pt-5">
            <div v-for="(device, index) in devices">
                <div class="max-w-sm overflow-hidden shadow-xl w-50 lg:w-100 p-4 border border-black-500 box-device">
                    <div class="grid grid-cols-2 py-0.5" :id="device.id">
                        <div class="text-center cursor-pointer" @click="device.device_name = update_name[index].textContent, changState(device.serial_num, 'device_name', device.device_name)">
                            Device name
                        </div>
                        <div ref="update_name" class="text-center" contenteditable="true">
                            {{ device.device_name }}
                        </div>
                    </div>

                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Status</label>
                        <div class="text-center cursor-pointer" 
                            @click="device.status = (device.status === 'Close') ? 'Open' : 'Close', 
                            changState(device.serial_num, 'status', device.status)"
                        >
                            {{ device.status }}
                        </div>
                    </div>
                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Admin</label>
                        <div class="text-center cursor-pointer"
                            @click="device.admin = (device.admin === 'On') ? 'Off' : 'On', 
                            changState(device.serial_num, 'admin', device.admin)"
                        >
                            {{ device.admin }}
                        </div>
                    </div>
                    <div class="grid grid-cols-2 py-0.5">
                        <label class="text-center">Sync</label>
                        <div class="text-center">
                            {{ device.sync }}
                        </div>
                    </div>
                    <div class="mt-2">
                        <button type="button" @click="router.push('/keys'), userData.serial_num = device.serial_num, userData.id = device.id" class="items-center w-full p-1 rounded-lg btn-device">Ключи</button>
                    </div>
                </div>            
            </div>
        </div>
    </main>
</template>