<template>
	<div class="app">
		<aside :class="`${is_expanded ? 'is-expanded' : ''}`">
			<div class="logo">
				<img :src="logoURL" alt="SELD" /> 
			</div>

			<div class="menu-toggle-wrap">
				<button class="menu-toggle" @click="ToggleMenu">
					<span class="material-icons">keyboard_double_arrow_right</span>
				</button>
			</div>

			<div class="menu">

				<nuxt-link to="/devices" class="button">
					<span class="material-icons">home</span>
					<span class="text">Devices</span>
				</nuxt-link>

				<nuxt-link to="/about" class="button">
					<span class="material-icons">info</span>
					<span class="text">About</span>
				</nuxt-link>

				<nuxt-link to="/team" class="button">
					<span class="material-icons">group</span>
					<span class="text">Team</span>
				</nuxt-link>

				<div @click=" router.push('/'), userData.login = '' " class="button">
					<span class="material-icons">logout</span>
					<span class="text">Logout</span>
				</div>

			</div>
		</aside>
		<slot></slot>
	</div>
</template>

<script setup>
import { ref } from 'vue'
import logoURL from '../assets/media/locked.png'
import { useUserData } from '~/store/authuser';

const userData = useUserData()

const router = useRouter();

const is_expanded = ref(localStorage.getItem("is_expanded") === "true")

const ToggleMenu = () => {
	is_expanded.value = !is_expanded.value
	localStorage.setItem("is_expanded", is_expanded.value)
}
</script>

<style lang="scss" scoped>
aside {
	display: flex;
	flex-direction: column;

	background-color: var(--dark);
	color: var(--light);

	width: calc(2rem + 32px);
	min-height: 100vh;
	padding: 1rem;
	z-index: 99;
	position: fixed;

	transition: 0.2s ease-in-out;

	.flex {
		flex: 1 1 0%;
	}

	.logo {
		margin-bottom: 1rem;
		padding-left: 0.25rem;

		img {
			width: 1.5rem;
		}
	}

	.menu-toggle-wrap {
		display: flex;
		justify-content: flex-end;
		padding-right: 0.25rem;
		margin-bottom: 1rem;

		position: relative;
		top: 0;
		transition: 0.2s ease-in-out;

		.menu-toggle {
			transition: 0.2s ease-in-out;
			.material-icons {
				font-size: 1.5rem;
				color: var(--light);
				transition: 0.2s ease-out;
			}
			
			&:hover {
				.material-icons {
					color: var(--primary);
					transform: translateX(0.5rem);
				}
			}
		}
	}

	h3, .button .text {
		opacity: 0;
		transition: opacity 0.3s ease-in-out;
	}

	h3 {
		color: var(--grey);
		font-size: 0.875rem;
		margin-bottom: 0.5rem;
		text-transform: uppercase;
	}

	.menu {
		margin: 0 -1rem;

		.button {
			display: flex;
			align-items: center;
			text-decoration: none;
			cursor: pointer;

			transition: 0.2s ease-in-out;
			padding: 0.5rem 1.25rem;

			.material-icons {
				font-size: 1.5rem;
				color: var(--light);
				transition: 0.2s ease-in-out;
			}
			.text {
				color: var(--light);
				transition: 0.2s ease-in-out;
			}

			&:hover {
				background-color: var(--dark-alt);

				.material-icons, .text {
					color: var(--primary);
				}
			}

			&.router-link-exact-active {
				background-color: var(--dark-alt);
				border-right: 5px solid var(--primary);

				.material-icons, .text {
					color: var(--primary);
				}
			}
		}
	}

	.footer {
		opacity: 0;
		transition: opacity 0.3s ease-in-out;

		p {
			font-size: 0.875rem;
			color: var(--grey);
		}
	}

	&.is-expanded {
		width: var(--sidebar-width);
		border-radius: 0 30px 30px 0;

		.menu-toggle-wrap {
			top: -3rem;
			justify-content: flex-end;
			
			.menu-toggle {
				transform: rotate(-180deg);
			}
		}

		h3, .button .text {
			opacity: 1;
		}

		.button {
			.material-icons {
				margin-right: 1rem;
			}
		}

		.footer {
			opacity: 0;
		}
	}
}
</style>