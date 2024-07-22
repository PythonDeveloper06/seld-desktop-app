import { useUserData } from "../store/authuser"

export default defineNuxtRouteMiddleware(async(to, from) => {
    const userData = useUserData();

    if (!userData.login) {
        return navigateTo('/')
    }

})