import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core"

export const useUserData = defineStore('userdata', {
    state: () => ({ login: "", password: "", message: "", serial_num: "" }),
    actions: {
        async getUserToken() {
            this.message = await invoke('get_token', { login: this.login, password: this.password })
            return this.message.split(' ')[0]
        }
    }
})